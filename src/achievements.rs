// savedata: Save data structs and impl methods.
use crate::savedata::{
    Creature,
    THItemType,
    THSaveData,
};
use std::collections::HashSet;
use std::fmt;
use std::iter::FromIterator;
use std::str;

// Achievement requirements
const ACHIEVEMENT_ALL_HEALTH: i32 = 13;
const ACHIEVEMENT_ALL_NOTES: i32 = 28;
const ACHIEVEMENT_ALL_POWER: i32 = 9;
const ACHIEVEMENT_ALL_RANGE: i32 = 4;
const ACHIEVEMENT_ALL_SIZE: i32 = 4;
const ACHIEVEMENT_ALL_TOOLS: i32 = 16;
const ACHIEVEMENT_ALL_WEAPONS: i32 = 20;
const ACHIEVEMENT_BRICK_BREAKER: i32 = 2_000;
const ACHIEVEMENT_BUBBLE_BREAKER: i32 = 2_000;
const FRAGMENTS_PER_NODE: usize = 5;

// Bosses that we can check for in the save file. We can't check for Athetos,
// since the game doesn't save after defeating him.
const BOSSES: &[&str] = &[
    "Xedur",
    "Telal",
    "Uruku",
    "Gir-Tab",
    "Vision",
    "Clone",
    "Ukhu",
    "Sentinel",
];

#[derive(Debug)]
enum BossState {
    Alive,
    Dead,
}

impl From<bool> for BossState {
    fn from(dead: bool) -> Self {
        if dead {
            Self::Dead
        }
        else {
            Self::Alive
        }
    }
}

impl fmt::Display for BossState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let desc = match self {
            Self::Alive => "Alive",
            Self::Dead  => "Dead",
        };

        write!(f, "{}", desc)
    }
}

pub struct Achievements<'a> {
    savedata: &'a THSaveData,
}

impl<'a> Achievements<'a> {
    pub fn new(savedata: &'a THSaveData) -> Self {
        Self {
            savedata: savedata,
        }
    }

    // Helper methods for achievements
    fn boss_state(&self, boss: &str) -> BossState {
        // Boss states are recorded in the speecrun_checkpoints even when we're
        // not on a speedrun. We can use this to check for bossing being
        // killed.
        let checkpoints = &self.savedata.speedrun_checkpoints;
        let state = if let Some(checkpoints) = checkpoints {
            // If we find the boss, it's dead.
            checkpoints
                .iter()
                .filter(|c| c.name == boss)
                .count() > 0
        }
        else {
            false
        };

        state.into()
    }

    fn item_type_count(&self, type_: THItemType) -> usize {
        // Get a count for specific item types. Used to check for various 100%
        // achievements.
        self.savedata.items
            .iter()
            .filter(|item| item.type_ == type_ && !item.excluded_from_count)
            .count()
    }

    fn item_counts(&self) -> (usize, i32, f32) {
        let needed = {
            ACHIEVEMENT_ALL_HEALTH
            + ACHIEVEMENT_ALL_NOTES
            + ACHIEVEMENT_ALL_POWER
            + ACHIEVEMENT_ALL_RANGE
            + ACHIEVEMENT_ALL_SIZE
            + ACHIEVEMENT_ALL_TOOLS
            + ACHIEVEMENT_ALL_WEAPONS
        };

        let health_nodes = self.item_type_count(THItemType::HealthNode);
        let health_frags = self.item_type_count(THItemType::HealthNodeFragment);
        let notes = self.item_type_count(THItemType::Lore);
        let power_nodes = self.item_type_count(THItemType::PowerNode);
        let power_frags = self.item_type_count(THItemType::PowerNodeFragment);
        let range_nodes = self.item_type_count(THItemType::RangeNode);
        let size_nodes = self.item_type_count(THItemType::SizeNode);
        let tools = self.item_type_count(THItemType::Tool);
        let upgrades = self.item_type_count(THItemType::PermanentUpgrade);
        let weapons = self.item_type_count(THItemType::Weapon);

        let current = {
            health_nodes
            + (health_frags / FRAGMENTS_PER_NODE)
            + notes
            + power_nodes
            + (power_frags / FRAGMENTS_PER_NODE)
            + range_nodes
            + size_nodes
            + tools
            + upgrades
            + weapons
        };

        let percent: f32 = current as f32 / needed as f32 * 100.0;

        (current, needed, percent)
    }

    // 100% Health achievement
    // The count here also includes node fragments
    fn all_health(&self) {
        let needed = ACHIEVEMENT_ALL_HEALTH;
        let nodes = self.item_type_count(THItemType::HealthNode);
        let frags = self.item_type_count(THItemType::HealthNodeFragment);
        let frags = frags / FRAGMENTS_PER_NODE;
        let current = nodes + frags;
        let percent: f32 = current as f32 / needed as f32 * 100.0;

        println!(
            "  - 100% Health: {}/{} ({:.2}%)",
            current,
            needed,
            percent,
        );
    }

    // 100% Items
    fn all_items(&self) {
        let (current, needed, percent) = self.item_counts();

        println!(
            "  - 100% Items: {}/{} ({:.2}%)",
            current,
            needed,
            percent,
        );
    }

    // 100% Map achievement
    fn all_map(&self) {
        let needed = self.savedata.total_screen_count;
        let current = self.savedata.screen_count;
        let percent: f32 = current as f32 / needed as f32 * 100.0;

        println!(
            "  - 100% Map: {}/{} screens ({:.2}%)",
            current, needed, percent,
        );
    }

