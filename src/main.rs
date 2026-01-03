#![allow(unstable_name_collisions)]

#[cfg(feature = "memory-stats")]
mod memory;
mod pronunciation;
pub mod set;

use crate::{
    pronunciation::{MON_PHONEMES, Pokémon, PronunciationReader, phoneme_index},
    set::BitSet,
};
use clap::{Parser, ValueEnum};
use eyre::{OptionExt, Result};
use itertools::Itertools;
use rand::seq::SliceRandom;
use std::{
    cmp::Reverse,
    collections::{BTreeMap, HashSet},
    fmt::Display,
    fs::File,
    io::BufReader,
    ops::BitOr,
    path::PathBuf,
};

#[derive(Parser)]
#[command(disable_help_flag = true)]
struct Opts {
    /// Print help
    // Forces using long help.
    #[clap(short, long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,

    /// Input filename
    input: PathBuf,

    /// Emit less debugging output
    #[clap(long)]
    quiet: bool,

    /// Emit a solution summary for the README
    #[clap(long)]
    summary: bool,

    /// Which Pokémon to preference when more than one would provide the same missing phonemes.
    #[clap(short, long, default_value = "most-unique")]
    prefer: SelectionPreference,

    /// Don't prune Pokémon that contain a subset of another's phonemes. This will significantly
    /// reduce performance and increase memory usage.
    #[clap(long)]
    no_prune: bool,
}

#[derive(Default, Copy, Clone, ValueEnum)]
enum SelectionPreference {
    /// Most unique phonemes (default, greedy selection that generally finds a solution quickly, but
    /// may be a tongue twister)
    #[default]
    MostUnique,

    /// Least unique phonemes (prefers simpler names)
    LeastUnique,

    /// Most phonemes (whichever takes the longest to say, even if it contains repeated phonemes)
    MostPhonemes,

    /// Least phonemes (whichever is shorter to say)
    LeastPhonemes,

    /// Longest name
    LongestName,

    /// Shortest name
    ShortestName,

    /// Prefers names that are earlier in the alphabet
    Alphabetical,

    /// Prefers names that are later in the alphabet
    ReverseAlphabetical,

    /// Random shuffle (diverse results)
    Random,
}

impl SelectionPreference {
    fn sort_mons(self, mons: &mut [Pokémon], rng: &mut impl rand::Rng) {
        match self {
            SelectionPreference::MostUnique => mons.sort_by_key(|e| Reverse(e.phoneme_set.len())),
            SelectionPreference::LeastUnique => mons.sort_by_key(|e| e.phoneme_set.len()),
            SelectionPreference::MostPhonemes => mons.sort_by_key(|e| Reverse(e.ipa().len())),
            SelectionPreference::LeastPhonemes => mons.sort_by_key(|e| e.ipa().len()),
            SelectionPreference::LongestName => mons.sort_by_key(|e| Reverse(e.name().len())),
            SelectionPreference::ShortestName => mons.sort_by_key(|e| e.name().len()),
            SelectionPreference::Alphabetical => mons.sort_by(|a, b| a.name().cmp(&b.name())),
            SelectionPreference::ReverseAlphabetical => {
                mons.sort_by(|a, b| b.name().cmp(&a.name()))
            }
            SelectionPreference::Random => mons.shuffle(rng),
        }
    }
}

/// Cache key for visited sets of Pokémon.
///
/// Rather than keep a list of exactly which Pokémon we visited (and keep it sorted to deal with
/// equivalent paths), this just uses the `coverage` and `mons.len()`.
///
/// This introduces collisions to reduce our search space: what matters is the coverage we
/// acquired for the number of Pokémon in the list.
fn cache_key(coverage: BitSet, iteration: usize) -> u64 {
    coverage.into_inner() | ((iteration as u64) << MON_PHONEMES.len())
}

fn solve(
    already_covered: BitSet,
    mon_list: Option<MonList>,
    call_depth: usize,
    phone_sets: &[(usize, &[Pokémon])],
    solution_state: &mut SolutionState,
) {
    solution_state.solve_calls += 1;
    // Find the next least frequent phoneme that is not already covered
    let Some((iterating_index, &(_, mons_to_iterate))) = phone_sets
        .iter()
        .enumerate()
        .find(|&(_, &(phone_index, _))| !already_covered.contains(phone_index))
    else {
        panic!("found no phoneme list to recurse down");
    };

    for mon in mons_to_iterate {
        let new_coverage = already_covered | mon.phoneme_set;
        let new_mon_list = MonList {
            prev: mon_list.as_ref(),
            pkmn: mon,
        };
        if new_coverage == solution_state.solve_target {
            solution_state.best_solution = new_mon_list.to_string();
            println!(
                "Found solution with {call_depth} Pokémon: {list}",
                list = solution_state.best_solution
            );
            assert!(solution_state.best_length > SolutionLength::Solved(call_depth));
            solution_state.best_length = SolutionLength::Solved(call_depth);
            return;
        }
        // We should only plan to make further recursive calls if more iterations could improve on
        // the best existing solution.
        // TODO: may want to continue finding alternatives, and option for showing *all* branching
        //  paths to solutions
        if SolutionLength::Solved(call_depth + 1) < solution_state.best_length
            && solution_state
                .cache
                .insert(cache_key(new_coverage, call_depth))
        {
            solve(
                new_coverage,
                Some(new_mon_list),
                call_depth + 1,
                &phone_sets[iterating_index + 1..],
                solution_state,
            );
        }
    }
}

