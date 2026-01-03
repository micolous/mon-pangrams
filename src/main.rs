#[cfg(feature = "memory-stats")]
mod memory;
mod pronunciation;

#[cfg(feature = "memory-stats")]
use crate::memory::get_memory_stats;
use crate::pronunciation::{Pokémon, PronunciationReader, MON_PHONEMES};
use clap::{Parser, ValueEnum};
use rand::seq::SliceRandom;
use rustc_hash::FxHashSet;
use std::{cmp::Reverse, fs::File, io::BufReader, path::PathBuf};

#[derive(Parser)]
#[command(disable_help_flag = true)]
struct Opts {
    /// Print help
    // Forces using long help.
    #[clap(short, long, action = clap::ArgAction::HelpLong)]
    help: Option<bool>,

    /// Input filename
    #[clap()]
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

#[derive(Default, Clone, ValueEnum)]
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

#[derive(Debug)]
struct Solution<'a> {
    mons: Vec<&'a Pokémon>,
    coverage: u64,
}

impl Solution<'_> {
    /// Cache key for visited sets of Pokémon.
    ///
    /// Rather than keep a list of exactly which Pokémon we visited (and keep it sorted to deal with
    /// equivalent paths), this just uses the `coverage` and `mons.len()`.
    ///
    /// This introduces collisions to reduce our search space: what matters is the coverage we
    /// acquired for the number of Pokémon in the list.
    pub const fn cache_key(&self) -> u64 {
        self.coverage | ((self.mons.len() as u64) << MON_PHONEMES.len())
    }
}

fn solve<'a>(
    missing_bits: u64,
    mons_by_phone: &[Vec<&'a Pokémon>; MON_PHONEMES.len()],
    existing_solution: &Solution<'a>,
    max_coverage: u64,
    cache: &mut FxHashSet<u64>,
) -> Vec<Solution<'a>> {
    let mut min_names = usize::MAX;
    let mut shortests = Vec::new();

    // Find the shortest list(s)
    for k in 0..MON_PHONEMES.len() {
        if 1 << k & missing_bits != 0 {
            // We are missing this bit
            let mons = &mons_by_phone[k];
            let l = mons.len();
            if l < min_names {
                shortests.clear();
                min_names = l;
                assert!(min_names > 1);
            }

            if l <= min_names {
                shortests.push(mons);
            }
        }
    }

    let mut o: Vec<Solution<'_>> = Vec::new();
    for mons in shortests {
        assert_eq!(mons.len(), min_names);

        for mon in mons {
            let coverage = existing_solution.coverage | mon.phonemes_mask;
            assert_ne!(
                coverage, existing_solution.coverage,
                "for {mon:#x?}, min_names {min_names}"
            );

            let mut mons = existing_solution.mons.clone();
            mons.push(mon);
            let solution = Solution { mons, coverage };

            if !cache.insert(solution.cache_key()) {
                // This isn't a new path.
                continue;
            }

            if coverage == max_coverage {
                // We have a solution, return this and nothing more
                return vec![solution];
            }

            // More work to do.
            let idx =
                o.partition_point(|x| solution.coverage.count_ones() <= x.coverage.count_ones());
            o.insert(idx, solution);
        }
    }

    o
}

