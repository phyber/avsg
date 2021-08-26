// savedata: Save data structs and impl methods.
use serde::Deserialize;
use std::collections::{
    HashMap,
    HashSet,
};
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

// Helper type for the SaveData structs
type SerializableDictionary = HashMap<String, String>;

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

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
pub enum Creature {
    // Fauna
    Arachnoptopus,
    Artichoker,
    Blite,
    Blurst,
    BlurstSpawn,
    Buoyg,
    Drometon,
    EyeCopter,
    FlynnStone,
    Fungine,
    Furglot,
    Gill,
    Glugg,
    Hookfish,
    Jorm,
    Jormite,
    LoopDiatom,
    Mogra,
    Mutant,
    Pliaa,
    Potato,
    Prongfish,
    Quadropus,
    Rugg,
    Scorpiant,
    Seamk,
    SmallMogra,
    Snailborg,
    SpaceBat,
    Spidler,
    Spiru,
    SpitBug,
    SpitBugBossSpawn,
    SwarmilyChild,
    SwarmilyParent,
    TrapClaw,
    TubeWorm,
    Volg,
    Yorchug,

    // TubePuff is known by two names.
    #[serde(alias = "TubeWorm_Meta")]
    TubePuff,

    #[serde(rename = "LoopDiatom_Violet")]
    LoopDiatomViolet,

    #[serde(rename = "Mutant_Strong")]
    MutantStrong,

    #[serde(rename = "Rugg_Meta")]
    RuggMeta,

    #[serde(rename = "Snailborg_Meta")]
    SnailborgMeta,

    #[serde(rename = "TrapClaw_Gamma")]
    TrapClawGamma,

    #[serde(rename = "TrapClaw_Meta")]
    TrapClawMeta,

    // Flora
    // DragWeed
    Goolumn,
    Hoverling,
    MushroomPoof,
    SpungusSpore,
    TentacleGrass,
    WillOWisp,

    // Mechanized
    Annihiwaiter,
    Diskko,
    Donaught,
    Hoverbug,
    RepairDrone,
    SentryBot,
    TieFlighter,

    #[serde(rename = "SentryBot_Meta")]
    SentryBotMeta,

    // Other
    Nrok,
    SpitbugNest,
}

