#[cfg(feature = "memory-stats")]
use crate::memory::get_memory_stats;
use crate::pronunciation::{phoneme_index, BitSet, Pokémon, PronunciationReader, MON_PHONEMES};
use clap::Parser;
use eyre::Result;
use itertools::Itertools;
use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, VecDeque},
    fs::File,
    io::BufReader,
    path::PathBuf,
};

#[cfg(feature = "memory-stats")]
mod memory;
mod pronunciation;

#[derive(Parser)]
struct Opts {
    /// Input filename
    #[clap()]
    input: PathBuf,
}

#[derive(Debug, Default)]
struct Solution<'a> {
    mons: Vec<&'a Pokémon>,
    coverage: BitSet,
    cache_key: Vec<u16>,
}

fn solve<'a>(
    mons_by_phone: &BTreeMap<char, &Vec<&'a Pokémon>>,
    existing_solution: &Solution<'a>,
    max_coverage: BitSet,
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

            let mut cache_key = existing_solution.cache_key.clone();
            let i = cache_key.partition_point(|&x| x <= mon.id);
            cache_key.insert(i, mon.id);

            let solution = Solution {
                mons,
                coverage,
                cache_key,
            };

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
    let mut mons = PronunciationReader::new(f).collect::<Result<Vec<_>>>()?;
    println!("Read {} Pokémon", mons.len());

    // Sort by number of bits in the mask then the mask itself, so that higher-coverage entries
    // appear earlier in the list (and we get a stable sort).
    mons.sort_by_key(|e| Reverse((e.phonemes_mask.len(), e.phonemes_mask)));

    // Working from the end of the list (= less bits), remove entries that are subsets of an earlier
    // entry.
    let mut max_coverage = BitSet::default();
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

    println!(
        "There are {} Pokémon that do not use a subset of another's phonemes:",
        mons.len()
    );
    for (i, mon) in mons.iter().enumerate() {
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

    // phone bit -> Vec<&Pokemon> that has it
    let mut mons_by_phone: BTreeMap<char, Vec<&Pokémon>> = MON_PHONEMES
        .iter()
        .map(|&phoneme| {
            (
                phoneme,
                mons.iter()
                    .filter(|mon| mon.ipa.contains(phoneme))
                    .collect(),
            )
        })
        .collect();

    println!();
    println!("{} phonemes represented:", mons_by_phone.len());
    // (frequency -> phone), sorted from lowest to highest frequency
    let frequency: Vec<(usize, char)> = mons_by_phone
        .iter()
        .map(|(&phone, mons)| (mons.len(), phone))
        .sorted()
        .collect();
    // println!("frequency -> phone: {frequency:?}");

    #[cfg(feature = "memory-stats")]
    {
        let (now, peak) = get_memory_stats();
        println!();
        println!("Memory usage before running solver: {now} now, {peak} peak");
    }

    println!();
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

        // We don't need to update the cache key for these initial solutions. The solver wouldn't
        // consider these mon as their other phonemes will be pruned later, and they also won't
        // improve coverage.
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
    let mut queue: VecDeque<Solution<'_>> = VecDeque::from([initial_solution]);
    let mut cache = BTreeSet::new();
    // let mut best_bits = 0;
    let mut best_length = mons_by_phone.len();
    let mut solution_count = 0;
    let mut peak_queue_len = queue.len();
    let mut peak_candidate_len = 0;

    while let Some(step) = queue.pop_front() {
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

        let mut list_of_solutions = solve(&lookup, &step, max_coverage);
        // println!("solver gave {} solutions", list_of_solutions.len());
        peak_candidate_len = peak_candidate_len.max(list_of_solutions.len());
        list_of_solutions.sort_by_key(|s| Reverse(s.coverage.len()));

        for solution in list_of_solutions {
            solution_count += 1;

            if cache.insert(solution.cache_key.clone()) {
                if solution.coverage == max_coverage {
                    best_length = solution.mons.len();
                    print!(" Solution #{solution_count} ({best_length} Pokémon): ");
                    let mut first = true;
                    for mon in solution.mons {
                        if first {
                            first = false;
                        } else {
                            print!(", ");
                        }
                        print!("{}", mon.name);
                    }
                    println!();

                    // Don't consider more solutions at this length (issue with gen3)
                    break;
                }

                if solution.mons.len() <= best_length {
                    let idx = queue.partition_point(|s| s.coverage.len() > solution.coverage.len());
                    // println!("potential solution: [{idx}] {solution:?}");
                    // best_bits = best_bits.max(solution.coverage.count_ones());
                    queue.insert(idx, solution);
                }
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

    #[cfg(feature = "memory-stats")]
    {
        let (now, peak) = get_memory_stats();
        println!("Memory usage after running solver: {now} now, {peak} peak");
    }

    Ok(())
}