    // 100% Notes
    fn all_notes(&self) {
        let needed = ACHIEVEMENT_ALL_NOTES;
        let current = self.item_type_count(THItemType::Lore);
        let percent: f32 = current as f32 / needed as f32 * 100.0;

        println!(
            "  - 100% Notes: {}/{} ({:.2}%)",
            current,
            needed,
            percent,
        );
    }

    // 100% Power
    fn all_power(&self) {
        let needed = ACHIEVEMENT_ALL_POWER;
        let nodes = self.item_type_count(THItemType::PowerNode);
        let frags = self.item_type_count(THItemType::PowerNodeFragment);
        let frags = frags / FRAGMENTS_PER_NODE;
        let current = nodes + frags;
        let percent: f32 = current as f32 / needed as f32 * 100.0;

        println!(
            "  - 100% Power: {}/{} ({:.2}%)",
            current,
            needed,
            percent,
        );
    }

    // 100% Tools
    fn all_tools(&self) {
        let needed = ACHIEVEMENT_ALL_TOOLS;
        let tools = self.item_type_count(THItemType::Tool);
        let upgrades = self.item_type_count(THItemType::PermanentUpgrade);
        let current = tools + upgrades;
        let percent: f32 = current as f32 / needed as f32 * 100.0;

        println!(
            "  - 100% Tools: {}/{} ({:.2}%)",
            current,
            needed,
            percent,
        );
    }

    // 100% Weapons
    fn all_weapons(&self) {
        let needed = ACHIEVEMENT_ALL_WEAPONS;
        let current = self.item_type_count(THItemType::Weapon);
        let percent: f32 = current as f32 / needed as f32 * 100.0;

        println!(
            "  - 100% Weapons: {}/{} ({:.2}%)",
            current,
            needed,
            percent,
        );
    }

    // Boss kill achievements
    fn boss(&self, boss: &str) {
        let state = self.boss_state(boss);

        // Vision is actually called Hallucination for the achievement
        let boss = match boss {
            "Vision" => "Hallucination",
            _        => boss,
        };

        println!("  - {}: {}", boss, state);
    }

    // Brick Breaker achievement
    fn brick_breaker(&self) {
        let needed = ACHIEVEMENT_BRICK_BREAKER;
        let current = self.savedata.bricks_destroyed.clamp(0, needed);
        let percent: f32 = current as f32 / needed as f32 * 100.0;

        println!(
            "  - Brick Breaker: {}/{} ({:.2}%)",
            current, needed, percent,
        );
    }

    // Bubble Breaker achievement
    fn bubble_breaker(&self) {
        let needed = ACHIEVEMENT_BUBBLE_BREAKER;
        let current = self.savedata.red_goo_destroyed.clamp(0, needed);
        let percent: f32 = current as f32 / needed as f32 * 100.0;

        println!(
            "  - Bubble Breaker: {}/{} ({:.2}%)",
            current, needed, percent,
        );
    }

    fn hack(&self) {
        let needed = 1;
        let glitched = &self.savedata.creatures_glitched;
        let current = if let Some(glitched) = glitched {
            glitched.len().clamp(0, needed)
        }
        else {
            0
        };

        let percent: f32 = current as f32 / needed as f32 * 100.0;

        println!(
            "  - Hack: {}/{} ({:.2}%)",
            current, needed, percent,
        );
    }

    fn hacker(&self) {
        let needed = Creature::achievement_list().len();
        let glitched = &self.savedata.creatures_glitched;
        let current = if let Some(glitched) = glitched {
            glitched.len()
        }
        else {
            0
        };

        let percent: f32 = current as f32 / needed as f32 * 100.0;

        println!(
            "  - Hacker: {}/{} ({:.2}%)",
            current, needed, percent,
        );
    }

    fn low_percent(&self) {
        let maximum = 40.0;
        let (current, needed, percent) = self.item_counts();

        let state = if percent >= maximum {
            "Failed"
        }
        else {
            "OK"
        };

        println!(
            "  - Low %: {}/{} ({:.2}%) ({})",
            current, needed, percent, state,
        );
    }

    fn mostly_invincible(&self) {
        let maximum = 1;
        let current = self.savedata.num_deaths;

        let state = if current > maximum {
            "Failed"
        }
        else {
            "OK"
        };

        let maybe_plural = if current == 1 {
            "death"
        }
        else {
            "deaths"
        };

        println!(
            "  - Mostly Invincible: {}/{} {} ({})",
            current, maximum, maybe_plural, state,
        );
    }

    fn pacifist(&self) {
        let state = self.boss_state("Clone");

        let ok  = match state {
            BossState::Alive => "OK",
            BossState::Dead  => "Failed",
        };

        println!("  - Pacifist: Clone {} ({})", state, ok);
    }

    pub fn progress(&self) {
        println!("Achievement Progress:");

        self.all_health();
        self.all_items();
        self.all_map();
        self.all_notes();
        self.all_power();
        self.all_tools();
        self.all_weapons();
        self.brick_breaker();
        self.bubble_breaker();
        self.hack();
        self.hacker();
        self.low_percent();
        self.mostly_invincible();
        self.pacifist();

        for boss in BOSSES {
            self.boss(boss);
        }
    }

    pub fn hacker_requires(&self) -> Option<Vec<Creature>> {
        if let Some(glitched) = &self.savedata.creatures_glitched {
            let needs = Creature::achievement_list();
            let needs: HashSet<&Creature> = HashSet::from_iter(needs);
            let glitched: HashSet<&Creature> = HashSet::from_iter(glitched);
            let difference = needs.difference(&glitched);

            // Get an owned Vec<Creature> from our difference.
            let required: Vec<Creature> = difference
                .into_iter()
                .map(|&&c| c)
                .collect();

            Some(required)
        }
        else {
            None
        }
    }
}