impl<'a> Creature {
    // Returns the real name of the creature as listed on
    // https://axiom-verge.fandom.com/wiki/Category:Bestiary
    fn name(&self) -> &str {
        match self {
            // Fauna
            Self::Arachnoptopus    => "Hopping Spider",
            Self::Artichoker       => "Hopping Shrubback",
            Self::Blite            => "Red Wasp",
            Self::Blurst           => "Slug",
            Self::BlurstSpawn      => "Slug Swarm",
            Self::Buoyg            => "Green Glider",
            Self::Drometon         => "Drometon",
            Self::EyeCopter        => "Firefly",
            Self::FlynnStone       => "Cyberdog",
            Self::Fungine          => "Jellyfish",
            Self::Furglot          => "Parasitic Shrub",
            Self::Gill             => "Laser Sea Urchin",
            Self::Glugg            => "Giant Boulderback",
            Self::Hookfish         => "Ancient Tunnel Hopper",
            Self::Jorm             => "Giant Greenworm",
            Self::Jormite          => "Baby Giant Greenworm",
            Self::LoopDiatom       => "Pink Giant Diatom",
            Self::LoopDiatomViolet => "Purple Giant Diatom",
            Self::Mogra            => "Mothmite",
            Self::Mutant           => "Brown Ghoul",
            Self::MutantStrong     => "Gray Ghoul",
            Self::Pliaa            => "Red Flying Krill",
            Self::Potato           => "Pillbug",
            Self::Prongfish        => "Tunnel Hopper",
            Self::Quadropus        => "Sudran Squid",
            Self::Rugg             => "Green Roller",
            Self::RuggMeta         => "Magenta Roller",
            Self::Scorpiant        => "Scorpiant",
            Self::Seamk            => "Purple Flying Krill",
            Self::SmallMogra       => "Baby Mothmite",
            Self::Snailborg        => "Red Nautilus",
            Self::SnailborgMeta    => "Blue Nautilus",
            Self::SpaceBat         => "Space Bat",
            Self::Spidler          => "Carnivorous Silk Bug",
            Self::Spiru            => "Spiru",
            Self::SpitBug          => "Purple Wasp",
            Self::SpitBugBossSpawn => "Ukhu Spawn",
            Self::SwarmilyChild    => "Small Butterfly",
            Self::SwarmilyParent   => "Large Butterfly",
            Self::TrapClaw         => "Purple Scissorbeak",
            Self::TrapClawGamma    => "Cyan Scissorbeak",
            Self::TrapClawMeta     => "Red Scissorbeak",
            Self::TubePuff         => "Green Sea Sponge",
            Self::TubeWorm         => "Yellow Sea Sponge",
            Self::Volg             => "Green Gilk Pupae",
            Self::Yorchug          => "Green Cephalopod",

            // Flora
            // DragWeed
            Self::Goolumn       => "Orb Wall",
            Self::Hoverling     => "Blade Vine",
            Self::MushroomPoof  => "Walking Shrub",
            Self::SpungusSpore  => "Mushroom Spores",
            Self::TentacleGrass => "Poison Grate Plant",
            Self::WillOWisp     => "Will o Wisp",

            // Mechanized
            Self::Annihiwaiter  => "Annihiwaiter",
            Self::Diskko        => "Omni-Sentry",
            Self::Donaught      => "Beholder Sentry",
            Self::Hoverbug      => "Ancient Sentry",
            Self::RepairDrone   => "Repair Drone",
            Self::SentryBot     => "Silver Sentry",
            Self::SentryBotMeta => "Purple Sentry",
            Self::TieFlighter   => "T-Type Sentry",

            // Other
            Self::Nrok        => "Boulder",
            Self::SpitbugNest => "Hive",
        }
    }

    // Returns a Vec of all creatures required for hacker achievement.
    fn achievement_list() -> &'a [Self] { //Vec<Self> {
        //vec![
        &[
            // Fauna
            Self::Arachnoptopus,
            Self::Artichoker,
            Self::Blite,
            Self::Blurst,
            Self::BlurstSpawn,
            Self::Buoyg,
            Self::Drometon,
            Self::EyeCopter,
            Self::FlynnStone,
            Self::Fungine,
            Self::Furglot,
            Self::Gill,
            Self::Glugg,
            Self::Hookfish,
            Self::Jorm,
            Self::LoopDiatom,
            Self::LoopDiatomViolet,
            Self::Mogra,
            Self::Mutant,
            Self::MutantStrong,
            Self::Pliaa,
            Self::Potato,
            Self::Prongfish,
            Self::Quadropus,
            Self::Rugg,
            Self::RuggMeta,
            Self::Scorpiant,
            Self::Seamk,
            Self::SmallMogra,
            Self::Snailborg,
            Self::SnailborgMeta,
            Self::SpaceBat,
            Self::Spidler,
            Self::Spiru,
            Self::SpitBug,
            Self::SpitBugBossSpawn,
            Self::SwarmilyChild,
            Self::SwarmilyParent,
            Self::TrapClaw,
            Self::TrapClawGamma,
            Self::TrapClawMeta,
            Self::TubePuff,
            Self::TubeWorm,
            Self::Volg,
            Self::Yorchug,

            // Flora
            Self::Goolumn,
            Self::Hoverling,
            Self::MushroomPoof,
            Self::SpungusSpore,
            Self::WillOWisp,

            // Mechanized
            Self::Annihiwaiter,
            Self::Diskko,
            Self::Donaught,
            Self::Hoverbug,
            Self::SentryBot,
            Self::SentryBotMeta,
            Self::TieFlighter,

            // Other
            Self::Nrok,
            Self::SpitbugNest,
        ]
    }

    // Returns true or false depending if the Fauna is required for the Hacker
    // achievement.
    fn hacker(&self) -> bool {
        // Most creatures are required, just call out creatures that aren't.
        !matches!(
            self,
            Self::Jormite | Self::TentacleGrass | Self::RepairDrone
        )
    }
}

impl fmt::Display for Creature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Deserialize)]
enum RandomizerDifficultySetting {
    #[serde(rename = "DEFAULT")]
    Default,

    #[serde(rename = "ADVANCED")]
    Advanced,

    #[serde(rename = "MASOCHIST")]
    Masochist,

    #[serde(rename = "EASY")]
    Easy,

    #[serde(rename = "NORMAL")]
    Normal,

    #[serde(rename = "HARD")]
    Hard,
}

