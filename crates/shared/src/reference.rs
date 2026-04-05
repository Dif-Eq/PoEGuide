//! Reference information from `poe2wiki.net/wiki/Guide:Acts_quick_guide`
pub struct GearRow {
    pub slot:    &'static str,
    pub item:    &'static str,
    pub benefit: &'static str,
}

pub struct WeaponRow {
    pub weapon_type: &'static str,
    /// Newline-separated if multiple mods
    pub key_mod:     &'static str,
    pub benefit:     &'static str,
}

pub struct TermEntry {
    pub term:        &'static str,
    pub description: &'static str,
}

pub fn unique_gear() -> Vec<GearRow> {
    vec![
        GearRow { slot: "Boots",        item: "Luminous Pace",          benefit: "10% movement speed" },
        GearRow { slot: "Boots",        item: "Wanderlust",             benefit: "20% movement speed" },
        GearRow { slot: "Helmet",       item: "Greymake",               benefit: "Attributes which helps upgrade skillgems and gear" },
        GearRow { slot: "Helmet",       item: "Goldrim",                benefit: "Resistances which reduce elemental damage taken" },
        GearRow { slot: "Body",         item: "Tabula Rasa",            benefit: "Can equip 6 jewels with powerful mods" },
        GearRow { slot: "Gloves",       item: "Northpaw",               benefit: "Damage boost for melee builds" },
        GearRow { slot: "Bow",          item: "Quill Rain",             benefit: "Low level bow for Ranger builds" },
        GearRow { slot: "Wand",         item: "Lifesprig",              benefit: "Low level wand for Sorceress/Witch builds" },
        GearRow { slot: "Talisman",     item: "Amor Mandragora",        benefit: "Low level talisman for Druid builds" },
        GearRow { slot: "Quarterstaff", item: "Pillar of the Caged God", benefit: "Low level quarterstaff for Monk/Titan builds" },
        GearRow { slot: "Spear",        item: "Tyranny's Grip",         benefit: "Low level spear for melee builds" },
        GearRow { slot: "Shield",       item: "Doomgate",               benefit: "Low level shield for melee builds" },
        GearRow { slot: "Amulet",       item: "The Everlasting Gaze",   benefit: "~90% mana regen (like Atziri's Foible)" },
        GearRow { slot: "Amulet",       item: "Rondel of Fragility",    benefit: "Attack speed and damage boost" },
    ]
}

pub fn weapons() -> Vec<WeaponRow> {
    vec![
        WeaponRow {
            weapon_type: "Bow / Crossbow / Quiver",
            key_mod:     "+X to Level of all Projectile Skills",
            benefit:     "Deal significantly more damage with projectile skills",
        },
        WeaponRow {
            weapon_type: "Sword / Quarterstaff",
            key_mod:     "+X to Level of all Attack Skills",
            benefit:     "Deal significantly more attack damage",
        },
        WeaponRow {
            weapon_type: "Wand / Focus",
            key_mod:     "+X to Level of all Spell Skills\n+X to Level of all Cold Spell Skills\n+X to Level of all Lightning Spell Skills",
            benefit:     "Deal significantly more spell damage",
        },
        WeaponRow {
            weapon_type: "Sceptre",
            key_mod:     "+X to Level of all Minion Skills",
            benefit:     "Minions will deal significantly more damage",
        },
    ]
}

pub fn terminology() -> Vec<TermEntry> {
    vec![
        TermEntry {
            term:        "WP",
            description: "Waypoint — allows navigation between zones (different from checkpoints)",
        },
    ]
}
