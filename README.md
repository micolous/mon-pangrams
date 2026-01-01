# mon-pangrams

My lazy Rust port of [`pokemon-phonetic-pangrams`][0], based on [this blog post][0].

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

## Example output

```
$ ./target/release/mon-pangrams pokemon-phonetic-pangrams/pokemon_gen_1_ipa_pronunciations.csv
There are 136 Pokémon that are not a subset of another's pronunciation:
  [000] = Charmeleon          , mask: 0x20860003b0, bits:  9
  [001] = Victreebel          , mask:   0xd00050d1, bits:  9
  [002] = Charmander          , mask: 0x208a080302, bits:  8
  [003] = Hitmonchan          , mask: 0x2042081308, bits:  8
  [004] = Magikarp            , mask: 0x10c2080540, bits:  8
  [005] = Jigglypuff          , mask: 0x1064000494, bits:  8
  [006] = Electrode           , mask:   0xd08010c2, bits:  8
  [007] = Articuno            , mask:   0xc2802242, bits:  8
  [008] = Exeggutor           , mask:   0xb2013020, bits:  8
  [009] = Dragonite           , mask:   0xa4181202, bits:  8
  [010] = Aerodactyl          , mask:   0x940810c2, bits:  8
  [011] = Tentacruel          , mask:   0x940032c0, bits:  8
  [012] = Bellsprout          , mask:   0x90021c81, bits:  8
  [013] = Wigglytuff          , mask:   0x64009094, bits:  8
  [014] = Weepinbell          , mask:   0x50008691, bits:  8
  [015] = Hitmonlee           , mask:   0x42001398, bits:  8
  [016] = Exeggcute           , mask:   0x34013060, bits:  8
  [017] = Kangaskhan          , mask:   0x27080a40, bits:  8
  [018] = Poliwrath           , mask: 0x4082080490, bits:  7
  [019] = Venomoth            , mask: 0x4012804300, bits:  7
  [020] = Bulbasaur           , mask:  0x286000881, bits:  7
  [021] = Vulpix              , mask:  0x240004cc0, bits:  7
  [022] = Sandshrew           , mask:  0x180082a02, bits:  7
  [023] = Raticate            , mask:   0xc00c1042, bits:  7
  [024] = Gyarados            , mask:   0xb4800802, bits:  7
  [025] = Dragonair           , mask:   0xb4080202, bits:  7
  [026] = Graveler            , mask:   0xac084080, bits:  7
  [027] = Dugtrio             , mask:   0xa4801012, bits:  7
  [028] = Parasect            , mask:   0x94001c40, bits:  7
  [029] = Flareon             , mask:   0x92000294, bits:  7
  [030] = Marowak             , mask:   0x90888140, bits:  7
  [031] = Butterfree          , mask:   0x8c000017, bits:  7
  [032] = Wartortle           , mask:   0x86009082, bits:  7
  [033] = Venusaur            , mask:   0x86004a10, bits:  7
  [034] = Snorlax             , mask:   0x82080ac0, bits:  7
  [035] = Vaporeon            , mask:   0x82044610, bits:  7
  [036] = Moltres             , mask:   0x82041980, bits:  7
  [037] = Nidoking            , mask:   0x41800252, bits:  7
  [038] = Magneton            , mask:   0x26081300, bits:  7
  [039] = Magnemite           , mask:   0x24181300, bits:  7
  [040] = Poliwag             , mask:   0x22088490, bits:  7
  [041] = Clefable            , mask:   0x140400c5, bits:  7
  [042] = Electabuzz          , mask:   0x140110c1, bits:  7
  [043] = Squirtle            , mask:    0xc0088c2, bits:  7
  [044] = Kabutops            , mask:    0x2003c41, bits:  7
  [045] = Growlithe           , mask: 0x40e0020080, bits:  6
  [046] = Charizard           , mask: 0x20c2010002, bits:  6
  [047] = Farfetch'd          , mask: 0x2092001004, bits:  6
  [048] = Pikachu             , mask: 0x2004002450, bits:  6
  [049] = Pidgeot             , mask: 0x1042001410, bits:  6
  [050] = Pidgeotto           , mask: 0x1040800412, bits:  6
  [051] = Jolteon             , mask: 0x1002001290, bits:  6
  [052] = Rapidash            , mask:  0x1c0080402, bits:  6
  [053] = Sandslash           , mask:  0x100080a82, bits:  6
  [054] = Beedrill            , mask:   0xc0000093, bits:  6
  [055] = Porygon             , mask:   0xa2000610, bits:  6
  [056] = Clefairy            , mask:   0x900000d4, bits:  6
  [057] = Arcanine            , mask:   0x86100240, bits:  6
  [058] = Omastar             , mask:   0x86001900, bits:  6
  [059] = Nidorina            , mask:   0x84800212, bits:  6
  [060] = Lapras              , mask:   0x84080c80, bits:  6
  [061] = Kadabra             , mask:   0x84080043, bits:  6
  [062] = Dratini             , mask:   0x84001212, bits:  6
  [063] = Ivysaur             , mask:   0x82104810, bits:  6
  [064] = Voltorb             , mask:   0x82005081, bits:  6
  [065] = Staryu              , mask:   0x82003820, bits:  6
  [066] = Starmie             , mask:   0x82001910, bits:  6
  [067] = Nidoran♂            , mask:   0x80880212, bits:  6
  [068] = Diglett             , mask:   0x64001082, bits:  6
  [069] = Mr. Mime            , mask:   0x48101900, bits:  6
  [070] = Lickitung           , mask:   0x450010c0, bits:  6
  [071] = Golduck             , mask:   0x260000c2, bits:  6
  [072] = Tangela             , mask:   0x25081080, bits:  6
  [073] = Golbat              , mask:   0x22081081, bits:  6
  [074] = Goldeen             , mask:   0x22000292, bits:  6
  [075] = Metapod             , mask:   0x16000502, bits:  6
  [076] = Venonat             , mask:   0x10885200, bits:  6
  [077] = Poliwhirl           , mask:    0xa008490, bits:  6
  [078] = Cloyster            , mask:    0x84018c0, bits:  6
  [079] = Caterpie            , mask:    0x8080452, bits:  6
  [080] = Omanyte             , mask:    0x6101300, bits:  6
  [081] = Kabuto              , mask:    0x4803041, bits:  6
  [082] = Alakazam            , mask:    0x40901c0, bits:  6
  [083] = Ponyta              , mask:    0x2801610, bits:  6
  [084] = Zapdos              , mask:     0x890c02, bits:  6
  [085] = Nidoqueen           , mask:     0x808252, bits:  6
  [086] = Cubone              , mask:     0x802261, bits:  6
  [087] = Blastoise           , mask:     0x481881, bits:  6
  [088] = Ninetales           , mask:     0x151280, bits:  6
  [089] = Vileplume           , mask:     0x106580, bits:  6
  [090] = Machop              , mask: 0x2006000500, bits:  5
  [091] = Machoke             , mask: 0x2004800140, bits:  5
  [092] = Machamp             , mask: 0x2004080500, bits:  5
  [093] = Chansey             , mask: 0x2000080a10, bits:  5
  [094] = Jynx                , mask: 0x1041000840, bits:  5
  [095] = Geodude             , mask: 0x1000802012, bits:  5
  [096] = Persian             , mask:  0x40c000600, bits:  5
  [097] = Shellder            , mask:  0x118000082, bits:  5
  [098] = Spearow             , mask:   0xc0800c00, bits:  5
  [099] = Gengar              , mask:   0xb3000000, bits:  5
  [100] = Grimer              , mask:   0xa8100100, bits:  5
  [101] = Magmar              , mask:   0xa2080100, bits:  5
  [102] = Seadra              , mask:   0x84000812, bits:  5
  [103] = Tauros              , mask:   0x82801800, bits:  5
  [104] = Rhyhorn             , mask:   0x82100208, bits:  5
  [105] = Rhydon              , mask:   0x82100202, bits:  5
  [106] = Horsea              , mask:   0x82000818, bits:  5
  [107] = Slowbro             , mask:   0x80800881, bits:  5
  [108] = Primeape            , mask:   0x80140500, bits:  5
  [109] = Krabby              , mask:   0x80080051, bits:  5
  [110] = Drowzee             , mask:   0x80030012, bits:  5
  [111] = Kingler             , mask:   0x490000c0, bits:  5
  [112] = Pinsir              , mask:   0x48000e00, bits:  5
  [113] = Koffing             , mask:   0x43000044, bits:  5
  [114] = Onix                , mask:   0x42000a40, bits:  5
  [115] = Weezing             , mask:   0x41018010, bits:  5
  [116] = Seaking             , mask:   0x41000850, bits:  5
  [117] = Hypno               , mask:   0x40800608, bits:  5
  [118] = Golem               , mask:   0x26000180, bits:  5
  [119] = Dewgong             , mask:   0x23002002, bits:  5
  [120] = Gastly              , mask:   0x20080890, bits:  5
  [121] = Ekans               , mask:   0x14010240, bits:  5
  [122] = Haunter             , mask:    0xa001208, bits:  5
  [123] = Psyduck             , mask:    0x4100842, bits:  5
  [124] = Weedle              , mask:    0x4008092, bits:  5
  [125] = Mankey              , mask:    0x1080150, bits:  5
  [126] = Slowpoke            , mask:     0x800cc0, bits:  5
  [127] = Zubat               , mask:      0x93001, bits:  5
  [128] = Scyther             , mask: 0x4008100800, bits:  4
  [129] = Meowth              , mask: 0x4000020110, bits:  4
  [130] = Raichu              , mask: 0x2080102000, bits:  4
  [131] = Oddish              , mask:  0x142000002, bits:  4
  [132] = Fearow              , mask:   0xc0800004, bits:  4
  [133] = Arbok               , mask:   0x82000041, bits:  4
  [134] = Gloom               , mask:   0x20002180, bits:  4
  [135] = Mewtwo              , mask:       0x3120, bits:  4

37 phones represented:
  b:  19 Pokémon
  d:  39 Pokémon
  f:   9 Pokémon
  h:   6 Pokémon
  i:  43 Pokémon
  j:   6 Pokémon
  k:  42 Pokémon
  l:  49 Pokémon
  m:  28 Pokémon
  n:  42 Pokémon
  p:  30 Pokémon
  s:  36 Pokémon
  t:  44 Pokémon
  u:  17 Pokémon
  v:  10 Pokémon
  w:  10 Pokémon
  z:  11 Pokémon
  Ø:   4 Pokémon
  ä:   6 Pokémon
  æ:  35 Pokémon
  ï:  15 Pokémon
  õ:   2 Pokémon
  ö:  24 Pokémon
  ŋ:  12 Pokémon
  ɑ:  52 Pokémon
  ə:  46 Pokémon
  ɚ:  15 Pokémon
  ɛ:  23 Pokémon
  ɡ:  27 Pokémon
  ɪ:  32 Pokémon
  ɹ:  62 Pokémon
  ʃ:   5 Pokémon
  ʊ:   2 Pokémon
  ʒ:   1 Pokémon
  ʤ:   7 Pokémon
  ʧ:  11 Pokémon
  θ:   5 Pokémon

Finding a solution...
 Solution #69 (11 Pokémon): Persian, Vulpix, Blastoise, Drowzee, Venomoth, Sandshrew, Primeape, Exeggcute, Hitmonchan, Jynx, Wigglytuff
 Solution #104466 (10 Pokémon): Persian, Vulpix, Blastoise, Meowth, Shellder, Primeape, Cubone, Hitmonchan, Jigglypuff, Weezing

Done, tried 348730 solutions
```

## Changes from the upstream TypeScript version

- Syllabic `̩h` (`\u{329}h`) is replaced with `ḥ` to make it a single codepoint, which fixes an issue
  with Hydreigon.

- This program uses a `u64` bitmask to indicate which phonemes were represented, rather than a `Set`
  of one-codepoint strings.

- This program considers all phonemes used by exactly 1 Pokémon as part of the initial solution,
  rather than going into a solve loop for each of them individually.
  
  eg: In the set of all Pokémon, `ḥ` and `ð` are each in one Pokémon. This program's initial
  solution includes both Hydregion and Slither Wing, rather than trying an empty initial solution
  and building a solution with each of them in the first position.

- This program automatically skips any potential solution which isn't better than the last best
  solution, so you'll get at most one answer for each length.

- This program considers a solution to be "complete" if all phonemes in the input data are
  represented in a solution, not just if it covers all 40 phonemes in US English Pokémon names.

[0]: https://graham.build/s/a-blog/034-pokemon-gen-1-phonetic-pangram/