#[derive(Debug, Deserialize)]
enum THCollisionDirs {
    None,
    Bottom,
    Left,
    Top,
    Right,
}

#[derive(Debug, Deserialize)]
enum THDifficultySetting {
    #[serde(rename = "NORMAL")]
    Normal,

    #[serde(rename = "HARD")]
    Hard,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
enum THItemType {
    #[serde(rename = "GLITCH_BOMB_DROP")]
    GlitchBombDrop,

    #[serde(rename = "HEALTH_NODE")]
    HealthNode,

    #[serde(rename = "HEALTH_NODE_FRAGMENT")]
    HealthNodeFragment,

    #[serde(rename = "HEALTH_DROP")]
    HealthDrop,

    #[serde(rename = "LORE")]
    Lore,

    #[serde(rename = "PERMANENT_UPGRADE")]
    PermanentUpgrade,

    #[serde(rename = "POWER_NODE")]
    PowerNode,

    #[serde(rename = "POWER_NODE_FRAGMENT")]
    PowerNodeFragment,

    #[serde(rename = "RANGE_NODE")]
    RangeNode,

    #[serde(rename = "SIZE_NODE")]
    SizeNode,

    #[serde(rename = "TOOL")]
    Tool,

    #[serde(rename = "WEAPON")]
    Weapon,
}

impl Default for THItemType {
    fn default() -> Self {
        Self::Weapon
    }
}

#[derive(Debug, Deserialize)]
enum THMapScreenSubScreen {
    #[serde(rename = "MAP")]
    Map,

    #[serde(rename = "START")]
    Start,

    #[serde(rename = "INVENTORY")]
    Inventory,

    #[serde(rename = "NOTES")]
    Notes,

    #[serde(rename = "PASSWORD")]
    Password,

    #[serde(rename = "CONSOLE")]
    Console,

    #[serde(rename = "COUNT")]
    Count,
}

#[derive(Debug, Deserialize)]
struct Point {
    #[serde(rename = "X")]
    x: i32,

    #[serde(rename = "Y")]
    y: i32,
}

#[derive(Debug, Deserialize)]
struct THAreaSaveData {
    #[serde(rename = "mAreaName")]
    area_name: String,

    #[serde(rename = "mSeed")]
    seed: i32,

    #[serde(rename = "mScreenCount")]
    screen_count: i32,

    #[serde(rename = "mX")]
    x: f32,

    #[serde(rename = "mY")]
    y: f32,

    #[serde(rename = "mItem")]
    items: Option<HashSet<String>>,
}

#[derive(Debug, Deserialize)]
struct THAutoMapData {
    //MAX_REMINDERS: i32,
    #[serde(rename = "mAreaName")]
    area_name: String,

    #[serde(rename = "mWidthScreens")]
    width_screens: i32,

    #[serde(rename = "mHeightScreens")]
    height_screens: i32,

    #[serde(rename = "mScreenCount")]
    screen_count: i32,

    #[serde(rename = "mCSVData")]
    csv_data: String,

    #[serde(rename = "mData")]
    data: Option<Vec<u32>>,

    #[serde(rename = "Entrance")]
    entrances: Option<Vec<THAutoMapDoor>>,

    #[serde(rename = "Door")]
    doors: Option<Vec<THAutoMapDoor>>,

    #[serde(rename = "Room")]
    rooms: Option<Vec<THAutoMapRoom>>,

    #[serde(rename = "Reminder")]
    reminders: Vec<Point>,
}

#[derive(Debug, Deserialize)]
struct THAutoMapDoor {
    #[serde(rename = "mX")]
    x: i32,

    #[serde(rename = "mY")]
    y: i32,

    #[serde(rename = "mWall")]
    wall: THCollisionDirs,
}

#[derive(Debug, Deserialize)]
struct THAutoMapRoom {
    #[serde(rename = "mX")]
    x: i32,

    #[serde(rename = "mY")]
    y: i32,

    #[serde(rename = "mWidth")]
    width: i32,

    #[serde(rename = "mHeight")]
    height: i32,
}

#[derive(Debug, Deserialize)]
struct THItemRecord {
    #[serde(rename = "mName")]
    name: String,

    #[serde(rename = "mType")]
    type_: THItemType,

    #[serde(rename = "mConsumable")]
    consumable: bool,

