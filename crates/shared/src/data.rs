//! All campaign data sourced from `poe2wiki.net/wiki/Guide:Acts_quick_guide`
//!
//! Inline color tags used in step text:
//!   `[b]...[/b]`  = boss name (crimson red)
//!   `[i]...[/i]`  = item to pick up (green)
//!   `[s]...[/s]`  = stat bonus / reward note (teal)
//!   `[q]...[/q]`  = quest item / barya (purple)
//!   `[z]...[/z]`  = zone name inline (gold)
//!   `[o]...[/o]`  = "once per league" prefix (orange)

pub struct Act {
    pub name: &'static str,
    pub subtitle: &'static str,
    pub zones: Vec<Zone>,
}

pub struct Zone {
    pub name: &'static str,
    pub steps: Vec<&'static str>,
}

#[must_use]
#[allow(clippy::too_many_lines)]
pub fn all_acts() -> Vec<Act> {
    vec![
        // ─── ACT 1 ───────────────────────────────────────────────────────────
        Act {
            name: "Act I",
            subtitle: "Twilight Strand - Ogham Village",
            zones: vec![
                Zone {
                    name: "Twilight Strand",
                    steps: vec![
                        "Kill [b]Bloated Miller[/b] - Use skillpoint - Go to Clearfell Encampment",
                    ],
                },
                Zone {
                    name: "Town (Clearfell Encampment)",
                    steps: vec![
                        "Talk to NPCs - Go to Clearfell",
                    ],
                },
                Zone {
                    name: "Clearfell",
                    steps: vec![
                        "Kill [b]Beira of the Rotten Pack[/b] [s](Cold Resistance bonus)[/s] - Go to Grelwood",
                    ],
                },
                Zone {
                    name: "Grelwood",
                    steps: vec![
                        "Tag WP - Summon Una (near WP) - Go to Red Vale (opposite side)",
                    ],
                },
                Zone {
                    name: "Red Vale",
                    steps: vec![
                        "Activate 3 Obelisks of Rust & Kill [b]The Rust King[/b] - Take [i]Runed Girdle[/i], [i]Runed Guard[/i] and [i]Runed Skull Cap[/i] - Portal to Town",
                    ],
                },
                Zone {
                    name: "Town",
                    steps: vec![
                        "Talk to NPCs for [i]Runed Spikes[/i] - WP to Grelwood",
                    ],
                },
                Zone {
                    name: "Grelwood (Runic Seals)",
                    steps: vec![
                        "Activate 3 [i]Runic Seals[/i] - Portal to Town",
                    ],
                },
                Zone {
                    name: "Town",
                    steps: vec![
                        "Talk to NPCs - Go to Grelwood",
                    ],
                },
                Zone {
                    name: "Grelwood (Grim Tangle)",
                    steps: vec![
                        "Go to [z]Grim Tangle[/z] (center of zone) - Summon Una - Go to Cemetery of the Eternals",
                    ],
                },
                Zone {
                    name: "Cemetery of the Eternals",
                    steps: vec![
                        "Talk to Lachlann - Enter Mausoleum of the Praetor - Kill [b]Draven, the Eternal Praetor[/b] - Take [i]Draven's Memorial Key Piece[/i] - Go to Cemetery",
                        "Enter Tomb of the Consort (opposite WP) - Kill [b]Asinia, the Praetor's Consort[/b] - Take [i]Asinia's Memorial Key Piece[/i] - Go to Cemetery",
                        "Talk to Lachlann - Kill [b]Lachlann of Endless Lament[/b] - Take [i]Count Lachlann's Ring[/i] - Portal to Town",
                    ],
                },
                Zone {
                    name: "Town",
                    steps: vec![
                        "Talk to NPCs - WP to Hunting Grounds",
                    ],
                },
                Zone {
                    name: "Hunting Grounds",
                    steps: vec![
                        "Talk to Delwyn - Kill [b]The Crowbell[/b] [s](2 skillpoints)[/s] - Go to Freythorn",
                    ],
                },
                Zone {
                    name: "Freythorn",
                    steps: vec![
                        "Complete all rituals - Kill [b]The King in the Mists[/b] [s](30 Spirit bonus)[/s] - Talk to Finn - Portal to Town",
                    ],
                },
                Zone {
                    name: "Town",
                    steps: vec![
                        "Talk to NPCs - WP to Hunting Grounds - Go to Ogham Farmlands",
                    ],
                },
                Zone {
                    name: "Ogham Farmlands",
                    steps: vec![
                        "Tag WP - Take [i]Una's Lute[/i] from Una's Lute Box (before farms) - Go to Ogham Village",
                    ],
                },
                Zone {
                    name: "Ogham Village",
                    steps: vec![
                        "[o]Once per league:[/o] Tag WP - Take [i]Smithing Tools[/i] [s](salvage bench)[/s] - Portal to Town - Talk to Renly - Return to Ogham Village",
                        "Kill [b]The Executioner[/b] (end of zone) - Use Lever - Talk to Leitis - Go to Manor Ramparts",
                    ],
                },
                Zone {
                    name: "Manor Ramparts",
                    steps: vec![
                        "Tag WP - Go to Town - Talk to NPCs [s](2 skillpoints)[/s] - WP to Manor Ramparts - Go to Ogham Manor",
                    ],
                },
                Zone {
                    name: "Ogham Manor",
                    steps: vec![
                        "Kill [b]Candlemass, the Living Rite[/b] [s](Life bonus)[/s] - Go down stairs to Arena - Kill [b]Count Geonor[/b] - Portal to Town",
                    ],
                },
                Zone {
                    name: "Town",
                    steps: vec![
                        "Talk to NPCs - Talk to Hooded One - Go to Act 2",
                    ],
                },
            ],
        },

        // ─── ACT 2 ───────────────────────────────────────────────────────────
        Act {
            name: "Act II",
            subtitle: "Vastiri Outskirts - Dreadnought",
            zones: vec![
                Zone {
                    name: "Vastiri Outskirts",
                    steps: vec![
                        "Kill [b]Rathbreaker[/b] - Portal to Town - Talk to Zarka - Go to Ardura Caravan",
                    ],
                },
                Zone {
                    name: "Town (Ardura Caravan)",
                    steps: vec![
                        "Talk to NPCs - Desert Map to Mawdun Quarry",
                    ],
                },
                Zone {
                    name: "Mawdun Quarry",
                    steps: vec![
                        "Go to Mawdun Mine",
                    ],
                },
                Zone {
                    name: "Mawdun Mine",
                    steps: vec![
                        "Tag WP - Kill [b]Rudja, the Dread Engineer[/b] - Open Cage - Talk to Risu - Portal to Town",
                    ],
                },
                Zone {
                    name: "Town",
                    steps: vec![
                        "Talk to NPCs - Desert Map to Halani Gates",
                    ],
                },
                Zone {
                    name: "Halani Gates",
                    steps: vec![
                        "Talk to Asala - Go to Town",
                    ],
                },
                Zone {
                    name: "Town",
                    steps: vec![
                        "Talk to NPCs - Desert Map to Traitor's Passage",
                    ],
                },
                Zone {
                    name: "Traitor's Passage",
                    steps: vec![
                        "Tag WP - Activate [i]Ancient Seal[/i] (middle of zone) - Activate 3 [i]Runic Seals[/i] - Kill [b]Balbala, the Traitor[/b] - Take [q]Balbala's Barya[/q] - Go to Halani Gates",
                    ],
                },
                Zone {
                    name: "Halani Gates",
                    steps: vec![
                        "Tag WP - Summon Asala - Defeat [b]Jamanra, the Risen King[/b] - Go to sandstorm - Portal to Town",
                    ],
                },
                Zone {
                    name: "Town",
                    steps: vec![
                        "Talk to NPCs - Desert Map to Keth",
                    ],
                },
                Zone {
                    name: "Keth",
                    steps: vec![
                        "Kill serpents for [i]Kabala Clan Relic[/i] - Kill [b]Kabala, Constrictor Queen[/b] [s](2 skillpoints)[/s] - Go to Lost City",
                    ],
                },
                Zone {
                    name: "Lost City",
                    steps: vec![
                        "Go to [z]Buried Shrines[/z] - Go to Heart of Keth",
                    ],
                },
                Zone {
                    name: "Heart of Keth",
                    steps: vec![
                        "Kill [b]Azarian, the Forsaken Son[/b] - Talk to Water Goddess - Take [i]Everburning Cinders[/i] - Ignite Water Goddess - Take [i]The Essence of Water[/i] - Portal to Town",
                    ],
                },
                Zone {
                    name: "Town",
                    steps: vec![
                        "Talk to NPCs - Desert Map to Mastodon Badlands",
                    ],
                },
                Zone {
                    name: "Mastodon Badlands",
                    steps: vec![
                        "Go to [z]Lightless Passage[/z] (center of zone) - Go to Well of Souls",
                    ],
                },
                Zone {
                    name: "Well of Souls",
                    steps: vec![
                        "Speak to Lurking Creature [s](Reveal Desecrated mods)[/s] - Speak to Sin - WP to Lightless Passage - Return to Mastodon Badlands",
                    ],
                },
                Zone {
                    name: "Mastodon Badlands",
                    steps: vec![
                        "Go to Bone Pits",
                    ],
                },
                Zone {
                    name: "Bone Pits",
                    steps: vec![
                        "Tag WP - Kill monsters for [i]Sun Clan Relic[/i] - Go to Blackrib Pit - Kill [b]Iktab, the Deathlord[/b] and [b]Ekbab, Ancient Steed[/b] - Take [i]Mastodon Tusks[/i] - Portal to Town",
                    ],
                },
                Zone {
                    name: "Town",
                    steps: vec![
                        "Talk to NPCs - Desert Map to Valley of Titans",
                    ],
                },
                Zone {
                    name: "Valley of Titans",
                    steps: vec![
                        "Tag WP - Activate [i]Medallion[/i] near WP [s](Charm bonus, can be changed)[/s] - Activate 3 Ancient Seals around the zone - Enter Titan Grotto",
                    ],
                },
                Zone {
                    name: "Titan Grotto",
                    steps: vec![
                        "Kill [b]Zalmarath, the Colossus[/b] - Take [i]The Flame Ruby[/i] - Portal to Town",
                    ],
                },
                Zone {
                    name: "Town",
                    steps: vec![
                        "Talk to NPCs for [i]The Horn of the Vastiri[/i] - Desert Map to Traitor's Passage - Go to the front of the caravan and Sound The Horn - Talk to NPCs - Desert Map to Deshar",
                    ],
                },
                Zone {
                    name: "Deshar",
                    steps: vec![
                        "Find Lailuma's Body (Fallen Dekhara) - Take [i]Final Letter[/i] - Kill [b]Hunin, Storm Caller[/b] and [b]Mugin, Frost Bringer[/b] - Take [q]Djinn Barya[/q] - Go to Path of Mourning",
                    ],
                },
                Zone {
                    name: "Path of Mourning",
                    steps: vec![
                        "Go to Spires of Deshar",
                    ],
                },
                Zone {
                    name: "Spires of Deshar",
                    steps: vec![
                        "Tag WP - Activate [i]Sisters of Garukhan[/i] shrine [s](Lightning Resistance boost)[/s] - Kill [b]Tor Gul, the Defiler[/b] - Portal to Town",
                    ],
                },
                Zone {
                    name: "Town",
                    steps: vec![
                        "Talk to NPCs [s](2 skillpoints)[/s] - Desert Map to Dreadnought",
                    ],
                },
                Zone {
                    name: "Dreadnought",
                    steps: vec![
                        "Go to [z]Dreadnought Vanguard[/z] - Tag WP - Kill [b]Jamanra, the Abomination[/b] - Portal to Town",
                    ],
                },
                Zone {
                    name: "Town",
                    steps: vec![
                        "Talk to NPCs - Talk to Asala - Go to Act 3",
                    ],
                },
            ],
        },

        // ─── ACT 3 ───────────────────────────────────────────────────────────
        Act {
            name: "Act III",
            subtitle: "Sandswept Marsh - Black Chambers",
            zones: vec![
                Zone {
                    name: "Sandswept Marsh",
                    steps: vec![
                        "Go to Ziggurat Encampment",
                    ],
                },
                Zone {
                    name: "Town (Ziggurat Encampment)",
                    steps: vec![
                        "Talk to NPCs - Go to Jungle Ruins (top)",
                    ],
                },
                Zone {
                    name: "Jungle Ruins",
                    steps: vec![
                        "Kill [b]Mighty Silverfist[/b] [s](2 skillpoints)[/s] - Go to Venom Crypts (center of zone near WP)",
                    ],
                },
                Zone {
                    name: "Venom Crypts",
                    steps: vec![
                        "Take [i]Corpse-snake Venom[/i] from Corpse (end of zone) - Portal to Town",
                    ],
                },
                Zone {
                    name: "Town",
                    steps: vec![
                        "Talk to Servi [s](Various boosts, cannot be changed)[/s] - WP to Jungle Ruins - Go to Infested Barrens (opposite from WP)",
                    ],
                },
                Zone {
                    name: "Infested Barrens",
                    steps: vec![
                        "Tag WP - Summon Alva - Go to Azak Bog",
                    ],
                },
                Zone {
                    name: "Azak Bog",
                    steps: vec![
                        "Tag WP - Summon Servi - Kill [b]Ignagduk, the Bog Witch[/b] [s](30 Spirit bonus)[/s] - Take [i]Ignagduk's Ghastly Spear[/i] - Portal to Town",
                    ],
                },
                Zone {
                    name: "Town",
                    steps: vec![
                        "Talk to NPCs - WP to Infested Barrens - Go to Chimeral Wetlands (opposite Azak Bog)",
                    ],
                },
                Zone {
                    name: "Chimeral Wetlands",
                    steps: vec![
                        "Tag WP - Kill [b]Xyclucian, the Chimera[/b] - Take [q]Chimeral Inscribed Ultimatum[/q] - Go to Jiquani's Machinarium (within boss arena)",
                    ],
                },
                Zone {
                    name: "Jiquani's Machinarium",
                    steps: vec![
                        "Tag WP - Summon Alva - Take [i]Small Soul Core[/i] - Activate Stone Altar - Take [i]Small Soul Cores[/i] & Activate Stone Altars - Kill [b]Blackjaw, the Remnant[/b] [s](Fire Resistance boost)[/s] - Go to Jiquani's Sanctum",
                    ],
                },
                Zone {
                    name: "Jiquani's Sanctum",
                    steps: vec![
                        "Tag WP - Summon Alva - Find 3 [i]Medium Soul Cores[/i] & Start 2 Generators (corners of zone) - Activate Large Soul Core near Alva - Kill [b]Zicoatl, Warden of the Core[/b] - Take [i]Large Soul Core[/i] - WP to Infested Barrens",
                    ],
                },
                Zone {
                    name: "Infested Barrens",
                    steps: vec![
                        "Activate Stone Altar (near WP) - Go to Matlan Waterways (nearby)",
                    ],
                },
                Zone {
                    name: "Matlan Waterways",
                    steps: vec![
                        "Pull all the [i]Canal Levers[/i] - Pull the large Canal Lever (end of zone) - Portal to Town",
                    ],
                },
                Zone {
                    name: "Town",
                    steps: vec![
                        "Go down the stairs - Talk to Alva - Go to Drowned City",
                    ],
                },
                Zone {
                    name: "Drowned City",
                    steps: vec![
                        "Tag WP - Summon Oswald - Go to Molten Vault",
                    ],
                },
                Zone {
                    name: "Molten Vault",
                    steps: vec![
                        "[o]Once per league:[/o] Tag WP - Use Lever - Pull Sluice Gate Lever - Kill [b]Mektul, the Forgemaster[/b] - Take [i]The Hammer of Kamasa[/i] - Talk to Oswald [s](reforging bench)[/s] - Portal to Town",
                    ],
                },
                Zone {
                    name: "Town",
                    steps: vec![
                        "Talk to NPCs - WP to Drowned City - Go to Apex of Filth",
                    ],
                },
                Zone {
                    name: "Apex of Filth",
                    steps: vec![
                        "Tag WP - Kill [b]Queen of Filth[/b] (end of zone) - Take [i]Temple Door Idol[/i] - Portal to Town",
                    ],
                },
                Zone {
                    name: "Town",
                    steps: vec![
                        "Go down the stairs - Talk to Alva - Open Door - Go to Temple of Kopec",
                    ],
                },
                Zone {
                    name: "Temple of Kopec",
                    steps: vec![
                        "Climb Stairs twice (corner of pyramid) - Kill [b]Ketzuli, High Priest of the Sun[/b] - Summon Alva, Investigate Platform - Enter gateway - Tag WP (behind gateway) - Go down stairs - Go to Utzaal",
                    ],
                },
                Zone {
                    name: "Utzaal",
                    steps: vec![
                        "Tag WP - Kill [b]Viper Napuatzi[/b] - Kill monsters for [i]Sacrificial Heart[/i] - Go to Aggorat",
                    ],
                },
                Zone {
                    name: "Aggorat",
                    steps: vec![
                        "Tag WP - Go to altar - Take [i]Sacrificial Dagger[/i] - Place and stab [i]Sacrificial Heart[/i] [s](2 skillpoints)[/s] - Go to Black Chambers",
                    ],
                },
                Zone {
                    name: "Black Chambers",
                    steps: vec![
                        "Tag WP - Kill [b]Doryani, Royal Thaumaturge[/b] and [b]Doryani's Triumph[/b] - Talk to Doryani - Portal to Town - Talk to Doryani for apocalypse",
                    ],
                },
                Zone {
                    name: "Town",
                    steps: vec![
                        "Talk to NPCs - Talk to Alva - Go to Act 4",
                    ],
                },
            ],
        },

        // ─── ACT 4 ───────────────────────────────────────────────────────────
        Act {
            name: "Act IV",
            subtitle: "Kingsmarch - Heart of the Tribe",
            zones: vec![
                Zone {
                    name: "TIP",
                    steps: vec![
                        "Three out of the first four islands are essential for the main quest, but required islands change between leagues",
                        "One island also holds [b]Matiki[/b], who needs to be saved to access Eye of Hinekora",
                        "Island boss rewards also change between leagues - optional bosses may be skipped",
                    ],
                },
                Zone {
                    name: "Town (Kingsmarch)",
                    steps: vec![
                        "Talk to NPCs for [i]Book Charter[/i] - Talk to Makoru - Sail to Kedge Bay",
                    ],
                },
                Zone {
                    name: "Kedge Bay",
                    steps: vec![
                        "Go to Journey's End",
                    ],
                },
                Zone {
                    name: "Journey's End",
                    steps: vec![
                        "Summon Tujen - Kill [b]Captain Hartlin[/b] (end of zone) - Take [i]Verisium[/i] - Portal to Town",
                    ],
                },
                Zone {
                    name: "Town",
                    steps: vec![
                        "Talk to Dannig for [i]Verisium Spikes[/i] - WP to Journey's End",
                    ],
                },
                Zone {
                    name: "Journey's End",
                    steps: vec![
                        "Talk to Freya - Activate [i]Karui Totems[/i] - Kill [b]Omniphobia, Fear Manifest[/b] - Portal to Town",
                    ],
                },
                Zone {
                    name: "Town",
                    steps: vec![
                        "Talk to Tujen [s](2 skillpoints)[/s] - Talk to Makoru - Sail to Isle of Kin",
                    ],
                },
                Zone {
                    name: "Isle of Kin",
                    steps: vec![
                        "Kill [b]The Blind Beast[/b] (optional) - Go to Volcanic Warrens",
                    ],
                },
                Zone {
                    name: "Volcanic Warrens",
                    steps: vec![
                        "Kill [b]Krutog, Lord of Kin[/b] (end of zone) - Talk to Hooded One - Return to Ship",
                    ],
                },
                Zone {
                    name: "Ship",
                    steps: vec![
                        "Talk to Makoru - Sail to Abandoned Prison",
                    ],
                },
                Zone {
                    name: "Abandoned Prison",
                    steps: vec![
                        "Kill monsters for [i]Chapel Key[/i] - Open Chapel Door - Tag WP - Activate [i]Goddess of Justice[/i] [s](Flask recovery, can be changed)[/s] - Use Levers - Go to Solitary Confinement",
                    ],
                },
                Zone {
                    name: "Solitary Confinement",
                    steps: vec![
                        "Use Levers - Kill [b]The Prisoner[/b] - Talk to Hooded One - Return to Ship",
                    ],
                },
                Zone {
                    name: "Ship",
                    steps: vec![
                        "Talk to Makoru - Sail to Whakapanu Island",
                    ],
                },
                Zone {
                    name: "Whakapanu Island",
                    steps: vec![
                        "Go to Singing Caverns",
                    ],
                },
                Zone {
                    name: "Singing Caverns",
                    steps: vec![
                        "Tag WP - Kill [b]Diamora, Song of Death[/b] - Talk to Hooded One - Return to Ship",
                    ],
                },
                Zone {
                    name: "Ship",
                    steps: vec![
                        "Talk to Makoru - Sail to Shrike Island",
                    ],
                },
                Zone {
                    name: "Shrike Island",
                    steps: vec![
                        "Kill [b]Scourge of the Skies[/b] (end of zone) - Talk to Hooded One - Return to Ship",
                    ],
                },
                Zone {
                    name: "Ship",
                    steps: vec![
                        "Talk to Makoru - Sail to Eye of Hinekora",
                    ],
                },
                Zone {
                    name: "Eye of Hinekora",
                    steps: vec![
                        "Talk to NPCs - Activate Well of Hinekora - Pass 3 tests - Click [i]Pay your Respects[/i] [s](Mana boost)[/s] - Go to Halls of the Dead (further down)",
                    ],
                },
                Zone {
                    name: "Halls of the Dead",
                    steps: vec![
                        "Tag WP - Pass Tawhoa's test [s](Dex/Lightning Resistance)[/s] - Pass Tasalio's test [s](Int/Cold Resistance)[/s] - Pass Ngamahu's test [s](Str/Fire Resistance)[/s] - Defeat [b]Yama The White[/b] - Take [i]Silver Coin[/i] - Go to Trial of the Ancestors",
                    ],
                },
                Zone {
                    name: "Trial of the Ancestors",
                    steps: vec![
                        "Tag WP - Talk to Navali - Take [i]Tattoo of Hinekora[/i] [s](2 skillpoints)[/s] - Return to Ship",
                    ],
                },
                Zone {
                    name: "Ship",
                    steps: vec![
                        "Talk to Makoru - Sail to Arastas",
                    ],
                },
                Zone {
                    name: "Arastas",
                    steps: vec![
                        "Tag WP - Talk to Missionary Lorandis - Enter church, Exit the church, Destroy forcefield - Kill [b]Torvian, Hand of the Saviour[/b] - Go to Excavation",
                    ],
                },
                Zone {
                    name: "Excavation",
                    steps: vec![
                        "Kill [b]Benedictus, First Herald of Utopia[/b] - Enter forge - Talk to Hooded One - Portal to Town",
                    ],
                },
                Zone {
                    name: "Town",
                    steps: vec![
                        "Talk to NPCs - Talk to Rhodri (at the ship) - Sail to Ngakanu",
                    ],
                },
                Zone {
                    name: "Ngakanu",
                    steps: vec![
                        "Go to Heart of the Tribe",
                    ],
                },
                Zone {
                    name: "Heart of the Tribe",
                    steps: vec![
                        "Tag WP - Kill [b]Tavakai, the Chieftain[/b], [b]Tavakai, the Fallen[/b] and [b]Tavakai, the Consumed[/b] - Portal to Town",
                    ],
                },
                Zone {
                    name: "Town",
                    steps: vec![
                        "Talk to NPCs - Talk to Hooded One - Go to Ogham, Vastiri, or Mount Kriar",
                    ],
                },
            ],
        },

        // ─── INTERLUDE ───────────────────────────────────────────────────────
        Act {
            name: "Interlude",
            subtitle: "Ogham - Vastiri - Mount Kriar - Endgame",
            zones: vec![
                Zone {
                    name: "Ogham",
                    steps: vec![
                        "Town (The Refuge) - Talk to NPCs - Go to Scorched Farmlands (bottom exit)",
                        "Scorched Farmlands - Kill [b]Heldra of the Black Pyre[/b] and [b]Isolde of the White Shroud[/b] - Go to Stones of Serle",
                        "Stones of Serle - Activate all Runed Megaliths - Go to center rune - Kill [b]Siora, Blade of the Mists[/b] - Talk to Una - Go to Scorched Farmlands",
                        "Scorched Farmlands - Go to darkness - Go to [z]Blackwood[/z] - Go to Holten",
                        "Holten - Go to Wolvenhold",
                        "Wolvenhold - Kill [b]Oswin, the Dread Warden[/b] [s](2 skillpoints)[/s] - Go to Holten - Go to Holten Estate",
                        "Holten Estate - Tag WP - Go upstairs, then downstairs - Kill [b]Thane Wulfric[/b] and [b]Lady Elswyth[/b] (ground floor) - Portal to Town",
                        "Town - Talk to NPCs - Talk to Hooded One - Go to Vastiri",
                    ],
                },
                Zone {
                    name: "Vastiri",
                    steps: vec![
                        "Town (The Khari Bazaar) - Talk to NPCs - Go to Khari Crossing",
                        "Khari Crossing - Kill [b]Akthi, the Final Sting[/b] and [b]Anundr, the Sandworm[/b] (top right) - Portal to Town",
                        "Town - Talk to Risu [s](2 skillpoints)[/s] - Return to Khari Crossing",
                        "Khari Crossing - Go to Skullmaw Stairway (top left) - Take [i]Molten One's Gift[/i] [s](Life boost)[/s] - Return to Khari Crossing - Go to Pools of Khatal (bottom left)",
                        "Pools of Khatal - Tag WP - Go to Sel Khari Sanctuary",
                        "Sel Khari Sanctuary - Tag WP - Kill [b]Elzarah, the Cobra Lord[/b] - Talk to Asala - Portal to Town",
                        "Town - Talk to NPCs - Go to Khari Crossing - Go to Galai Gates (left)",
                        "Galai Gates - Tag WP - Kill [b]Vornas, the Fell Flame[/b] - Go to Qimah",
                        "Qimah - Tag WP - Activate Seven Pillars [s](Various boosts, can be changed)[/s] - Summon Jado (end of zone) - Go to Qimah Reservoir",
                        "Qimah Reservoir - Tag WP - Kill [b]Azmadi, the Faridun Prince[/b] - Activate [q]Grand Barya[/q] - Talk to Jado - Portal to Town",
                        "Town - Talk to NPCs - Talk to Hooded One - Go to Mount Kriar",
                    ],
                },
                Zone {
                    name: "Mount Kriar",
                    steps: vec![
                        "Town (The Glade) - Talk to NPCs - Go to Ashen Forest",
                        "Ashen Forest - Go to Kriar Village",
                        "Kriar Village - Tag WP - Kill [b]Lythara, the Wayward Spear[/b] [s](40 Spirit boost)[/s] - Go to Glacial Tarn",
                        "Glacial Tarn - Tag WP - Go to Howling Caves",
                        "Howling Caves - Tag WP - Kill [b]The Abominable Yeti[/b] - Take [i]Icy Tusks[/i] - Portal to Town",
                        "Town - Talk to Hilda [s](2 skillpoints)[/s] - WP to Glacial Tarn",
                        "Glacial Tarn - Kill [b]Rakkar, the Frozen Talon[/b] - Go to Kriar Peaks",
                        "Kriar Peaks - Tag WP - Take gift from Elder Madox - Go to Etched Ravine",
                        "Etched Ravine - Tag WP - Kill [b]Stormgore, the Guardian[/b] - Go to Cuachic Vault",
                        "Cuachic Vault - Tag WP - Kill [b]Zelina, Blood Priestess[/b] and [b]Zolin, Blood Priest[/b] - Summon Doryani - Portal to Town",
                    ],
                },
                Zone {
                    name: "Endgame",
                    steps: vec![
                        "WP to Kingsmarch (Act 4 town)",
                        "Kingsmarch - Talk to NPCs - Talk to Hooded One [s](2 skillpoints)[/s] - Travel to Oriath",
                        "The Ziggurat Refuge - Talk to NPCs - Run one map in Ziggurat to unlock maps - Start mapping in hideout",
                    ],
                },
            ],
        },
    ]
}
