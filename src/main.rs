#[cfg(feature = "memory-stats")]
use crate::memory::get_memory_stats;
use crate::pronunciation::{phoneme_index, Pokémon, PronunciationReader, MON_PHONEMES};
use crate::set::BitSet;
use clap::Parser;
use eyre::{OptionExt, Result};
use itertools::Itertools;
use std::cmp::{Ordering, Reverse};
use std::collections::{BTreeMap, BinaryHeap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::ops::BitOr;
use std::path::PathBuf;

#[cfg(feature = "memory-stats")]
mod memory;
mod pronunciation;
mod set;

#[derive(Parser)]
struct Opts {
    /// Input filename
    input: PathBuf,

    /// Emit less debugging output, and a solution summary for the README
    #[clap(long)]
    summary: bool,
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Solution<'a> {
    mons: Vec<&'a Pokémon>,
    coverage: BitSet,
}

impl<'a> PartialOrd for Solution<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Solution<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ordering_key().cmp(&other.ordering_key())
    }
}

impl Solution<'_> {
    /// Cache key for visited sets of Pokémon.
    ///
    /// Rather than keep a list of exactly which Pokémon we visited (and keep it sorted to deal with
    /// equivalent paths), this just uses the `coverage` and `mons.len()`.
    ///
    /// This introduces collisions to reduce our search space: what matters is the coverage we
    /// acquired for the number of Pokémon in the list.
    pub fn cache_key(&self) -> u64 {
        self.coverage.into_inner() | ((self.mons.len() as u64) << MON_PHONEMES.len())
    }

    pub fn ordering_key(&self) -> impl Ord + use<'_> {
        (self.coverage.len(), self.coverage, &self.mons)
    }
}