    #[serde(rename = "mExcludedFromCount")]
    excluded_from_count: bool,

    #[serde(rename = "mRequiredItem")]
    required_item: Option<String>,
}

#[derive(Debug, Deserialize)]
struct THPasswordSaveEntry {
    #[serde(rename = "mPassword")]
    password: String,

    #[serde(rename = "mEnabled")]
    enabled: bool,
}

#[derive(Debug, Deserialize)]
struct THSecretWorldSaveData {
    #[serde(rename = "mAreaName")]
    area_name: String,

    #[serde(rename = "mSecretWorldName")]
    secret_world_name: String,

    #[serde(rename = "mPrimaryItem")]
    primary_item: String,

    #[serde(rename = "mSecondaryItem")]
    secondary_item: String,
}

#[derive(Debug, Deserialize)]
struct THSpeedrunCheckpoint {
    #[serde(rename = "mName")]
    name: String,

    #[serde(rename = "mFrames")]
    frames: i64,
}

#[derive(Debug, Deserialize)]
struct Vector2 {
    #[serde(rename = "X")]
    x: i32,

    #[serde(rename = "Y")]
    y: i32,
}

// Save data structure
#[derive(Debug, Deserialize)]
pub struct THSaveData {
    #[serde(rename = "mScreenSize")]
    screen_size: i32,

    #[serde(rename = "mPlayerName")]
    player_name: String,

    #[serde(rename = "mDifficulty")]
    difficulty: THDifficultySetting,

    #[serde(rename = "mRandomizerDifficulty")]
    randomizer_difficulty: Option<RandomizerDifficultySetting>,

    #[serde(rename = "mCurrentWeapon")]
    current_weapon: String,

    #[serde(rename = "mPreviousWeapon")]
    previous_weapon: Option<String>,

    #[serde(rename = "mCurrentTool")]
    current_tool: Option<String>,

    #[serde(rename = "mSaveArea")]
    save_area: String,

    #[serde(rename = "mSaveRoom")]
    save_room: String,

    #[serde(rename = "mSaveRoomPos")]
    save_room_pos: Vector2,

    #[serde(rename = "mTotalFrames")]
    total_frames: i64,

    #[serde(rename = "mEffectiveFrames")]
    effective_frames: f64,

    #[serde(rename = "mScreenCount")]
    screen_count: i32,

    #[serde(rename = "mTotalScreenCount")]
    total_screen_count: i32,

    #[serde(rename = "mNumDeaths")]
    num_deaths: i32,

    #[serde(rename = "mRedGooDestroyed")]
    red_goo_destroyed: i32,

    #[serde(rename = "mBricksDestroyed")]
    bricks_destroyed: i32,

    #[serde(rename = "mIsSpeedRun")]
    is_speed_run: bool,

    #[serde(rename = "mIsRandomizer")]
    is_randomizer: Option<bool>,

    #[serde(rename = "mRandomItem")]
    random_item: Option<SerializableDictionary>,

    #[serde(rename = "mUseRealTimers")]
    use_real_timers: bool,

    #[serde(rename = "mLastMapSubScreen")]
    last_map_sub_screen: THMapScreenSubScreen,

    #[serde(rename = "mBaseSeed")]
    base_seed: i32,

    #[serde(rename = "mRandomizerSeed")]
    randomizer_seed: Option<String>,

    #[serde(rename = "mBiofluxVisions")]
    bioflux_visions: bool,

    #[serde(rename = "mHallucinationAmount")]
    hallucination_amount: f32,

    #[serde(rename = "mTranslatePrimordial")]
    translate_primordial: bool,

    #[serde(rename = "mTranslateVykhya")]
    translate_vykhya: bool,

    #[serde(rename = "mJustinBailey")]
    justin_bailey: bool,

    #[serde(rename = "mTraceBlues")]
    trace_blues: Option<bool>,

    #[serde(rename = "mTraceBlack")]
    trace_black: Option<bool>,

    #[serde(rename = "mTraceYellow")]
    trace_yellow: Option<bool>,

    #[serde(rename = "mSecretWindow")]
    secret_window: Option<bool>,

    #[serde(rename = "mHasDrone")]
    has_drone: bool,

    #[serde(rename = "mCheatsUsed")]
    cheats_used: bool,

    #[serde(rename = "QuickSelectWeapon")]
    weapon_quick_select: Vec<String>,