/// Linked list back up the callstack of the pokemon visited so far
struct MonList<'a> {
    prev: Option<&'a MonList<'a>>,
    pkmn: &'a Pokémon,
}

impl Display for MonList<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut current = self;
        let mut names = vec![current.pkmn.name()];
        while let Some(next) = current.prev {
            current = next;
            names.push(current.pkmn.name());
        }
        names.sort();
        names
            .into_iter()
            .intersperse(", ")
            .try_for_each(|s| write!(f, "{s}"))
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum SolutionLength {
    Solved(usize),

    #[default]
    /// Unsolved is ordered after every solution with a length
    Unsolved,
}

#[derive(Debug, Default)]
struct SolutionState {
    solve_target: BitSet,
    cache: HashSet<u64>,
    best_length: SolutionLength,
    solve_calls: usize,
    best_solution: String,
}

fn main() -> Result<()> {
    let opts = Opts::parse();
    let mut rng = rand::rng();

    let f = BufReader::new(File::open(opts.input)?);
    // Sort by number of bits in the mask then the mask itself, so that higher-coverage entries
    // appear earlier in the list (and we get a stable sort).
    let mut mons: Vec<Pokémon> = PronunciationReader::new(f).collect::<Result<_>>()?;
    println!("Read {count} Pokémon", count = mons.len());

    // max_coverage is the set of every covered phoneme
    let max_coverage = mons
        .iter()
        .map(|mon| mon.phoneme_set)
        .reduce(BitOr::bitor)
        .ok_or_eyre("no pokemon loaded")?;

    opts.prefer.sort_mons(&mut mons, &mut rng);
    let mons = mons;

    if !opts.summary {
        println!("There are {count} total Pokémon:", count = mons.len());
        for (i, mon) in mons.iter().enumerate() {
            println!(
                "  [{i:03}] = {name:20} phonemes: {phonemes}",
                name = mon.name(),
                phonemes = mon
                    .phoneme_set
                    .iter()
                    .map(|n| MON_PHONEMES[n])
                    .collect::<String>(),
            );
        }
    }

    // phone -> Vec<Pokemon> that has it
    let mons_by_phone: BTreeMap<char, Vec<Pokémon>> = MON_PHONEMES
        .iter()
        .map(|&phoneme| {
            (
                phoneme,
                mons.iter()
                    .filter(|mon| mon.ipa().contains(phoneme))
                    .cloned()
                    .collect(),
            )
        })
        .collect();

    // Vec of (phoneme index, slice of corresponding mons) smallest to largest subgroup
    let phone_sets: Vec<(usize, &[Pokémon])> = mons_by_phone
        .iter()
        .map(|(phone, mons)| {
            (
                phoneme_index(*phone).expect("failed phoneme lookup"),
                mons.as_slice(),
            )
        })
        .sorted_by_key(|(_, mons)| mons.len())
        .collect();

    if !opts.quiet {
        println!();
        println!("{count} phonemes represented:", count = mons_by_phone.len());
        for (phone_index, mons) in phone_sets.iter() {
            if mons.is_empty() {
                continue;
            }
            let phone = MON_PHONEMES[*phone_index];
            println!("  {phone}: {:3} Pokémon", mons.len());
        }
    }

    #[cfg(feature = "memory-stats")]
    {
        let (now, peak) = memory::get_memory_stats();
        println!();
        println!("Memory usage before running solver: {now} now, {peak} peak");
    }

    if !opts.quiet {
        println!();
    }
    println!("Finding a solution...");

    // Start finding solutions
    let mut solution_state = SolutionState {
        solve_target: max_coverage,
        ..Default::default()
    };
    solve(
        BitSet::default(),
        None,
        1,
        phone_sets.as_slice(),
        &mut solution_state,
    );

    println!();
    println!(
        "Done, {calls} calls to solve, {cache_len} cache entries",
        calls = solution_state.solve_calls,
        cache_len = solution_state.cache.len(),
    );

    println!(
        "{pokemon_count} total Pokémon; {phoneme_count} total phonemes; {solution}",
        pokemon_count = mons.len(),
        phoneme_count = mons_by_phone.len(),
        solution = match solution_state.best_length {
            SolutionLength::Solved(best) => format!("solved in {best}"),
            SolutionLength::Unsolved => "unsolved??".to_owned(),
        }
    );

    if opts.summary {
        println!();
        println!(
            "| **Generation** | {pokemon_count} | {phoneme_count} | {solution} |",
            pokemon_count = mons.len(),
            phoneme_count = mons_by_phone.len(),
            solution = match solution_state.best_length {
                SolutionLength::Solved(best) =>
                    format!("**{best}** Pokémon: {}", solution_state.best_solution),
                SolutionLength::Unsolved => "unsolved??".to_owned(),
            }
        );
    }

    #[cfg(feature = "memory-stats")]
    {
        let (now, peak) = memory::get_memory_stats();
        println!();
        println!("Memory usage after running solver: {now} now, {peak} peak");
    }

    Ok(())
}
