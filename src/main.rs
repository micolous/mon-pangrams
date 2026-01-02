#[cfg(feature = "memory-stats")]
mod memory;
mod pronunciation;

#[cfg(feature = "memory-stats")]
use crate::memory::get_memory_stats;
use crate::pronunciation::{Pokémon, PronunciationReader, MON_PHONEMES};
use clap::Parser;
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fs::File,
    io::BufReader,
    path::PathBuf,
};

#[derive(Parser)]
struct Opts {
    /// Input filename
    #[clap()]
    input: PathBuf,
}

#[derive(Debug)]
struct Solution<'a> {
    mons: Vec<&'a Pokémon>,
    coverage: u64,
    cache_key: Vec<u16>,
}

fn solve<'a>(
    mons_by_phone: &BTreeMap<u8, &Vec<&'a Pokémon>>,
    existing_solution: &Solution<'a>,
    max_coverage: u64,
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

fn main() {
    let opts = Opts::parse();

    let f = BufReader::new(File::open(opts.input).unwrap());
    let reader = PronunciationReader::new(f);
    let mut mons = reader.into_vec().unwrap();
    println!("Read {} Pokémon", mons.len());

    // Sort by number of bits in the mask then the mask itself, so that higher-coverage entries
    // appear earlier in the list (and we get a stable sort).
    mons.sort_by_key(|e| {
        e.phonemes_mask | ((e.phonemes_mask.count_ones() as u64) << MON_PHONEMES.len())
    });
    mons.reverse();

    // Working from the end of the list (= less bits), remove entries that are subsets of an earlier
    // entry.
    let mut max_coverage = 0;
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
            "  [{i:03}] = {:20} mask: {:#12x}, bits: {:2}",
            mon.name,
            mon.phonemes_mask,
            mon.phonemes_mask.count_ones(),
        );
    }

    // phone bit -> Vec<&Pokemon> that has it
    let mut mons_by_phone: BTreeMap<u8, Vec<&Pokémon>> = BTreeMap::new();
    for mon in mons.iter() {
        for b in 0..(MON_PHONEMES.len() as u8) {
            if mon.phonemes_mask & (1 << b) != 0 {
                if let Some(e) = mons_by_phone.get_mut(&b) {
                    e.push(mon);
                } else {
                    mons_by_phone.insert(b, vec![mon]);
                }
            }
        }
    }

    println!();
    println!("{} phonemes represented:", mons_by_phone.len());
    // frequency -> phone ID
    let mut frequency: Vec<(u16, u8)> = Vec::with_capacity(MON_PHONEMES.len());
    for (&k, v) in &mons_by_phone {
        let phone = MON_PHONEMES[k as usize];
        let count = v.len() as u16;
        println!("  {phone}: {count:3} Pokémon");
        frequency.push((count, k));
    }

    // Sort the frequency table by lowest -> highest frequency
    frequency.sort();
    // println!("frequency -> phone_id: {frequency:?}");

    #[cfg(feature = "memory-stats")]
    {
        let (now, peak) = get_memory_stats();
        println!();
        println!("Memory usage before running solver: {now} now, {peak} peak");
    }

    println!();
    println!("Finding a solution...");

    // Check if there are any unique phonemes first, and include them in the initial solution
    let mut initial_solution = Solution {
        mons: Vec::new(),
        coverage: 0,
        cache_key: Vec::new(),
    };

    for (count, phone_id) in frequency {
        assert!(count > 0);
        if count > 1 {
            break;
        }

        let mut mons = mons_by_phone.remove(&phone_id).unwrap();
        assert_eq!(mons.len(), 1);
        let mon = mons.remove(0);

        let coverage = initial_solution.coverage | mon.phonemes_mask;
        if coverage == initial_solution.coverage {
            // We already have something to handle this
            continue;
        }

        initial_solution.coverage = coverage;
        initial_solution.mons.push(mon);

        // We don't need to update the cache key for these initial solutions. The solver wouldn't
        // consider these mon as their other phonemes will be pruned later, and they also won't
        // improve coverage.
    }

    // We have some initial solution
    if initial_solution.coverage != 0 {
        assert!(!initial_solution.mons.is_empty());
        // Keep phonemes that are not represented by our unique-phoneme Pokemon.
        mons_by_phone.retain(|&k, _| (1 << k) & initial_solution.coverage == 0);
    } else {
        assert!(initial_solution.mons.is_empty());
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
        let lookup: BTreeMap<u8, &Vec<&Pokémon>> = mons_by_phone
            .iter()
            .filter_map(|(&k, v)| {
                if step.coverage & (1 << k) == 0 {
                    Some((k, v))
                } else {
                    None
                }
            })
            .collect();

        let mut list_of_solutions = solve(&lookup, &step, max_coverage);
        // println!("solver gave {} solutions", list_of_solutions.len());
        peak_candidate_len = peak_candidate_len.max(list_of_solutions.len());
        list_of_solutions.sort_by_key(|s| u32::MAX - s.coverage.count_ones());

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
                    let idx = queue.partition_point(|s| {
                        s.coverage.count_ones() > solution.coverage.count_ones()
                    });
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
    println!("Done, tried {solution_count} candidates, {peak_queue_len} peak queue length, {peak_candidate_len} peak solver length, {} cache entries", cache.len());

    #[cfg(feature = "memory-stats")]
    {
        let (now, peak) = get_memory_stats();
        println!("Memory usage after running solver: {now} now, {peak} peak");
    }
}
