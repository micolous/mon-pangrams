# mon-pangrams

My lazy Rust port of [`pokemon-phonetic-pangrams`][0].

## Building / running

Clone the repository with submodules to pull in Graham's original CSV files. Otherwise, you'll need
to fetch them yourself.

You'll need a recent Rust toolchain, then run:

```sh
cargo build --release
```

Then run with:

```sh
./target/release/mon-pangrams pokemon-phonetic-pangrams/pokemon_gen_1_ipa_pronunciations.csv
```

> [!NOTE]
> Running with data from _all_ Pokémon generations is slow.

## Changes from the upstream TypeScript version

- Syllabic `̩h` (`\u{329}h`) is replaced with `ḥ` to make it a single codepoint, which fixes an issue
  with Hydreigon.

- This program uses a `u64` bitmask to indicate which phones were represented, rather than a `Set`
  of one-codepoint strings.

- This program considers a solution to be "complete" if all phonemes in the input data are
  represented in a solution, not just if it covers all 40 phonemes in US English Pokémon names.

- This program automatically skips any potential solution which isn't better than the last best
  solution, so you'll get at most one answer for each length.

[0]: https://graham.build/s/a-blog/034-pokemon-gen-1-phonetic-pangram/