fn main() {
    let opts = Opts::parse();
    let mut rng = rand::rng();

    let f = BufReader::new(File::open(opts.input).unwrap());
    let reader = PronunciationReader::new(f);
    let mut mons = reader.into_vec().unwrap();
    assert!(!mons.is_empty());

    let pokemon_count = mons.len();
    println!("Read {pokemon_count} Pokémon");

    let mut max_coverage = 0;

    if opts.no_prune {
        max_coverage = mons.iter().fold(0, |acc, e| acc | e.phonemes_mask);
    } else {
        // Sort by number of one-bits in the mask then the mask itself, so that higher-coverage entries
        // appear earlier in the list (and we get a stable sort).
        mons.sort_by_key(|e| Reverse((e.phonemes_mask.count_ones(), e.phonemes_mask)));

        // Working from the end of the list (= less ones), remove entries that are subsets of an earlier
        // entry.
        let mut i = mons.len() - 1;
        while i > 0 {
            let mask = mons[i].phonemes_mask;
            for o in &mons[..i] {
                if o.phonemes_mask | mask == o.phonemes_mask {
                    // This mon is a subset of another mon
                    mons.remove(i);
                    break;
                }
            }
            i -= 1;
            max_coverage |= mask;
        }
    }

    // Re-sort the list of Pokémon to apply the selection preference
    match opts.prefer {
        SelectionPreference::MostUnique => {
            if opts.no_prune {
                // We have to sort it here
                mons.sort_by_key(|e| Reverse(e.phonemes_mask.count_ones()));
            }
        }
        SelectionPreference::LeastUnique => {
            mons.sort_by_key(|e| e.phonemes_mask.count_ones());
        }
        SelectionPreference::MostPhonemes => {
            mons.sort_by_key(|e| Reverse(e.ipa.len()));
        }
        SelectionPreference::LeastPhonemes => {
            mons.sort_by_key(|e| e.ipa.len());
        }
        SelectionPreference::LongestName => {
            mons.sort_by_key(|e| Reverse(e.name.len()));
        }
        SelectionPreference::ShortestName => {
            mons.sort_by_key(|e| e.name.len());
        }
        SelectionPreference::Alphabetical => {
            mons.sort_by_key(|e| e.name.clone());
        }
        SelectionPreference::ReverseAlphabetical => {
            mons.sort_by_key(|e| Reverse(e.name.clone()));
        }
        SelectionPreference::Random => {
            mons.shuffle(&mut rng);
        }
    }

    let distinct_pokemon_count = mons.len();
    if !opts.no_prune {
        print!("There are {distinct_pokemon_count} Pokémon that do not use a subset of another's phonemes");
        if opts.quiet {
            println!();
        } else {
            println!(":");
            for (i, mon) in mons.iter().enumerate() {
                println!(
                    "  [{i:03}] = {:20} mask: {:#12x}, bits: {:2}",
                    mon.name,
                    mon.phonemes_mask,
                    mon.phonemes_mask.count_ones(),
                );
            }
        }
    }
    // phone bit -> Vec<&Pokemon> that has it
    let mut mons_by_phone: [Vec<&Pokémon>; MON_PHONEMES.len()] =
        [const { vec![] }; MON_PHONEMES.len()];
    // let mut mons_by_phone: BTreeMap<u8, Vec<&Pokémon>> = BTreeMap::new();
    for mon in mons.iter() {
        for b in 0..mons_by_phone.len() {
            if mon.phonemes_mask & (1 << b) != 0 {
                mons_by_phone[b].push(mon);
                // if let Some(e) = mons_by_phone.get_mut(&b) {
                //     e.push(mon);
                // } else {
                //     mons_by_phone.insert(b, vec![mon]);
                // }
            }
        }
    }

    let phoneme_count = max_coverage.count_ones();
    if !opts.quiet {
        println!();
    }
    print!("{phoneme_count} phonemes represented");

    // frequency -> phone ID
    // let mut frequency: Vec<(u16, u8)> = Vec::with_capacity(MON_PHONEMES.len());
    if opts.quiet {
        println!();
    } else {
        println!(":");
        for (k, v) in mons_by_phone.iter().enumerate() {
            if v.is_empty() {
                continue;
            }
            let phone = MON_PHONEMES[k as usize];
            println!("  {phone}: {:3} Pokémon", v.len());
            // frequency.push((count, k));
        }
    }

    // Sort the frequency table by lowest -> highest frequency
    // frequency.sort();
    // println!("frequency -> phone_id: {frequency:?}");

    #[cfg(feature = "memory-stats")]
    {
        let (now, peak) = get_memory_stats();
        println!();
        println!("Memory usage before running solver: {now} now, {peak} peak");
    }

    if !opts.quiet {
        println!();
    }
    println!("Finding a solution...");

    // Check if there are any unique phonemes first, and include them in the initial solution
    let mut initial_solution = Solution {
        mons: Vec::new(),
        coverage: 0,
    };

    for mons in mons_by_phone.iter_mut() {
        if mons.len() != 1 {
            continue;
        }

        let mon = mons.remove(0);

        let coverage = initial_solution.coverage | mon.phonemes_mask;

        // We shouldn't fully overlap here
        assert_ne!(coverage, initial_solution.coverage);

        initial_solution.coverage = coverage;
        initial_solution.mons.push(mon);
    }

    // We have some initial solution
    if initial_solution.coverage != 0 {
        assert!(!initial_solution.mons.is_empty());
        // Remove phonemes that are represented in our unique-phoneme Pokemon.
        for k in 0..MON_PHONEMES.len() {
            if (1 << k) & initial_solution.coverage != 0 {
                mons_by_phone[k] = Vec::with_capacity(0);
            }
        }
    } else {
        assert!(initial_solution.mons.is_empty());
    }

    // Prevent further mutation
    let mons_by_phone = mons_by_phone;

    // Start finding solutions
    let mut missing_bits = max_coverage ^ initial_solution.coverage;
    let mut best_solution = String::new();

    if missing_bits == 0 {
        let best_length = initial_solution.mons.len();
        let mut first = true;
        for mon in initial_solution.mons {
            if first {
                first = false;
            } else {
                best_solution.push_str(", ");
            }
            best_solution.push_str(&mon.name);
        }

        println!("Initial solution is complete ({best_length} Pokémon): {best_solution}");
        return;
    }

    let mut stack: Vec<Solution<'_>> = vec![initial_solution];
    let mut cache = FxHashSet::default();
    // let mut best_bits = 0;
    let mut best_length = mons_by_phone.len();
    let mut solution_count = 0;
    let mut peak_stack_len = stack.len();
    let mut peak_candidate_len = 0;
    let mut solve_calls = 0usize;

    while let Some(step) = stack.pop() {
        if step.mons.len() + 1 >= best_length {
            // There's no way we could beat this solution.
            continue;
        }

        missing_bits = max_coverage ^ step.coverage;
        assert_ne!(0, missing_bits);

        let list_of_solutions = solve(
            missing_bits,
            &mons_by_phone,
            &step,
            max_coverage,
            &mut cache,
        );
        solve_calls += 1;
        // println!("solver gave {} solutions", list_of_solutions.len());
        peak_candidate_len = peak_candidate_len.max(list_of_solutions.len());

        for solution in list_of_solutions {
            solution_count += 1;

            if solution.coverage == max_coverage {
                best_length = solution.mons.len();
                let mut first = true;
                best_solution.clear();
                for mon in solution.mons {
                    if first {
                        first = false;
                    } else {
                        best_solution.push_str(", ");
                    }
                    best_solution.push_str(&mon.name);
                }

                println!(" Solution #{solution_count} ({best_length} Pokémon): {best_solution}");

                // Don't consider more solutions at this length (issue with gen3)
                break;
            }

            if solution.mons.len() <= best_length {
                // We don't have a full solution, but it's a potentially-better next step.
                // Put the most-complete solutions at the end, so we try that next.
                let idx = stack
                    .partition_point(|s| s.coverage.count_ones() <= solution.coverage.count_ones());
                stack.insert(idx, solution);
            }
        }

        peak_stack_len = peak_stack_len.max(stack.len());
    }

    println!();
    println!("Done: {solve_calls} solver calls, {peak_candidate_len} largest solve result, {solution_count} processed candidates, {peak_stack_len} peak queue length, {} cache entries", cache.len());

    if opts.summary {
        println!();
        println!("| **Generation** | {pokemon_count} | {distinct_pokemon_count} | {phoneme_count} | **{best_length} Pokémon**: {best_solution} |");
    }

    #[cfg(feature = "memory-stats")]
    {
        let (now, peak) = get_memory_stats();
        println!();
        println!("Memory usage after running solver: {now} now, {peak} peak");
    }
}