    #[serde(rename = "THItemRecord")]
    items: Vec<THItemRecord>,

    #[serde(rename = "KeyPoint")]
    key_points_completed: Vec<String>,

    #[serde(rename = "PasswordEntry")]
    passwords: Vec<THPasswordSaveEntry>,

    #[serde(rename = "AreaSaveData")]
    area_save_data: Vec<THAreaSaveData>,

    #[serde(rename = "SecretWorldSaveData")]
    secret_world_save_data: Option<Vec<THSecretWorldSaveData>>,

    #[serde(rename = "AutoMap")]
    auto_maps: Vec<THAutoMapData>,

    #[serde(rename = "SpeedrunCheckpoint")]
    speedrun_checkpoints: Option<Vec<THSpeedrunCheckpoint>>,

    #[serde(rename = "CreatureGlitched")]
    creatures_glitched: Option<Vec<Creature>>,
}

impl THSaveData {
    // Helper methods for achievements
    fn boss_state(&self, boss: &str) -> BossState {
        // Boss states are recorded in the speecrun_checkpoints even when we're
        // not on a speedrun. We can use this to check for bossing being
        // killed.
        let state = if let Some(checkpoints) = &self.speedrun_checkpoints {
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
        self.items
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
    fn achievement_all_health(&self) {
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
    fn achievement_all_items(&self) {
        let (current, needed, percent) = self.item_counts();

        println!(
            "  - 100% Items: {}/{} ({:.2}%)",
            current,
            needed,
            percent,
        );
    }

    // 100% Map achievement
    fn achievement_all_map(&self) {
        let needed = self.total_screen_count;
        let current = self.screen_count;
        let percent: f32 = current as f32 / needed as f32 * 100.0;

        println!(
            "  - 100% Map: {}/{} screens ({:.2}%)",
            current, needed, percent,
        );
    }

    // 100% Notes
    fn achievement_all_notes(&self) {
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
    fn achievement_all_power(&self) {
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
    fn achievement_all_tools(&self) {
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
    fn achievement_all_weapons(&self) {
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
    fn achievement_boss(&self, boss: &str) {
        let state = self.boss_state(boss);

        // Vision is actually called Hallucination for the achievement
        let boss = match boss {
            "Vision" => "Hallucination",
            _        => boss,
        };

        println!("  - {}: {}", boss, state);
    }

    // Brick Breaker achievement
    fn achievement_brick_breaker(&self) {
        let needed = ACHIEVEMENT_BRICK_BREAKER;
        let current = self.bricks_destroyed.clamp(0, needed);
        let percent: f32 = current as f32 / needed as f32 * 100.0;

        println!(
            "  - Brick Breaker: {}/{} ({:.2}%)",
            current, needed, percent,
        );
    }

    // Bubble Breaker achievement
    fn achievement_bubble_breaker(&self) {
        let needed = ACHIEVEMENT_BUBBLE_BREAKER;
        let current = self.red_goo_destroyed.clamp(0, needed);
        let percent: f32 = current as f32 / needed as f32 * 100.0;

        println!(
            "  - Bubble Breaker: {}/{} ({:.2}%)",
            current, needed, percent,
        );
    }

    fn achievement_hack(&self) {
        let needed = 1;
        let current = if let Some(glitched) = &self.creatures_glitched {
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

    fn achievement_hacker(&self) {
        let needed = Creature::achievement_list().len();
        let current = if let Some(glitched) = &self.creatures_glitched {
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

    fn achievement_low_percent(&self) {
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

    fn achievement_mostly_invincible(&self) {
        let maximum = 1;
        let current = self.num_deaths;

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

    pub fn achievement_progress(&self) {
        println!("Achievement Progress:");

        self.achievement_all_health();
        self.achievement_all_items();
        self.achievement_all_map();
        self.achievement_all_notes();
        self.achievement_all_power();
        self.achievement_all_tools();
        self.achievement_all_weapons();
        self.achievement_brick_breaker();
        self.achievement_bubble_breaker();
        self.achievement_hack();
        self.achievement_hacker();
        self.achievement_low_percent();
        self.achievement_mostly_invincible();

        for boss in BOSSES {
            self.achievement_boss(boss);
        }
    }

    pub fn hacker_requires(&self) -> Option<Vec<Creature>> {
        if let Some(glitched) = &self.creatures_glitched {
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