fn solve<'a>(
    mons_by_phone: &BTreeMap<char, &Vec<&'a Pokémon>>,
    existing_solution: &Solution<'a>,
    max_coverage: BitSet,
    cache: &mut HashSet<u64>,
) -> Vec<Solution<'a>> {
    let mut min_names = usize::MAX;

    // Find the shortest list(s)
    for mons in mons_by_phone.values() {
        min_names = min_names.min(mons.len());
        assert!(min_names > 1);
    }

    let mut o = Vec::new();
    for mons in mons_by_phone.values() {
        if mons.len() != min_names {
            // Only consider the shortest lists
            continue;
        }

        for mon in *mons {
            let coverage = existing_solution.coverage | mon.phonemes_mask;
            if coverage == existing_solution.coverage {
                // unchanged coverage, skip it
                continue;
            }

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
            o.push(solution);
        }
    }

    o
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    let f = BufReader::new(File::open(opts.input)?);
    // Sort by number of bits in the mask then the mask itself, so that higher-coverage entries
    // appear earlier in the list (and we get a stable sort).
    let original_mons: Vec<Pokémon> = PronunciationReader::new(f)
        .sorted_by_key(|e| {
            let mask = e.as_ref().ok()?.phonemes_mask;
            Some(Reverse((mask.len(), mask)))
        })
        .collect::<Result<_>>()?;
    println!("Read {count} Pokémon", count = original_mons.len());

    // max_coverage is the set of every covered phoneme
    let max_coverage = original_mons
        .iter()
        .map(|mon| mon.phonemes_mask)
        .reduce(BitOr::bitor)
        .ok_or_eyre("no pokemon loaded")?;

    let non_redundant_mons = {
        let mut res: Vec<Pokémon> = vec![];
        for mon in &original_mons {
            if res
                .iter()
                .any(|preceding_mon| mon.phonemes_mask.is_subset_of(preceding_mon.phonemes_mask))
            {
                continue; // exclude all pokemon that are covered by another's pronunciation
            }
            res.push(mon.clone());
        }
        res
    };

    println!(
        "There are {count} Pokémon that do not use a subset of another's phonemes:",
        count = non_redundant_mons.len()
    );
    for (i, mon) in non_redundant_mons.iter().enumerate() {
        println!(
            "  [{i:03}] = {name:20} phonemes: {phonemes}",
            name = mon.name,
            phonemes = mon
                .phonemes_mask
                .iter()
                .map(|n| MON_PHONEMES[n])
                .collect::<String>(),
        );
    }

    // phone -> Vec<&Pokemon> that has it
    let mut mons_by_phone: BTreeMap<char, Vec<&Pokémon>> = MON_PHONEMES
        .iter()
        .map(|&phoneme| {
            (
                phoneme,
                non_redundant_mons
                    .iter()
                    .filter(|mon| mon.ipa.contains(phoneme))
                    .collect(),
            )
        })
        .collect();

    if !opts.summary {
        println!();
        println!("{count} phonemes represented:", count = mons_by_phone.len());
    }
    // (frequency -> phone), sorted from lowest to highest frequency
    let frequency: Vec<(usize, char)> = mons_by_phone
        .iter()
        .map(|(&phone, mons)| {
            if !opts.summary {
                println!("  {phone}: {count:3} Pokémon", count = mons.len());
            }
            (mons.len(), phone)
        })
        .sorted()
        .collect();

    #[cfg(feature = "memory-stats")]
    {
        let (now, peak) = get_memory_stats();
        println!();
        println!("Memory usage before running solver: {now} now, {peak} peak");
    }

    if !opts.summary {
        println!();
    }
    println!("Finding a solution...");

    // Check if there are any unique phonemes first, and include them in the initial solution
    let mut initial_solution = Solution::default();

    for (count, phone) in frequency {
        if count > 1 {
            break;
        }

        let mut mons = mons_by_phone.remove(&phone).unwrap();
        assert_eq!(mons.len(), 1);
        let mon = mons.remove(0);

        if mon.phonemes_mask.is_subset_of(initial_solution.coverage) {
            continue;
        }
        initial_solution.coverage |= mon.phonemes_mask;
        initial_solution.mons.push(mon);
    }

    // We have some initial solution
    if !initial_solution.coverage.is_empty() {
        // Keep phonemes that are not represented by our unique-phoneme Pokemon.
        mons_by_phone.retain(|&phone, _| {
            !initial_solution
                .coverage
                .contains(phoneme_index(phone).unwrap())
        });
    }

    // Prevent further mutation
    let mons_by_phone = mons_by_phone;

    // Start finding solutions
    let mut queue: BinaryHeap<Solution> = BinaryHeap::from_iter([initial_solution]);
    let mut cache = HashSet::new();
    // let mut best_bits = 0;
    let mut best_length = mons_by_phone.len();
    let mut solution_count = 0;
    let mut peak_queue_len = queue.len();
    let mut peak_candidate_len = 0;
    let mut best_solution = String::new();

    while let Some(step) = queue.pop() {
        if step.mons.len() + 1 >= best_length {
            // There's no way we could beat this solution.
            continue;
        }

        // Include only the subset of mons_by_phone that we haven't already covered
        let lookup: BTreeMap<char, &Vec<&Pokémon>> = mons_by_phone
            .iter()
            .filter_map(|(&k, v)| {
                if !step.coverage.contains(phoneme_index(k).unwrap()) {
                    Some((k, v))
                } else {
                    None
                }
            })
            .collect();

        let mut list_of_solutions = solve(&lookup, &step, max_coverage, &mut cache);
        // println!("solver gave {} solutions", list_of_solutions.len());
        peak_candidate_len = peak_candidate_len.max(list_of_solutions.len());
        list_of_solutions.sort_by_key(|s| Reverse(s.coverage.len()));

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
                queue.push(solution);
            }
        }

        // println!("{} queued solutions, {best_length} is best", queue.len());
        peak_queue_len = peak_queue_len.max(queue.len());
    }

    println!();
    println!(
        "Done, tried {solution_count} candidates, {peak_queue_len} peak queue length, \
        {peak_candidate_len} peak solver length, {cache_len} cache entries",
        cache_len = cache.len()
    );

    if opts.summary {
        println!(
            "| **Generation** | {pokemon_count} | {distinct_pokemon_count} | {phoneme_count} | \
            **{best_length} Pokémon**: {best_solution} |",
            pokemon_count = original_mons.len(),
            distinct_pokemon_count = non_redundant_mons.len(),
            phoneme_count = mons_by_phone.len(),
        );
    }

    #[cfg(feature = "memory-stats")]
    {
        let (now, peak) = get_memory_stats();
        println!("Memory usage after running solver: {now} now, {peak} peak");
    }

    Ok(())
}
