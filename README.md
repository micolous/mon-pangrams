# mon-pangrams

My Rust port of [`pokemon-phonetic-pangrams`][1], based on [Graham's blog post][0], with
[a bunch of optimisations](#changes-from-the-original-typescript-program) to make it faster and use
less memory.

## Solutions

If you don't want to run this yourself, here are the solutions that my program comes up with:

| Generation  | Pokémon | Non-subset Pokémon | Unique phonemes | Best solution                                                                                                                                         |
| :---------: | ------: | -----------------: | --------------: | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
|   **1st**   |     151 |                136 |              37 | **10 Pokémon**: Persian, Vulpix, Blastoise, Meowth, Shellder, Primeape, Cubone, Hitmonchan, Jigglypuff, Weezing                                       |
|   **2nd**   |     100 |                 95 |              35 | **11 Pokémon**: Typhlosion, Girafarig, Pupitar, Snubbull, Qwilfish, Misdreavus, Chinchou, Azumarill, Miltank, Remoraid, Heracross                     |
|   **3rd**   |     135 |                128 |              36 | **9 Pokémon**: Gulpin, Spoink, Loudred, Beautifly, Vigoroth, Whiscash, Jirachi, Illumise, Metagross                                                   |
|   **4th**   |     107 |                104 |              34 | **8 Pokémon**: Tangrowth, Mime Jr., Hippowdon, Shinx, Buizel, Staravia, Cherrim, Drifloon                                                             |
|   **5th**   |     156 |                136 |              37 | **10 Pokémon**: Purrloin, Duosion, Krookodile, Mienshao, Watchog, Tranquill, Simisage, Beheeyem, Ferrothorn, Virizion                                 |
|   **6th**   |      72 |                 69 |              34 | **9 Pokémon**: Meowstic, Aegislash, Noivern, Spewpa, Quilladin, Zygarde, Fletchling, Heliolisk, Braixen                                               |
|   **7th**   |      88 |                 84 |              34 | **9 Pokémon**: Hakamo-o, Charjabug, Type: Null, Bounsweet, Silvally, Yungoos, Turtonator, Gumshoos, Blacephalon                                       |
|   **8th**   |      96 |                 88 |              35 | **9 Pokémon**: Falinks, Copperajah, Hattrem, Thievul, Regidrago, Dubwool, Zacian, Chewtle, Eiscue                                                     |
|   **9th**   |     120 |                111 |          **38** | **11 Pokémon**: Slither Wing, Oinkologne, Tadbulb, Armarouge, Sandy Shocks, Iron Jugulis, Espathra, Maushold, Flutter Mane, Iron Leaves, Poltchageist |
| **1st-9th** |     937 |                749 |          **38** | **9 Pokémon**: Slither Wing, Typhlosion, Spoink, Granbull, Venomoth, Houndoom, Exeggutor, Shaymin, Jirachi                                            |

> [!NOTE]
> These aren't the _only_ solutions, [there will be others](#changes-from-the-original-typescript-program).

## Building / running

Clone the repository with submodules to pull in Graham's original CSV files. Otherwise, you'll need
to [fetch them yourself][1].

You'll need [to install a recent Rust toolchain][2], then run:

```sh
cargo build --release

# Or to build with memory stats reporting (which makes it slower):
cargo build --release --features memory-stats
```

Then run with:

```sh
./target/release/mon-pangrams pokemon-phonetic-pangrams/pokemon_gen_1_ipa_pronunciations.csv
```

## Example output

Times are from running on a late-2023 MacBook Pro with an 11-core M3 Pro SoC on macOS.

The algorithm is single-threaded, so won't see much benefit from more cores.

<details>

<summary>Generation 1 output</summary>

```
% time ./target/release/mon-pangrams pokemon-phonetic-pangrams/pokemon_gen_1_ipa_pronunciations.csv
Read 151 Pokémon
There are 136 Pokémon that do not use a subset of another's phonemes:
  [000] = Charmeleon           mask: 0x10860003b0, bits:  9
  [001] = Victreebel           mask:   0xd00050d1, bits:  9
  [002] = Charmander           mask: 0x108a080302, bits:  8
  [003] = Hitmonchan           mask: 0x1042081308, bits:  8
  [004] = Magikarp             mask:  0x8c2080540, bits:  8
  [005] = Jigglypuff           mask:  0x864000494, bits:  8
  [006] = Electrode            mask:   0xd08010c2, bits:  8
  [007] = Articuno             mask:   0xc2802242, bits:  8
  [008] = Exeggutor            mask:   0xb2013020, bits:  8
  [009] = Dragonite            mask:   0xa4181202, bits:  8
  [010] = Aerodactyl           mask:   0x940810c2, bits:  8
  [011] = Tentacruel           mask:   0x940032c0, bits:  8
  [012] = Bellsprout           mask:   0x90021c81, bits:  8
  [013] = Wigglytuff           mask:   0x64009094, bits:  8
  [014] = Weepinbell           mask:   0x50008691, bits:  8
  [015] = Hitmonlee            mask:   0x42001398, bits:  8
  [016] = Exeggcute            mask:   0x34013060, bits:  8
  [017] = Kangaskhan           mask:   0x27080a40, bits:  8
  [018] = Poliwrath            mask: 0x2082080490, bits:  7
  [019] = Venomoth             mask: 0x2012804300, bits:  7
  [020] = Bulbasaur            mask:  0x286000881, bits:  7
  [021] = Vulpix               mask:  0x240004cc0, bits:  7
  [022] = Sandshrew            mask:  0x180082a02, bits:  7
  [023] = Raticate             mask:   0xc00c1042, bits:  7
  [024] = Gyarados             mask:   0xb4800802, bits:  7
  [025] = Dragonair            mask:   0xb4080202, bits:  7
  [026] = Graveler             mask:   0xac084080, bits:  7
  [027] = Dugtrio              mask:   0xa4801012, bits:  7
  [028] = Parasect             mask:   0x94001c40, bits:  7
  [029] = Flareon              mask:   0x92000294, bits:  7
  [030] = Marowak              mask:   0x90888140, bits:  7
  [031] = Butterfree           mask:   0x8c000017, bits:  7
  [032] = Wartortle            mask:   0x86009082, bits:  7
  [033] = Venusaur             mask:   0x86004a10, bits:  7
  [034] = Snorlax              mask:   0x82080ac0, bits:  7
  [035] = Vaporeon             mask:   0x82044610, bits:  7
  [036] = Moltres              mask:   0x82041980, bits:  7
  [037] = Nidoking             mask:   0x41800252, bits:  7
  [038] = Magneton             mask:   0x26081300, bits:  7
  [039] = Magnemite            mask:   0x24181300, bits:  7
  [040] = Poliwag              mask:   0x22088490, bits:  7
  [041] = Clefable             mask:   0x140400c5, bits:  7
  [042] = Electabuzz           mask:   0x140110c1, bits:  7
  [043] = Squirtle             mask:    0xc0088c2, bits:  7
  [044] = Kabutops             mask:    0x2003c41, bits:  7
  [045] = Growlithe            mask: 0x20e0020080, bits:  6
  [046] = Charizard            mask: 0x10c2010002, bits:  6
  [047] = Farfetch'd           mask: 0x1092001004, bits:  6
  [048] = Pikachu              mask: 0x1004002450, bits:  6
  [049] = Pidgeot              mask:  0x842001410, bits:  6
  [050] = Pidgeotto            mask:  0x840800412, bits:  6
  [051] = Jolteon              mask:  0x802001290, bits:  6
  [052] = Rapidash             mask:  0x1c0080402, bits:  6
  [053] = Sandslash            mask:  0x100080a82, bits:  6
  [054] = Beedrill             mask:   0xc0000093, bits:  6
  [055] = Porygon              mask:   0xa2000610, bits:  6
  [056] = Clefairy             mask:   0x900000d4, bits:  6
  [057] = Arcanine             mask:   0x86100240, bits:  6
  [058] = Omastar              mask:   0x86001900, bits:  6
  [059] = Nidorina             mask:   0x84800212, bits:  6
  [060] = Lapras               mask:   0x84080c80, bits:  6
  [061] = Kadabra              mask:   0x84080043, bits:  6
  [062] = Dratini              mask:   0x84001212, bits:  6
  [063] = Ivysaur              mask:   0x82104810, bits:  6
  [064] = Voltorb              mask:   0x82005081, bits:  6
  [065] = Staryu               mask:   0x82003820, bits:  6
  [066] = Starmie              mask:   0x82001910, bits:  6
  [067] = Nidoran♂             mask:   0x80880212, bits:  6
  [068] = Diglett              mask:   0x64001082, bits:  6
  [069] = Mr. Mime             mask:   0x48101900, bits:  6
  [070] = Lickitung            mask:   0x450010c0, bits:  6
  [071] = Golduck              mask:   0x260000c2, bits:  6
  [072] = Tangela              mask:   0x25081080, bits:  6
  [073] = Golbat               mask:   0x22081081, bits:  6
  [074] = Goldeen              mask:   0x22000292, bits:  6
  [075] = Metapod              mask:   0x16000502, bits:  6
  [076] = Venonat              mask:   0x10885200, bits:  6
  [077] = Poliwhirl            mask:    0xa008490, bits:  6
  [078] = Cloyster             mask:    0x84018c0, bits:  6
  [079] = Caterpie             mask:    0x8080452, bits:  6
  [080] = Omanyte              mask:    0x6101300, bits:  6
  [081] = Kabuto               mask:    0x4803041, bits:  6
  [082] = Alakazam             mask:    0x40901c0, bits:  6
  [083] = Ponyta               mask:    0x2801610, bits:  6
  [084] = Zapdos               mask:     0x890c02, bits:  6
  [085] = Nidoqueen            mask:     0x808252, bits:  6
  [086] = Cubone               mask:     0x802261, bits:  6
  [087] = Blastoise            mask:     0x481881, bits:  6
  [088] = Ninetales            mask:     0x151280, bits:  6
  [089] = Vileplume            mask:     0x106580, bits:  6
  [090] = Machop               mask: 0x1006000500, bits:  5
  [091] = Machoke              mask: 0x1004800140, bits:  5
  [092] = Machamp              mask: 0x1004080500, bits:  5
  [093] = Chansey              mask: 0x1000080a10, bits:  5
  [094] = Jynx                 mask:  0x841000840, bits:  5
  [095] = Geodude              mask:  0x800802012, bits:  5
  [096] = Persian              mask:  0x40c000600, bits:  5
  [097] = Shellder             mask:  0x118000082, bits:  5
  [098] = Spearow              mask:   0xc0800c00, bits:  5
  [099] = Gengar               mask:   0xb3000000, bits:  5
  [100] = Grimer               mask:   0xa8100100, bits:  5
  [101] = Magmar               mask:   0xa2080100, bits:  5
  [102] = Seadra               mask:   0x84000812, bits:  5
  [103] = Tauros               mask:   0x82801800, bits:  5
  [104] = Rhyhorn              mask:   0x82100208, bits:  5
  [105] = Rhydon               mask:   0x82100202, bits:  5
  [106] = Horsea               mask:   0x82000818, bits:  5
  [107] = Slowbro              mask:   0x80800881, bits:  5
  [108] = Primeape             mask:   0x80140500, bits:  5
  [109] = Krabby               mask:   0x80080051, bits:  5
  [110] = Drowzee              mask:   0x80030012, bits:  5
  [111] = Kingler              mask:   0x490000c0, bits:  5
  [112] = Pinsir               mask:   0x48000e00, bits:  5
  [113] = Koffing              mask:   0x43000044, bits:  5
  [114] = Onix                 mask:   0x42000a40, bits:  5
  [115] = Weezing              mask:   0x41018010, bits:  5
  [116] = Seaking              mask:   0x41000850, bits:  5
  [117] = Hypno                mask:   0x40800608, bits:  5
  [118] = Golem                mask:   0x26000180, bits:  5
  [119] = Dewgong              mask:   0x23002002, bits:  5
  [120] = Gastly               mask:   0x20080890, bits:  5
  [121] = Ekans                mask:   0x14010240, bits:  5
  [122] = Haunter              mask:    0xa001208, bits:  5
  [123] = Psyduck              mask:    0x4100842, bits:  5
  [124] = Weedle               mask:    0x4008092, bits:  5
  [125] = Mankey               mask:    0x1080150, bits:  5
  [126] = Slowpoke             mask:     0x800cc0, bits:  5
  [127] = Zubat                mask:      0x93001, bits:  5
  [128] = Scyther              mask: 0x2008100800, bits:  4
  [129] = Meowth               mask: 0x2000020110, bits:  4
  [130] = Raichu               mask: 0x1080102000, bits:  4
  [131] = Oddish               mask:  0x142000002, bits:  4
  [132] = Fearow               mask:   0xc0800004, bits:  4
  [133] = Arbok                mask:   0x82000041, bits:  4
  [134] = Gloom                mask:   0x20002180, bits:  4
  [135] = Mewtwo               mask:       0x3120, bits:  4

37 phonemes represented:
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

Memory usage before running solver: 24858 now, 24706 peak

Finding a solution...
 Solution #69 (11 Pokémon): Persian, Vulpix, Blastoise, Drowzee, Venomoth, Sandshrew, Ninetales, Exeggutor, Hitmonchan, Jynx, Wigglytuff
 Solution #8309 (10 Pokémon): Persian, Vulpix, Blastoise, Meowth, Shellder, Primeape, Cubone, Hitmonchan, Jigglypuff, Weezing

Done, tried 56636 candidates, 39 peak queue length, 18 peak solver length, 10965 cache entries
Memory usage after running solver: 197418 now, 200354 peak
./target/release/mon-pangrams   0.01s user 0.00s system 90% cpu 0.015 total
```

</details>

<details>

<summary>All generations output</summary>

```
% time ./target/release/mon-pangrams pokemon-phonetic-pangrams/pokemon_ipa_pronunciations.csv
Read 937 Pokémon
There are 749 Pokémon that do not use a subset of another's phonemes:
  [000] = Basculegion          mask:  0x804082af1, bits: 11
  [001] = Raging Bolt          mask:  0x8d3001091, bits: 10
  [002] = Iron Jugulis         mask:  0x8a4102aa0, bits: 10
  [003] = Centiskorch          mask: 0x1096001a40, bits:  9
  [004] = Charmeleon           mask: 0x10860003b0, bits:  9
  [005] = Poltchageist         mask: 0x1026101c80, bits:  9
  [006] = Dragapult            mask:  0x2a4081482, bits:  9
  [007] = Sprigatito           mask:   0xe4801c10, bits:  9
  [008] = Electivire           mask:   0xd41050c0, bits:  9
  [009] = Escavalier           mask:   0xd40848c0, bits:  9
  [010] = Victreebel           mask:   0xd00050d1, bits:  9
  [011] = Brambleghast         mask:   0xa4081981, bits:  9
  [012] = Gastrodon            mask:   0xa2881a02, bits:  9
  [013] = Zebstrika            mask:   0x94111841, bits:  9
  [014] = Barraskewda          mask:   0x94002863, bits:  9
  [015] = Kilowattrel          mask:   0x868090d0, bits:  9
  [016] = Dracozolt            mask:   0x860910c2, bits:  9
  [017] = Karrablast           mask:   0x860818c1, bits:  9
  [018] = Meowscarada          mask:   0x86020952, bits:  9
  [019] = Iron Valiant         mask:   0x841852a0, bits:  9
  [020] = Reuniclus            mask:   0x84002af0, bits:  9
  [021] = Kingambit            mask:   0x61081151, bits:  9
  [022] = Fezandipiti          mask:   0x54010616, bits:  9
  [023] = Squawkabilly         mask:   0x460088d1, bits:  9
  [024] = Basculin             mask:   0x40082ae1, bits:  9
  [025] = Bastiodon            mask:    0x2881a13, bits:  9
  [026] = Relicanth            mask: 0x20d00802c0, bits:  8
  [027] = Chikorita            mask: 0x10c6000052, bits:  8
  [028] = Charcadet            mask: 0x1096001042, bits:  8
  [029] = Charmander           mask: 0x108a080302, bits:  8
  [030] = Archaludon           mask: 0x1086080282, bits:  8
  [031] = Fletchinder          mask: 0x1058000286, bits:  8
  [032] = Hitmonchan           mask: 0x1042081308, bits:  8
  [033] = Galvantula           mask: 0x1024086280, bits:  8
  [034] = Poochyena            mask: 0x1014002630, bits:  8
  [035] = Munchlax             mask: 0x1004080bc0, bits:  8
  [036] = Regidrago            mask:  0x8f0800012, bits:  8
  [037] = Girafarig            mask:  0x8ec080004, bits:  8
  [038] = Magikarp             mask:  0x8c2080540, bits:  8
  [039] = Frigibax             mask:  0x8c0080845, bits:  8
  [040] = Dragalge             mask:  0x8a4080092, bits:  8
  [041] = Jigglypuff           mask:  0x864000494, bits:  8
  [042] = Jellicent            mask:  0x854001a80, bits:  8
  [043] = Gouging Fire         mask:  0x829120014, bits:  8
  [044] = Skeledirge           mask:  0x81c0008c2, bits:  8
  [045] = Typhlosion           mask:  0x404901284, bits:  8
  [046] = Chandelure           mask:  0x384080282, bits:  8
  [047] = Clobbopus            mask:  0x206000cc1, bits:  8
  [048] = Arctovish            mask:  0x1c6005040, bits:  8
  [049] = Dracovish            mask:  0x1c4084042, bits:  8
  [050] = Sandy Shocks         mask:  0x102080a52, bits:  8
  [051] = Great Tusk           mask:   0xf4001840, bits:  8
  [052] = Tatsugiri            mask:   0xe2003810, bits:  8
  [053] = Grimmsnarl           mask:   0xe2000b80, bits:  8
  [054] = Excadrill            mask:   0xd40008c2, bits:  8
  [055] = Electrode            mask:   0xd08010c2, bits:  8
  [056] = Manectric            mask:   0xd0041340, bits:  8
  [057] = Toxtricity           mask:   0xc6001850, bits:  8
  [058] = Silicobra            mask:   0xc48008c1, bits:  8
  [059] = Spinarak             mask:   0xc4080e40, bits:  8
  [060] = Misdreavus           mask:   0xc4004912, bits:  8
  [061] = Spiritomb            mask:   0xc4003d00, bits:  8
  [062] = Articuno             mask:   0xc2802242, bits:  8
  [063] = Arctibax             mask:   0xc2081841, bits:  8
  [064] = Darmanitan           mask:   0xc2081302, bits:  8
  [065] = Tranquill            mask:   0xc10890c0, bits:  8
  [066] = Metagross            mask:   0xb4801900, bits:  8
  [067] = Runerigus            mask:   0xb4002a10, bits:  8
  [068] = Accelgor             mask:   0xb20808c0, bits:  8
  [069] = Exeggutor            mask:   0xb2013020, bits:  8
  [070] = Glastrier            mask:   0xa8081890, bits:  8
  [071] = Floragato            mask:   0xa6801084, bits:  8
  [072] = Cryogonal            mask:   0xa61002c0, bits:  8
  [073] = Garganacl            mask:   0xa60802c0, bits:  8
  [074] = Cofagrigus           mask:   0xa6000854, bits:  8
  [075] = Dragonite            mask:   0xa4181202, bits:  8
  [076] = Grapploct            mask:   0xa20814c0, bits:  8
  [077] = Porygon2             mask:   0xa2003610, bits:  8
  [078] = Spectrier            mask:   0x98001c50, bits:  8
  [079] = Larvesta             mask:   0x96005880, bits:  8
  [080] = Aerodactyl           mask:   0x940810c2, bits:  8
  [081] = Tentacruel           mask:   0x940032c0, bits:  8
  [082] = Eelektross           mask:   0x920018d0, bits:  8
  [083] = Bellsprout           mask:   0x90021c81, bits:  8
  [084] = Cramorant            mask:   0x8c081340, bits:  8
  [085] = Iron Boulder         mask:   0x8a100283, bits:  8
  [086] = Munkidori            mask:   0x87000152, bits:  8
  [087] = Volcarona            mask:   0x868042c0, bits:  8
  [088] = Cyclizar             mask:   0x861108c0, bits:  8
  [089] = Corvisquire          mask:   0x8610c840, bits:  8
  [090] = Corviknight          mask:   0x86105240, bits:  8
  [091] = Staravia             mask:   0x86045810, bits:  8
  [092] = Tornadus             mask:   0x86041a02, bits:  8
  [093] = Dusknoir             mask:   0x86008a42, bits:  8
  [094] = Scorbunny            mask:   0x86000a51, bits:  8
  [095] = Hydrapple            mask:   0x8418048a, bits:  8
  [096] = Rampardos            mask:   0x82880d02, bits:  8
  [097] = Castform             mask:   0x82081944, bits:  8
  [098] = Cranidos             mask:   0x80840a52, bits:  8
  [099] = Toedscruel           mask:   0x808130c2, bits:  8
  [100] = Scream Tail          mask:   0x800419d0, bits:  8
  [101] = Wigglytuff           mask:   0x64009094, bits:  8
  [102] = Vespiquen            mask:   0x5000ce40, bits:  8
  [103] = Weepinbell           mask:   0x50008691, bits:  8
  [104] = Clawitzer            mask:   0x4a0098c0, bits:  8
  [105] = Slither Wing         mask:   0x49208890, bits:  8
  [106] = Whirlipede           mask:   0x4800849a, bits:  8
  [107] = Hippopotas           mask:   0x46801c08, bits:  8
  [108] = Mabosstiff           mask:   0x46001905, bits:  8
  [109] = Scovillain           mask:   0x44804ac0, bits:  8
  [110] = Mandibuzz            mask:   0x44090303, bits:  8
  [111] = Sizzlipede           mask:   0x44010c92, bits:  8
  [112] = Cyndaquil            mask:   0x44008ac2, bits:  8
  [113] = Whimsicott           mask:   0x42009948, bits:  8
  [114] = Hitmonlee            mask:   0x42001398, bits:  8
  [115] = Scolipede            mask:   0x40800cd2, bits:  8
  [116] = Exeggcute            mask:   0x34013060, bits:  8
  [117] = Terapagos            mask:   0x2e801c00, bits:  8
  [118] = Feraligatr           mask:   0x2c0c0086, bits:  8
  [119] = Scatterbug           mask:   0x2c081841, bits:  8
  [120] = Kangaskhan           mask:   0x27080a40, bits:  8
  [121] = Obstagoon            mask:   0x26003a01, bits:  8
  [122] = Polteageist          mask:   0x22101c90, bits:  8
  [123] = Mightyena            mask:   0x14101330, bits:  8
  [124] = Flutter Mane         mask:    0xc040386, bits:  8
  [125] = Volcanion            mask:    0x61042d0, bits:  8
  [126] = Bouffalant           mask:    0x6003285, bits:  8
  [127] = Makuhita             mask:    0x6003158, bits:  8
  [128] = Seismitoad           mask:    0x4911902, bits:  8
  [129] = Cobalion             mask:    0x48402d1, bits:  8
  [130] = Beautifly            mask:    0x41030a5, bits:  8
  [131] = Talonflame           mask:    0x40c1384, bits:  8
  [132] = Tandemaus            mask:    0x40a1b02, bits:  8
  [133] = Gothorita            mask: 0x20a6000012, bits:  7
  [134] = Tangrowth            mask: 0x20a1881000, bits:  7
  [135] = Espathra             mask: 0x2094080c00, bits:  7
  [136] = Poliwrath            mask: 0x2082080490, bits:  7
  [137] = Gothitelle           mask: 0x2072001080, bits:  7
  [138] = Venomoth             mask: 0x2012804300, bits:  7
  [139] = Trapinch             mask: 0x10c0081600, bits:  7
  [140] = Pecharunt            mask: 0x1094001600, bits:  7
  [141] = Tarountula           mask: 0x1084021280, bits:  7
  [142] = Honchkrow            mask: 0x1082800248, bits:  7
  [143] = Pachirisu            mask: 0x1082002c10, bits:  7
  [144] = Pincurchin           mask: 0x104c000640, bits:  7
  [145] = Shedinja             mask:  0x954000202, bits:  7
  [146] = Aegislash            mask:  0x940080890, bits:  7
  [147] = Florges              mask:  0x8c2000884, bits:  7
  [148] = Regigigas            mask:  0x8b4000810, bits:  7
  [149] = Ceruledge            mask:  0x894002880, bits:  7
  [150] = Regieleki            mask:  0x8940000d0, bits:  7
  [151] = Registeel            mask:  0x890001890, bits:  7
  [152] = Genesect             mask:  0x814001a40, bits:  7
  [153] = Mime Jr.             mask:  0x808102320, bits:  7
  [154] = Granbull             mask:  0x2a0080281, bits:  7
  [155] = Bulbasaur            mask:  0x286000881, bits:  7
  [156] = Krookodile           mask:  0x2809000c2, bits:  7
  [157] = Vulpix               mask:  0x240004cc0, bits:  7
  [158] = Musharna             mask:  0x186002300, bits:  7
  [159] = Sharpedo             mask:  0x182800412, bits:  7
  [160] = Sandshrew            mask:  0x180082a02, bits:  7
  [161] = Whiscash             mask:  0x140088848, bits:  7
  [162] = Gorebyss             mask:   0xe6000801, bits:  7
  [163] = Frogadier            mask:   0xe6000006, bits:  7
  [164] = Giratina             mask:   0xe4001210, bits:  7
  [165] = Smeargle             mask:   0xe4000980, bits:  7
  [166] = Trevenant            mask:   0xd4005200, bits:  7
  [167] = Mesprit              mask:   0xd0001d00, bits:  7
  [168] = Bombirdier           mask:   0xca000103, bits:  7
  [169] = Mr. Rime             mask:   0xc8101900, bits:  7
  [170] = Azumarill            mask:   0xc4012180, bits:  7
  [171] = Kricketune           mask:   0xc4003240, bits:  7
  [172] = Rillaboom            mask:   0xc4002181, bits:  7
  [173] = Toxicroak            mask:   0xc2801840, bits:  7
  [174] = Carnivine            mask:   0xc2104240, bits:  7
  [175] = Miraidon             mask:   0xc2100302, bits:  7
  [176] = Simipour             mask:   0xc2000d10, bits:  7
  [177] = Raticate             mask:   0xc00c1042, bits:  7
  [178] = Bramblin             mask:   0xc0080381, bits:  7
  [179] = Braixen              mask:   0xc0040a41, bits:  7
  [180] = Cradily              mask:   0xc00400d2, bits:  7
  [181] = Drifloon             mask:   0xc0002286, bits:  7
  [182] = Drifblim             mask:   0xc0000187, bits:  7
  [183] = Gyarados             mask:   0xb4800802, bits:  7
  [184] = Dragonair            mask:   0xb4080202, bits:  7
  [185] = Graveler             mask:   0xac084080, bits:  7
  [186] = Garbodor             mask:   0xaa800003, bits:  7
  [187] = Magmortar            mask:   0xaa081100, bits:  7
  [188] = Roggenrola           mask:   0xa6800280, bits:  7
  [189] = Gardevoir            mask:   0xa600c002, bits:  7
  [190] = Wugtrio              mask:   0xa4809010, bits:  7
  [191] = Dugtrio              mask:   0xa4801012, bits:  7
  [192] = Hydreigon            mask:   0xa410020a, bits:  7
  [193] = Greedent             mask:   0xa4001212, bits:  7
  [194] = Magcargo             mask:   0xa2880140, bits:  7
  [195] = Gliscor              mask:   0xa21008c0, bits:  7
  [196] = Porygon-Z            mask:   0xa2010610, bits:  7
  [197] = Heracross            mask:   0x96000848, bits:  7
  [198] = Barbaracle           mask:   0x960000c1, bits:  7
  [199] = Roselia              mask:   0x94810090, bits:  7
  [200] = Ariados              mask:   0x94800812, bits:  7
  [201] = Parasect             mask:   0x94001c40, bits:  7
  [202] = Cresselia            mask:   0x940008d0, bits:  7
  [203] = Morpeko              mask:   0x92800540, bits:  7
  [204] = Flareon              mask:   0x92000294, bits:  7
  [205] = Marowak              mask:   0x90888140, bits:  7
  [206] = Iron Treads          mask:   0x90111202, bits:  7
  [207] = Butterfree           mask:   0x8c000017, bits:  7
  [208] = Staraptor            mask:   0x8a081c00, bits:  7
  [209] = Skarmory             mask:   0x8a000950, bits:  7
  [210] = Tyranitar            mask:   0x86181200, bits:  7
  [211] = Ampharos             mask:   0x86080904, bits:  7
  [212] = Haxorus              mask:   0x86080848, bits:  7
  [213] = Wartortle            mask:   0x86009082, bits:  7
  [214] = Venusaur             mask:   0x86004a10, bits:  7
  [215] = Pupitar              mask:   0x86003420, bits:  7
  [216] = Brute Bonnet         mask:   0x86003201, bits:  7
  [217] = Darumaka             mask:   0x86002142, bits:  7
  [218] = Tropius              mask:   0x86001c10, bits:  7
  [219] = Dudunsparce          mask:   0x86000e02, bits:  7
  [220] = Sunflora             mask:   0x86000a84, bits:  7
  [221] = Umbreon              mask:   0x86000311, bits:  7
  [222] = Hariyama             mask:   0x86000138, bits:  7
  [223] = Orbeetle             mask:   0x86000093, bits:  7
  [224] = Aromatisse           mask:   0x84801910, bits:  7
  [225] = Tyrantrum            mask:   0x84181300, bits:  7
  [226] = Iron Bundle          mask:   0x84100283, bits:  7
  [227] = Maractus             mask:   0x84081940, bits:  7
  [228] = Alcremie             mask:   0x840801d0, bits:  7
  [229] = Roaring Moon         mask:   0x83002310, bits:  7
  [230] = Lucario              mask:   0x828020d0, bits:  7
  [231] = Snorlax              mask:   0x82080ac0, bits:  7
  [232] = Drapion              mask:   0x82080612, bits:  7
  [233] = Vaporeon             mask:   0x82044610, bits:  7
  [234] = Moltres              mask:   0x82041980, bits:  7
  [235] = Skorupi              mask:   0x82002c50, bits:  7
  [236] = Iron Hands           mask:   0x8019020a, bits:  7
  [237] = Iron Leaves          mask:   0x80114290, bits:  7
  [238] = Scrafty              mask:   0x80081854, bits:  7
  [239] = Diggersby            mask:   0x68010013, bits:  7
  [240] = Togekiss             mask:   0x64801840, bits:  7
  [241] = Mismagius            mask:   0x64080910, bits:  7
  [242] = Igglybuff            mask:   0x64000095, bits:  7
  [243] = Inteleon             mask:   0x54001290, bits:  7
  [244] = Venipede             mask:   0x50004612, bits:  7
  [245] = Octillery            mask:   0x4a0010d0, bits:  7
  [246] = Overqwil             mask:   0x4880c0c0, bits:  7
  [247] = Baxcalibur           mask:   0x480808c1, bits:  7
  [248] = Quilava              mask:   0x4600c0c0, bits:  7
  [249] = Vanillite            mask:   0x44105280, bits:  7
  [250] = Capsakid             mask:   0x44080c42, bits:  7
  [251] = Palafin              mask:   0x44080684, bits:  7
  [252] = Luvdisc              mask:   0x440048c2, bits:  7
  [253] = Stunfisk             mask:   0x44001a44, bits:  7
  [254] = Hippowdon            mask:   0x4202060a, bits:  7
  [255] = Sylveon              mask:   0x42004a90, bits:  7
  [256] = Lumineon             mask:   0x42002390, bits:  7
  [257] = Hitmontop            mask:   0x42001708, bits:  7
  [258] = Nidoking             mask:   0x41800252, bits:  7
  [259] = Miltank              mask:   0x410811c0, bits:  7
  [260] = Falinks              mask:   0x410408c4, bits:  7
  [261] = Milotic              mask:   0x409011c0, bits:  7
  [262] = Heliolisk            mask:   0x408008d8, bits:  7
  [263] = Illumise             mask:   0x40052190, bits:  7
  [264] = Meowstic             mask:   0x40021950, bits:  7
  [265] = Skiploom             mask:   0x40002dc0, bits:  7
  [266] = Eldegoss             mask:   0x36000882, bits:  7
  [267] = Yanmega              mask:   0x34080320, bits:  7
  [268] = Gossifleur           mask:   0x2e000884, bits:  7
  [269] = Quagsire             mask:   0x28188840, bits:  7
  [270] = Gholdengo            mask:   0x26800282, bits:  7
  [271] = Magneton             mask:   0x26081300, bits:  7
  [272] = Magnezone            mask:   0x24890300, bits:  7
  [273] = Flamigo              mask:   0x24800194, bits:  7
  [274] = Magnemite            mask:   0x24181300, bits:  7
  [275] = Poliwag              mask:   0x22088490, bits:  7
  [276] = Glaceon              mask:   0x22040a90, bits:  7
  [277] = Teddiursa            mask:   0x1c001812, bits:  7
  [278] = Conkeldurr           mask:   0x1b0000c2, bits:  7
  [279] = Zamazenta            mask:   0x16011300, bits:  7
  [280] = Wobbuffet            mask:   0x16009005, bits:  7
  [281] = Bellossom            mask:   0x16000981, bits:  7
  [282] = Salamence            mask:   0x14080b80, bits:  7
  [283] = Clefable             mask:   0x140400c5, bits:  7
  [284] = Electabuzz           mask:   0x140110c1, bits:  7
  [285] = Delphox              mask:   0x120008c6, bits:  7
  [286] = Empoleon             mask:   0x12000790, bits:  7
  [287] = Lampent              mask:   0x10081780, bits:  7
  [288] = Delcatty             mask:   0x100810d2, bits:  7
  [289] = Exploud              mask:   0x10020cc2, bits:  7
  [290] = Duraludon            mask:    0xe080282, bits:  7
  [291] = Virizion             mask:    0xc114210, bits:  7
  [292] = Camerupt             mask:    0xc081540, bits:  7
  [293] = Terrakion            mask:    0xc081250, bits:  7
  [294] = Landorus             mask:    0xc080a82, bits:  7
  [295] = Squirtle             mask:    0xc0088c2, bits:  7
  [296] = Clodsire             mask:    0xa1008c2, bits:  7
  [297] = Swampert             mask:    0xa009d00, bits:  7
  [298] = Masquerain           mask:    0x80c0b40, bits:  7
  [299] = Abomasnow            mask:    0x6800b01, bits:  7
  [300] = Quaquaval            mask:    0x608c0c0, bits:  7
  [301] = Sandaconda           mask:    0x6080a42, bits:  7
  [302] = Boltund              mask:    0x6001283, bits:  7
  [303] = Dusclops             mask:    0x6000cc2, bits:  7
  [304] = Dachsbun             mask:    0x6000a43, bits:  7
  [305] = Oinkologne           mask:    0x5c002c0, bits:  7
  [306] = Skuntank             mask:    0x5081a40, bits:  7
  [307] = Palpitoad            mask:    0x4881482, bits:  7
  [308] = Naclstack            mask:    0x4081ac0, bits:  7
  [309] = Phantump             mask:    0x4081704, bits:  7
  [310] = Blaziken             mask:    0x40502c1, bits:  7
  [311] = Stoutland            mask:    0x4021a82, bits:  7
  [312] = Cufant               mask:    0x4003264, bits:  7
  [313] = Politoed             mask:    0x2801492, bits:  7
  [314] = Helioptile           mask:    0x2101498, bits:  7
  [315] = Maushold             mask:    0x202098a, bits:  7
  [316] = Swadloon             mask:    0x200aa82, bits:  7
  [317] = Kabutops             mask:    0x2003c41, bits:  7
  [318] = Mamoswine            mask:     0x988b00, bits:  7
  [319] = Piloswine            mask:     0x908e80, bits:  7
  [320] = Houndstone           mask:     0x821a0a, bits:  7
  [321] = Growlithe            mask: 0x20e0020080, bits:  6
  [322] = Anorith              mask: 0x20c4080200, bits:  6
  [323] = Orthworm             mask: 0x208a008100, bits:  6
  [324] = Ferrothorn           mask: 0x208a000204, bits:  6
  [325] = Iron Thorns          mask: 0x2082110200, bits:  6
  [326] = Iron Moth            mask: 0x2082100300, bits:  6
  [327] = Frosmoth             mask: 0x2082000904, bits:  6
  [328] = Vigoroth             mask: 0x206a004000, bits:  6
  [329] = Thundurus            mask: 0x200c000a02, bits:  6
  [330] = Slakoth              mask: 0x20020808c0, bits:  6
  [331] = Charizard            mask: 0x10c2010002, bits:  6
  [332] = Torchic              mask: 0x10c2001040, bits:  6
  [333] = Garchomp             mask: 0x10a2000500, bits:  6
  [334] = Farfetch'd           mask: 0x1092001004, bits:  6
  [335] = Fletchling           mask: 0x1051000084, bits:  6
  [336] = Chespin              mask: 0x1050000e00, bits:  6
  [337] = Minccino             mask: 0x1040800310, bits:  6
  [338] = Maschiff             mask: 0x1040080904, bits:  6
  [339] = Sirfetch'd           mask: 0x1018001804, bits:  6
  [340] = Medicham             mask: 0x1014080102, bits:  6
  [341] = Chimecho             mask: 0x1010900140, bits:  6
  [342] = Wo-Chien             mask: 0x1010808210, bits:  6
  [343] = Chien-Pao            mask: 0x1010020610, bits:  6
  [344] = Clauncher            mask: 0x100a0002c0, bits:  6
  [345] = Hawlucha             mask: 0x1006002088, bits:  6
  [346] = Lechonk              mask: 0x10060002c0, bits:  6
  [347] = Pancham              mask: 0x1004080700, bits:  6
  [348] = Pikachu              mask: 0x1004002450, bits:  6
  [349] = Greninja             mask:  0x8e4000200, bits:  6
  [350] = Regirock             mask:  0x892000050, bits:  6
  [351] = Sigilyph             mask:  0x844000884, bits:  6
  [352] = Pidgeot              mask:  0x842001410, bits:  6
  [353] = Joltik               mask:  0x8420010c0, bits:  6
  [354] = Pidgeotto            mask:  0x840800412, bits:  6
  [355] = Ninjask              mask:  0x840080a40, bits:  6
  [356] = Simisage             mask:  0x840040910, bits:  6
  [357] = Elgyem               mask:  0x8100001b0, bits:  6
  [358] = Stonjourner          mask:  0x808801a00, bits:  6
  [359] = Jumpluff             mask:  0x804000584, bits:  6
  [360] = Jolteon              mask:  0x802001290, bits:  6
  [361] = Pansage              mask:  0x8000c0e00, bits:  6
  [362] = Armarouge            mask:  0x48a002100, bits:  6
  [363] = Copperajah           mask:  0x40e000440, bits:  6
  [364] = Duosion              mask:  0x404802202, bits:  6
  [365] = Rookidee             mask:  0x284000052, bits:  6
  [366] = Gulpin               mask:  0x260000680, bits:  6
  [367] = Dubwool              mask:  0x204008083, bits:  6
  [368] = Snubbull             mask:  0x204000a81, bits:  6
  [369] = Tadbulb              mask:  0x200081083, bits:  6
  [370] = Trubbish             mask:  0x1c4001001, bits:  6
  [371] = Bisharp              mask:  0x1c2000401, bits:  6
  [372] = Corphish             mask:  0x1c2000044, bits:  6
  [373] = Rapidash             mask:  0x1c0080402, bits:  6
  [374] = Shiftry              mask:  0x1c0001014, bits:  6
  [375] = Reshiram             mask:  0x194080100, bits:  6
  [376] = Fraxure              mask:  0x188080044, bits:  6
  [377] = Shroodle             mask:  0x184002082, bits:  6
  [378] = Marshtomp            mask:  0x182001500, bits:  6
  [379] = Vanillish            mask:  0x144004280, bits:  6
  [380] = Qwilfish             mask:  0x1400080c4, bits:  6
  [381] = Shelgon              mask:  0x132000280, bits:  6
  [382] = Shelmet              mask:  0x114001180, bits:  6
  [383] = Zacian               mask:  0x106010210, bits:  6
  [384] = Shieldon             mask:  0x102000292, bits:  6
  [385] = Sandslash            mask:  0x100080a82, bits:  6
  [386] = Tirtouga             mask:   0xe4003000, bits:  6
  [387] = Grumpig              mask:   0xe4000500, bits:  6
  [388] = Druddigon            mask:   0xe4000202, bits:  6
  [389] = Ursaring             mask:   0xcd000800, bits:  6
  [390] = Rhyperior            mask:   0xca000410, bits:  6
  [391] = Serperior            mask:   0xc8000c10, bits:  6
  [392] = Drilbur              mask:   0xc8000083, bits:  6
  [393] = Kingdra              mask:   0xc5000042, bits:  6
  [394] = Prinplup             mask:   0xc4000680, bits:  6
  [395] = Buneary              mask:   0xc4000211, bits:  6
  [396] = Carbink              mask:   0xc3000041, bits:  6
  [397] = Drizzile             mask:   0xc0110082, bits:  6
  [398] = Beedrill             mask:   0xc0000093, bits:  6
  [399] = Glimmora             mask:   0xa6000180, bits:  6
  [400] = Grotle               mask:   0xa6000082, bits:  6
  [401] = Croagunk             mask:   0xa5800040, bits:  6
  [402] = Zygarde              mask:   0xa2110002, bits:  6
  [403] = Gourgeist            mask:   0xa2101800, bits:  6
  [404] = Groudon              mask:   0xa2020202, bits:  6
  [405] = Greavard             mask:   0xa2004012, bits:  6
  [406] = Grovyle              mask:   0xa0904080, bits:  6
  [407] = Scraggy              mask:   0xa0080850, bits:  6
  [408] = Remoraid             mask:   0x94040102, bits:  6
  [409] = Revavroom            mask:   0x94006100, bits:  6
  [410] = Bibarel              mask:   0x94000091, bits:  6
  [411] = Zekrom               mask:   0x92010140, bits:  6
  [412] = Hattrem              mask:   0x90081108, bits:  6
  [413] = Braviary             mask:   0x90044011, bits:  6
  [414] = Breloom              mask:   0x90002181, bits:  6
  [415] = Kyurem               mask:   0x90002160, bits:  6
  [416] = Skrelp               mask:   0x90000cc0, bits:  6
  [417] = Clefairy             mask:   0x900000d4, bits:  6
  [418] = Vibrava              mask:   0x86104001, bits:  6
  [419] = Malamar              mask:   0x86080180, bits:  6
  [420] = Arboliva             mask:   0x86004081, bits:  6
  [421] = Omastar              mask:   0x86001900, bits:  6
  [422] = Forretress           mask:   0x86001804, bits:  6
  [423] = Snorunt              mask:   0x84801a00, bits:  6
  [424] = Nidorina             mask:   0x84800212, bits:  6
  [425] = Lapras               mask:   0x84080c80, bits:  6
  [426] = Kadabra              mask:   0x84080043, bits:  6
  [427] = Rayquaza             mask:   0x84058040, bits:  6
  [428] = Luxray               mask:   0x840408c0, bits:  6
  [429] = Bronzong             mask:   0x83010201, bits:  6
  [430] = Armaldo              mask:   0x82800182, bits:  6
  [431] = Boldore              mask:   0x82800083, bits:  6
  [432] = Ivysaur              mask:   0x82104810, bits:  6
  [433] = Koraidon             mask:   0x82100242, bits:  6
  [434] = Froslass             mask:   0x82080884, bits:  6
  [435] = Walrein              mask:   0x82048280, bits:  6
  [436] = Wailord              mask:   0x82048082, bits:  6
  [437] = Lombre               mask:   0x82040181, bits:  6
  [438] = Voltorb              mask:   0x82005081, bits:  6
  [439] = Kleavor              mask:   0x820040d0, bits:  6
  [440] = Staryu               mask:   0x82003820, bits:  6
  [441] = Starmie              mask:   0x82001910, bits:  6
  [442] = Crawdaunt            mask:   0x82001242, bits:  6
  [443] = Pawniard             mask:   0x82000622, bits:  6
  [444] = Crobat               mask:   0x80881041, bits:  6
  [445] = Probopass            mask:   0x80880c01, bits:  6
  [446] = Nidoran♂             mask:   0x80880212, bits:  6
  [447] = Drakloak             mask:   0x808800c2, bits:  6
  [448] = Heatran              mask:   0x80081218, bits:  6
  [449] = Wingull              mask:   0x65008080, bits:  6
  [450] = Zigzagoon            mask:   0x64012200, bits:  6
  [451] = Lilligant            mask:   0x64001280, bits:  6
  [452] = Glimmet              mask:   0x64001180, bits:  6
  [453] = Diglett              mask:   0x64001082, bits:  6
  [454] = Blipbug              mask:   0x64000481, bits:  6
  [455] = Pignite              mask:   0x60101600, bits:  6
  [456] = Gimmighoul           mask:   0x60002190, bits:  6
  [457] = Petilil              mask:   0x54000482, bits:  6
  [458] = Fennekin             mask:   0x54000244, bits:  6
  [459] = Eternatus            mask:   0x4c001a00, bits:  6
  [460] = Enamorus             mask:   0x48080b00, bits:  6
  [461] = Cinderace            mask:   0x48040a02, bits:  6
  [462] = Infernape            mask:   0x48040604, bits:  6
  [463] = Whismur              mask:   0x48008908, bits:  6
  [464] = Milcery              mask:   0x48000990, bits:  6
  [465] = Nincada              mask:   0x46000242, bits:  6
  [466] = Lickitung            mask:   0x450010c0, bits:  6
  [467] = Sinistea             mask:   0x44001a10, bits:  6
  [468] = Blitzle              mask:   0x44001881, bits:  6
  [469] = Spinda               mask:   0x44000e02, bits:  6
  [470] = Dipplin              mask:   0x44000682, bits:  6
  [471] = Mudkip               mask:   0x44000542, bits:  6
  [472] = Nymble               mask:   0x44000381, bits:  6
  [473] = Ambipom              mask:   0x42080501, bits:  6
  [474] = Vivillon             mask:   0x42004230, bits:  6
  [475] = Tympole              mask:   0x42001580, bits:  6
  [476] = Deoxys               mask:   0x42000852, bits:  6
  [477] = Slowking             mask:   0x418008c0, bits:  6
  [478] = Slaking              mask:   0x410808c0, bits:  6
  [479] = Smoliv               mask:   0x40804980, bits:  6
  [480] = Victini              mask:   0x40005250, bits:  6
  [481] = Steelix              mask:   0x400018d0, bits:  6
  [482] = Purugly              mask:   0x2c000490, bits:  6
  [483] = Ogerpon              mask:   0x2a800600, bits:  6
  [484] = Pangoro              mask:   0x29880400, bits:  6
  [485] = Bergmite             mask:   0x28101101, bits:  6
  [486] = Golduck              mask:   0x260000c2, bits:  6
  [487] = Dialga               mask:   0x26000092, bits:  6
  [488] = Tangela              mask:   0x25081080, bits:  6
  [489] = Amoonguss            mask:   0x25002900, bits:  6
  [490] = Foongus              mask:   0x25002804, bits:  6
  [491] = Meganium             mask:   0x24040310, bits:  6
  [492] = Flygon               mask:   0x22100284, bits:  6
  [493] = Golbat               mask:   0x22081081, bits:  6
  [494] = Goldeen              mask:   0x22000292, bits:  6
  [495] = Zangoose             mask:   0x21092800, bits:  6
  [496] = Glameow              mask:   0x200a0190, bits:  6
  [497] = Delibird             mask:   0x18000093, bits:  6
  [498] = Metapod              mask:   0x16000502, bits:  6
  [499] = Meloetta             mask:   0x14800182, bits:  6
  [500] = Meditite             mask:   0x14101102, bits:  6
  [501] = Unfezant             mask:   0x14011204, bits:  6
  [502] = Dwebble              mask:   0x14008083, bits:  6
  [503] = Celebi               mask:   0x14000891, bits:  6
  [504] = Beldum               mask:   0x14000183, bits:  6
  [505] = Ledyba               mask:   0x14000093, bits:  6
  [506] = Yveltal              mask:   0x12005090, bits:  6
  [507] = Espeon               mask:   0x12000e10, bits:  6
  [508] = Kecleon              mask:   0x120002d0, bits:  6
  [509] = Ledian               mask:   0x12000292, bits:  6
  [510] = Venonat              mask:   0x10885200, bits:  6
  [511] = Keldeo               mask:   0x108000d2, bits:  6
  [512] = Sceptile             mask:   0x10101c80, bits:  6
  [513] = Quaxwell             mask:   0x100888c0, bits:  6
  [514] = Beheeyem             mask:   0x10000139, bits:  6
  [515] = Sunkern              mask:    0xd000a40, bits:  6
  [516] = Seviper              mask:    0xc104c00, bits:  6
  [517] = Wormadam             mask:    0xc088102, bits:  6
  [518] = Xerneas              mask:    0xc010a10, bits:  6
  [519] = Wurmple              mask:    0xc008580, bits:  6
  [520] = Ursaluna             mask:    0xc002a80, bits:  6
  [521] = Slurpuff             mask:    0xc000c84, bits:  6
  [522] = Monferno             mask:    0xa800304, bits:  6
  [523] = Samurott             mask:    0xa081900, bits:  6
  [524] = Poliwhirl            mask:    0xa008490, bits:  6
  [525] = Ferroseed            mask:    0xa000816, bits:  6
  [526] = Cloyster             mask:    0x84018c0, bits:  6
  [527] = Stantler             mask:    0x8081a80, bits:  6
  [528] = Hatterene            mask:    0x8081218, bits:  6
  [529] = Clamperl             mask:    0x80805c0, bits:  6
  [530] = Caterpie             mask:    0x8080452, bits:  6
  [531] = Sneasler             mask:    0x8010a90, bits:  6
  [532] = Omanyte              mask:    0x6101300, bits:  6
  [533] = Sawsbuck             mask:    0x6010841, bits:  6
  [534] = Sewaddle             mask:    0x6008882, bits:  6
  [535] = Cetoddle             mask:    0x6001882, bits:  6
  [536] = Dustox               mask:    0x6001842, bits:  6
  [537] = Lopunny              mask:    0x6000690, bits:  6
  [538] = Palkia               mask:    0x60004d0, bits:  6
  [539] = Stunky               mask:    0x5001850, bits:  6
  [540] = Tinkaton             mask:    0x5001250, bits:  6
  [541] = Tinkatuff            mask:    0x5001054, bits:  6
  [542] = Tynamo               mask:    0x4901300, bits:  6
  [543] = Totodile             mask:    0x4901082, bits:  6
  [544] = Lunatone             mask:    0x4803280, bits:  6
  [545] = Kabuto               mask:    0x4803041, bits:  6
  [546] = Floatzel             mask:    0x4801884, bits:  6
  [547] = Luxio                mask:    0x48008d0, bits:  6
  [548] = Sableye              mask:    0x4140881, bits:  6
  [549] = Annihilape           mask:    0x4140680, bits:  6
  [550] = Zweilous             mask:    0x4114880, bits:  6
  [551] = Swinub               mask:    0x4108a01, bits:  6
  [552] = Binacle              mask:    0x41002c1, bits:  6
  [553] = Alakazam             mask:    0x40901c0, bits:  6
  [554] = Appletun             mask:    0x4081680, bits:  6
  [555] = Manaphy              mask:    0x4080314, bits:  6
  [556] = Huntail              mask:    0x4041288, bits:  6
  [557] = Buizel               mask:    0x4018091, bits:  6
  [558] = Sneasel              mask:    0x4010a90, bits:  6
  [559] = Nuzleaf              mask:    0x4010294, bits:  6
  [560] = Skwovet              mask:    0x400d840, bits:  6
  [561] = Pumpkaboo            mask:    0x4002541, bits:  6
  [562] = Combusken            mask:    0x4000b41, bits:  6
  [563] = Walking Wake         mask:    0x3048050, bits:  6
  [564] = Latios               mask:    0x2801890, bits:  6
  [565] = Ponyta               mask:    0x2801610, bits:  6
  [566] = Bonsly               mask:    0x2100a81, bits:  6
  [567] = Diancie              mask:    0x2100a12, bits:  6
  [568] = Yamask               mask:    0x2080960, bits:  6
  [569] = Noctowl              mask:    0x20212c0, bits:  6
  [570] = Swablu               mask:    0x200a881, bits:  6
  [571] = Nosepass             mask:     0x890e00, bits:  6
  [572] = Zapdos               mask:     0x890c02, bits:  6
  [573] = Nidoqueen            mask:     0x808252, bits:  6
  [574] = Volbeat              mask:     0x805091, bits:  6
  [575] = Cubone               mask:     0x802261, bits:  6
  [576] = Ludicolo             mask:     0x8020d2, bits:  6
  [577] = Blastoise            mask:     0x481881, bits:  6
  [578] = Mantyke              mask:     0x181340, bits:  6
  [579] = Sandile              mask:     0x180a82, bits:  6
  [580] = Ninetales            mask:     0x151280, bits:  6
  [581] = Vileplume            mask:     0x106580, bits:  6
  [582] = Swoobat              mask:      0x8b801, bits:  6
  [583] = Quaxly               mask:      0x888d0, bits:  6
  [584] = Houndoom             mask:      0x2230a, bits:  6
  [585] = Suicune              mask:       0xaa50, bits:  6
  [586] = Gigalith             mask: 0x2064000080, bits:  5
  [587] = Thievul              mask: 0x2004004090, bits:  5
  [588] = Thwackey             mask: 0x2000088050, bits:  5
  [589] = Jirachi              mask: 0x180a000010, bits:  5
  [590] = Cherrim              mask: 0x10d0000100, bits:  5
  [591] = Chimchar             mask: 0x10c2000100, bits:  5
  [592] = Barboach             mask: 0x1082800001, bits:  5
  [593] = Sinistcha            mask: 0x1044000a00, bits:  5
  [594] = Cherubi              mask: 0x1008002011, bits:  5
  [595] = Machop               mask: 0x1006000500, bits:  5
  [596] = Machoke              mask: 0x1004800140, bits:  5
  [597] = Smoochum             mask: 0x1004002900, bits:  5
  [598] = Chewtle              mask: 0x1004002082, bits:  5
  [599] = Cubchoo              mask: 0x1004002041, bits:  5
  [600] = Chansey              mask: 0x1000080a10, bits:  5
  [601] = Regice               mask:  0x890100800, bits:  5
  [602] = Jynx                 mask:  0x841000840, bits:  5
  [603] = Honedge              mask:  0x810800208, bits:  5
  [604] = Geodude              mask:  0x800802012, bits:  5
  [605] = Persian              mask:  0x40c000600, bits:  5
  [606] = Shroomish            mask:  0x1c0002100, bits:  5
  [607] = Frillish             mask:  0x1c0000084, bits:  5
  [608] = Shaymin              mask:  0x140040300, bits:  5
  [609] = Shellder             mask:  0x118000082, bits:  5
  [610] = Shellos              mask:  0x112000880, bits:  5
  [611] = Urshifu              mask:  0x108002014, bits:  5
  [612] = Oshawott             mask:  0x106009000, bits:  5
  [613] = Shinx                mask:  0x101000850, bits:  5
  [614] = Mienshao             mask:  0x100020310, bits:  5
  [615] = Scizor               mask:   0xc2010800, bits:  5
  [616] = Deerling             mask:   0xc1000082, bits:  5
  [617] = Gengar               mask:   0xb3000000, bits:  5
  [618] = Morgrem              mask:   0xb2000100, bits:  5
  [619] = Grimer               mask:   0xa8100100, bits:  5
  [620] = Grafaiai             mask:   0xa4100004, bits:  5
  [621] = Goodra               mask:   0xa4002002, bits:  5
  [622] = Tyrogue              mask:   0xa0901000, bits:  5
  [623] = Grookey              mask:   0xa0002050, bits:  5
  [624] = Marill               mask:   0x94000180, bits:  5
  [625] = Drednaw              mask:   0x92000202, bits:  5
  [626] = Emboar               mask:   0x92000101, bits:  5
  [627] = Loudred              mask:   0x90020082, bits:  5
  [628] = Murkrow              mask:   0x88800140, bits:  5
  [629] = Zorua                mask:   0x86012000, bits:  5
  [630] = Mareep               mask:   0x84000510, bits:  5
  [631] = Zoroark              mask:   0x82810040, bits:  5
  [632] = Rotom                mask:   0x82801100, bits:  5
  [633] = Rhyhorn              mask:   0x82100208, bits:  5
  [634] = Zarude               mask:   0x82012002, bits:  5
  [635] = Horsea               mask:   0x82000818, bits:  5
  [636] = Roserade             mask:   0x80850002, bits:  5
  [637] = Froakie              mask:   0x80800054, bits:  5
  [638] = Primeape             mask:   0x80140500, bits:  5
  [639] = Iron Crown           mask:   0x80120240, bits:  5
  [640] = Raboot               mask:   0x80083001, bits:  5
  [641] = Krabby               mask:   0x80080051, bits:  5
  [642] = Drowzee              mask:   0x80030012, bits:  5
  [643] = Tepig                mask:   0x70001400, bits:  5
  [644] = Turtwig              mask:   0x68009000, bits:  5
  [645] = Sliggoo              mask:   0x60002880, bits:  5
  [646] = Pelipper             mask:   0x58000480, bits:  5
  [647] = Kingler              mask:   0x490000c0, bits:  5
  [648] = Timburr              mask:   0x48001101, bits:  5
  [649] = Pinsir               mask:   0x48000e00, bits:  5
  [650] = Flittle              mask:   0x44000086, bits:  5
  [651] = Koffing              mask:   0x43000044, bits:  5
  [652] = Dolliv               mask:   0x42004082, bits:  5
  [653] = Onix                 mask:   0x42000a40, bits:  5
  [654] = Finneon              mask:   0x42000214, bits:  5
  [655] = Weezing              mask:   0x41018010, bits:  5
  [656] = Seaking              mask:   0x41000850, bits:  5
  [657] = Litleo               mask:   0x40801090, bits:  5
  [658] = Hypno                mask:   0x40800608, bits:  5
  [659] = Skiddo               mask:   0x40002842, bits:  5
  [660] = Golurk               mask:   0x2a0000c0, bits:  5
  [661] = Kyogre               mask:   0x28900040, bits:  5
  [662] = Gabite               mask:   0x24101001, bits:  5
  [663] = Lugia                mask:   0x24002090, bits:  5
  [664] = Dewgong              mask:   0x23002002, bits:  5
  [665] = Bagon                mask:   0x22040201, bits:  5
  [666] = Okidogi              mask:   0x20800052, bits:  5
  [667] = Dedenne              mask:   0x14040202, bits:  5
  [668] = Ekans                mask:   0x14010240, bits:  5
  [669] = Hatenna              mask:   0x14001208, bits:  5
  [670] = Swellow              mask:   0x10808880, bits:  5
  [671] = Floette              mask:   0x10801084, bits:  5
  [672] = Azelf                mask:   0x10090084, bits:  5
  [673] = Banette              mask:   0x10041201, bits:  5
  [674] = Kirlia               mask:    0xc0000d0, bits:  5
  [675] = Haunter              mask:    0xa001208, bits:  5
  [676] = Dottler              mask:    0xa001082, bits:  5
  [677] = Snover               mask:    0x8804a00, bits:  5
  [678] = Purrloin             mask:    0x8400680, bits:  5
  [679] = Servine              mask:    0x8104a00, bits:  5
  [680] = Liepard              mask:    0x8100482, bits:  5
  [681] = Durant               mask:    0x8081202, bits:  5
  [682] = Yamper               mask:    0x8080520, bits:  5
  [683] = Wailmer              mask:    0x8048180, bits:  5
  [684] = Houndour             mask:    0x802020a, bits:  5
  [685] = Pawmot               mask:    0x6001500, bits:  5
  [686] = Metang               mask:    0x5081100, bits:  5
  [687] = Vullaby              mask:    0x4104081, bits:  5
  [688] = Cetitan              mask:    0x4101a00, bits:  5
  [689] = Psyduck              mask:    0x4100842, bits:  5
  [690] = Doublade             mask:    0x4040083, bits:  5
  [691] = Veluza               mask:    0x4016080, bits:  5
  [692] = Weedle               mask:    0x4008092, bits:  5
  [693] = Pidove               mask:    0x4004412, bits:  5
  [694] = Spewpa               mask:    0x4002c20, bits:  5
  [695] = Kubfu                mask:    0x4002045, bits:  5
  [696] = Happiny              mask:    0x4000618, bits:  5
  [697] = Baltoy               mask:    0x2401081, bits:  5
  [698] = Wynaut               mask:    0x2109200, bits:  5
  [699] = Mawile               mask:    0x2108180, bits:  5
  [700] = Spidops              mask:    0x2100c02, bits:  5
  [701] = Donphan              mask:    0x2080206, bits:  5
  [702] = Claydol              mask:    0x20400c2, bits:  5
  [703] = Dewott               mask:    0x200b002, bits:  5
  [704] = Spoink               mask:    0x1400c40, bits:  5
  [705] = Ting-Lu              mask:    0x1003090, bits:  5
  [706] = Pineco               mask:     0x900640, bits:  5
  [707] = Fuecoco              mask:     0x848044, bits:  5
  [708] = Phione               mask:     0x840214, bits:  5
  [709] = Sudowoodo            mask:     0x80a802, bits:  5
  [710] = Combee               mask:     0x800151, bits:  5
  [711] = Noibat               mask:     0x481201, bits:  5
  [712] = Weavile              mask:     0x10c090, bits:  5
  [713] = Snivy                mask:     0x104a10, bits:  5
  [714] = Eiscue               mask:     0x102860, bits:  5
  [715] = Zubat                mask:      0x93001, bits:  5
  [716] = Leavanny             mask:      0x84290, bits:  5
  [717] = Mantine              mask:      0x81310, bits:  5
  [718] = Feebas               mask:      0x80815, bits:  5
  [719] = Phanpy               mask:      0x80614, bits:  5
  [720] = Bayleef              mask:      0x40095, bits:  5
  [721] = Mienfoo              mask:       0x2314, bits:  5
  [722] = Bidoof               mask:       0x2017, bits:  5
  [723] = Mothim               mask: 0x2042000100, bits:  4
  [724] = Scyther              mask: 0x2008100800, bits:  4
  [725] = Meowth               mask: 0x2000020110, bits:  4
  [726] = Raichu               mask: 0x1080102000, bits:  4
  [727] = Chinchou             mask: 0x1040020200, bits:  4
  [728] = Watchog              mask: 0x1022008000, bits:  4
  [729] = Oddish               mask:  0x142000002, bits:  4
  [730] = Shuppet              mask:  0x104001400, bits:  4
  [731] = Shuckle              mask:  0x1040000c0, bits:  4
  [732] = Fearow               mask:   0xc0800004, bits:  4
  [733] = Wyrdeer              mask:   0xc0008002, bits:  4
  [734] = Furfrou              mask:   0x88002004, bits:  4
  [735] = Pyroar               mask:   0x82100400, bits:  4
  [736] = Furret               mask:    0xc001004, bits:  4
  [737] = Noivern              mask:    0x8404200, bits:  4
  [738] = Wooper               mask:    0x800a400, bits:  4
  [739] = Burmy                mask:    0x8000111, bits:  4
  [740] = Hoopa                mask:    0x4002408, bits:  4
  [741] = Aipom                mask:    0x2040500, bits:  4
  [742] = Deino                mask:     0x900202, bits:  4
  [743] = Fidough              mask:     0x900006, bits:  4
  [744] = Taillow              mask:     0x841080, bits:  4
  [745] = Dondozo              mask:     0x810202, bits:  4
  [746] = Flaaffy              mask:      0x80094, bits:  4
  [747] = Mewtwo               mask:       0x3120, bits:  4
  [748] = Spheal               mask:        0x894, bits:  4

38 phonemes represented:
  b: 121 Pokémon
  d: 198 Pokémon
  f:  88 Pokémon
  h:  37 Pokémon
  i: 218 Pokémon
  j:  29 Pokémon
  k: 235 Pokémon
  l: 314 Pokémon
  m: 175 Pokémon
  n: 272 Pokémon
  p: 139 Pokémon
  s: 256 Pokémon
  t: 228 Pokémon
  u:  98 Pokémon
  v:  65 Pokémon
  w:  69 Pokémon
  z:  57 Pokémon
  Ø:  24 Pokémon
  ä:  51 Pokémon
  æ: 162 Pokémon
  ï:  97 Pokémon
  ð:   1 Pokémon
  õ:   8 Pokémon
  ö: 119 Pokémon
  ŋ:  47 Pokémon
  ɑ: 272 Pokémon
  ə: 340 Pokémon
  ɚ: 116 Pokémon
  ɛ: 137 Pokémon
  ɡ: 134 Pokémon
  ɪ: 209 Pokémon
  ɹ: 305 Pokémon
  ʃ:  38 Pokémon
  ʊ:  12 Pokémon
  ʒ:   5 Pokémon
  ʤ:  39 Pokémon
  ʧ:  51 Pokémon
  θ:  23 Pokémon

Memory usage before running solver: 152028 now, 151876 peak

Finding a solution...
 Solution #140 (9 Pokémon): Slither Wing, Typhlosion, Spoink, Granbull, Venomoth, Houndoom, Exeggutor, Shaymin, Jirachi

Done, tried 6566136 candidates, 63 peak queue length, 102 peak solver length, 468910 cache entries
Memory usage after running solver: 7486892 now, 7493636 peak
./target/release/mon-pangrams   0.99s user 0.01s system 98% cpu 1.012 total
```

</details>

## Changes from the original TypeScript program

- The TypeScript version cleans up the IPA phonemes, then removes any character which is not on an
  allow list. If there were unknown characters, it keeps running.

  This program cleans up the IPA phonemes, removes known-wrong characters. If there are any unknown
  characters, it `panic`s.

- This program automatically skips any potential solution which isn't better than the last best
  solution, so you'll get at most one answer for each length.

- This program uses a `u64` bitmask to indicate which phonemes were represented, rather than a `Set`
  of one-codepoint strings.

  This makes things faster, and use less memory.

- This program uses `u64` cache key of `bitmask | (path.len() << 38)`, rather than a `String` of
  UTF-16 codepoints for each Pokémon ID visited.

  This introduces a bunch of collisions to exclude equivalent paths: if we have a set of phoenemes
  represented (which we already track as a `u64`) for a given path length, then another set of
  Pokémon of the same length that achieves the same coverage is equivalent and not worth exploring.

  This makes things **much** faster, and use **a lot** less memory.

- This program considers all phonemes used by exactly 1 Pokémon as part of the initial solution,
  rather than going into a solve loop for each of them individually.

  eg: In the set of Gen 5 Pokémon, `ʒ` and `ɔɪ` (`õ`) are each in one Pokémon. This program's
  initial solution includes both Duosion and Purrloin, rather than trying an empty initial solution
  and building a solution with each of them in the first position.

[0]: https://graham.build/s/a-blog/034-pokemon-gen-1-phonetic-pangram/
[1]: https://codeberg.org/anvilfood/pokemon-phonetic-pangrams/
[2]: https://rust-lang.org/tools/install/
