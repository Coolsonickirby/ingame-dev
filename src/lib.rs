use imgui_api::bindings::*;
use smash::Vector3f;
use smash::lib::lua_const::{FIGHTER_INSTANCE_WORK_ID_INT_COLOR, GROUND_CORRECT_KIND_AIR, SITUATION_KIND_AIR};
use smash_arc::{ArcLookup, SearchLookup};
use std::f32;
use std::ffi::CStr;
use std::ops::{Index, SubAssign, Mul, Add, AddAssign};
use std::str::FromStr;
use skyline::hook;
use skyline::libc::{c_char, c_void, size_t};
use skyline::hooks::InlineCtx;
use std::collections::HashMap;
use nnsdk::nn;
use lua_bind::{
    ControlModule,
    FighterManager::{is_ready_go, is_result_mode},
};
use smash::app::lua_bind::{FighterManager, GroundModule, MotionModule, PostureModule, StatusModule, WorkModule};
use smash::app::{lua_bind::DamageModule, *};
use smash::lua2cpp::L2CFighterCommon;

use cstring_array::CStringArray;

mod offsets;
mod resource;


extern "C" {
    #[link_name = "\u{1}_ZN2nn3hid13GetMouseStateEPNS0_10MouseStateE"]
    pub fn get_mouse_state(
        arg1: *const MouseState
    );
    #[link_name = "\u{1}_ZN2nn3hid16GetKeyboardStateEPNS0_13KeyboardStateE"]
    pub fn get_keyboard_state(
        arg1: *const KeyboardState
    );
    #[link_name = "\u{1}_ZN3app11FighterUtil15is_photo_cameraEv"]
    pub fn is_photo_camera() -> bool;
}

enum MouseButton {
    Left = 0,
    Right = 1,
    Middle = 2,
    Forward = 3,
    Back = 4
}

#[repr(C)]
#[derive(Default)]
struct MouseState {
    pub sampling_number: u64,
    pub x: i32,
    pub y: i32,
    pub delta_x: i32,
    pub delta_y: i32,
    pub wheel_delta_x: i32,
    pub wheel_delta_y: i32,
    pub buttons: u32,
    pub attributes: u32,
}

impl MouseState {
    pub fn get_current_state() -> MouseState {
        let state = MouseState::default();
        unsafe {
            get_mouse_state(&state as _);
        }
        state
    }

    pub fn button_clicked(&self, btn: MouseButton) -> bool {
        self.buttons & (1 << btn as u32) != 0
    }
}

#[repr(C)]
struct KeyboardState {
    pub sampling_number: u64,
    pub modifiers: u64,
    pub keys: [u64; 4]
}

enum KeyboardModifier {
    Control = 0,
    Shift = 1,
    LeftAlt = 2,
    RightAlt = 3,
    Gui = 4,
    CapsLock = 8,
    ScrollLock = 9,
    NumLock = 10,
    Katakana = 11,
    Hiragana = 12,
}

enum KeyboardKey {
    A = 4,
    B = 5,
    C = 6,
    D = 7,
    E = 8,
    F = 9,
    G = 10,
    H = 11,
    I = 12,
    J = 13,
    K = 14,
    L = 15,
    M = 16,
    N = 17,
    O = 18,
    P = 19,
    Q = 20,
    R = 21,
    S = 22,
    T = 23,
    U = 24,
    V = 25,
    W = 26,
    X = 27,
    Y = 28,
    Z = 29,
    D1 = 30,
    D2 = 31,
    D3 = 32,
    D4 = 33,
    D5 = 34,
    D6 = 35,
    D7 = 36,
    D8 = 37,
    D9 = 38,
    D0 = 39,
    Return = 40,
    Escape = 41,
    Backspace = 42,
    Tab = 43,
    Space = 44,
    Minus = 45,
    Plus = 46,
    OpenBracket = 47,
    CloseBracket = 48,
    Pipe = 49,
    Tilde = 50,
    Semicolon = 51,
    Quote = 52,
    Backquote = 53,
    Comma = 54,
    Period = 55,
    Slash = 56,
    CapsLock = 57,
    F1 = 58,
    F2 = 59,
    F3 = 60,
    F4 = 61,
    F5 = 62,
    F6 = 63,
    F7 = 64,
    F8 = 65,
    F9 = 66,
    F10 = 67,
    F11 = 68,
    F12 = 69,
    PrintScreen = 70,
    ScrollLock = 71,
    Pause = 72,
    Insert = 73,
    Home = 74,
    PageUp = 75,
    Delete = 76,
    End = 77,
    PageDown = 78,
    RightArrow = 79,
    LeftArrow = 80,
    DownArrow = 81,
    UpArrow = 82,
    NumLock = 83,
    NumPadDivide = 84,
    NumPadMultiply = 85,
    NumPadSubtract = 86,
    NumPadAdd = 87,
    NumPadEnter = 88,
    NumPad1 = 89,
    NumPad2 = 90,
    NumPad3 = 91,
    NumPad4 = 92,
    NumPad5 = 93,
    NumPad6 = 94,
    NumPad7 = 95,
    NumPad8 = 96,
    NumPad9 = 97,
    NumPad0 = 98,
    NumPadDot = 99,
    Backslash = 100,
    Application = 101,
    Power = 102,
    NumPadEquals = 103,
    F13 = 104,
    F14 = 105,
    F15 = 106,
    F16 = 107,
    F17 = 108,
    F18 = 109,
    F19 = 110,
    F20 = 111,
    F21 = 112,
    F22 = 113,
    F23 = 114,
    F24 = 115,
    NumPadComma = 133,
    Ro = 135,
    KatakanaHiragana = 136,
    Yen = 137,
    Henkan = 138,
    Muhenkan = 139,
    NumPadCommaPc98 = 140,
    HangulEnglish = 144,
    Hanja = 145,
    Katakana = 146,
    Hiragana = 147,
    ZenkakuHankaku = 148,
    LeftControl = 224,
    LeftShift = 225,
    LeftAlt = 226,
    LeftGui = 227,
    RightControl = 228,
    RightShift = 229,
    RightAlt = 230,
    RightGui = 231,
}

impl KeyboardState {
    pub fn get_current_state() -> KeyboardState {
        let state = KeyboardState { sampling_number: 0, modifiers: 0, keys: [0; 4] };
        unsafe {
            get_keyboard_state(&state as _);
        }
        state
    }

    pub fn modifier_pressed(&self, key: KeyboardModifier) -> bool {
        self.modifiers & (1 << key as u64) != 0
    }

    pub fn key_pressed(&self, key: KeyboardKey) -> bool {
        let key = key as usize;
        let storage_bits: usize = 8 * 8;
        (self.keys[key / storage_bits] & (1 << (key % storage_bits))) != 0
    }
}


unsafe extern "C" {
    #[link_name = "\u{1}_ZN3app8lua_bind38FighterManager__get_fighter_entry_implEPNS_14FighterManagerENS_14FighterEntryIDE"]
    fn get_fighter_entry(manager: *mut smash::app::FighterManager, entry_id: u32) -> *mut u8;
}

pub fn get_fighter_common_from_accessor<'a>(
    boma: &'a mut BattleObjectModuleAccessor,
) -> &'a mut L2CFighterCommon {
    unsafe {
        let lua_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x190 / 8);
        std::mem::transmute(*((lua_module + 0x1D8) as *mut *mut L2CFighterCommon))
    }
}

pub fn get_battle_object_from_entry_id(entry_id: u32) -> Option<*mut BattleObject> {
    unsafe {
        let entry = get_fighter_entry(singletons::FighterManager(), entry_id);
        if entry.is_null() {
            None
        } else {
            Some(*(entry.add(0x4160) as *mut *mut BattleObject))
        }
    }
}

pub fn get_fighter_common_from_entry_id(entry_id: u32) -> Option<&'static mut L2CFighterCommon> {
    if let Some(object) = get_battle_object_from_entry_id(entry_id) {
        unsafe {
            Some(get_fighter_common_from_accessor(std::mem::transmute(
                (*object).module_accessor,
            )))
        }
    } else {
        None
    }
}


#[link(name = "imgui_smash")]
unsafe extern "C" {}


pub const SOURCE_DISP_WIDTH: f32 = 1920.0;
pub const SOURCE_DISP_HEIGHT: f32 = 1080.0;

pub fn get_fixed_width(pos: f32, cur_disp_width: f32) -> f32 {
    if(cur_disp_width == SOURCE_DISP_WIDTH){
        pos
    } else {
        let multi = pos / SOURCE_DISP_WIDTH;
        multi * cur_disp_width
    }
}


pub fn get_fixed_height(pos: f32, cur_disp_height: f32) -> f32 {
    if(cur_disp_height == SOURCE_DISP_HEIGHT){
        pos
    } else {
        let multi = pos / SOURCE_DISP_HEIGHT;
        multi * cur_disp_height
    }
}

#[derive(PartialEq, Clone, Debug)]
enum GameState {
    WAITING_FOR_GAME_START = 0,
    GAME_STARTED = 2,
    GAME_IN_PROGRESS = 3,
}

pub static tool_title: &str = "Photo Mode Options\0";

pub fn get_character_name(id: i32) -> &'static str {
    match id {
        0 => "Mario",
        1 => "Donkey Kong",
        2 => "Link",
        3 => "Samus",
        4 => "Dark Samus",
        5 => "Yoshi",
        6 => "Kirby",
        7 => "Fox",
        8 => "Pikachu",
        9 => "Luigi",
        10 => "Ness",
        11 => "Captain Falcon",
        12 => "Jigglypuff",
        13 => "Princess Peach",
        14 => "Princess Daisy",
        15 => "Bowser",
        16 => "Sheik",
        17 => "Zelda",
        18 => "Dr. Mario",
        19 => "Pichu",
        20 => "Falco",
        21 => "Marth",
        22 => "Lucina",
        23 => "Young Link",
        24 => "Ganondorf",
        25 => "Mewtwo",
        26 => "Roy",
        27 => "Chrom",
        28 => "Game & Watch",
        29 => "Meta Knight",
        30 => "Pit",
        31 => "Dark Pit",
        32 => "Zero Suit Samus",
        33 => "Wario",
        34 => "Snake",
        35 => "Ike",
        36 => "Pokemon Trainer", // Squirtle
        37 => "Pokemon Trainer", // Ivysaur
        38 => "Pokemon Trainer", // Charizard
        39 => "Diddy Kong",
        40 => "Lucas",
        41 => "Sonic",
        42 => "King Dedede",
        43 => "Olimar",
        44 => "Lucario",
        45 => "Robot",
        46 => "Toon Link",
        47 => "Wolf",
        48 => "Villager",
        49 => "Megaman",
        50 => "Wii Fit",
        51 => "Rosalina & Luma",
        52 => "Little Mac",
        53 => "Greninja",
        54 => "Palutena",
        55 => "Pac-Man",
        56 => "Robin",
        57 => "Shulk",
        58 => "Bowser Jr.",
        59 => "Duck Hunt",
        60 => "Ryu",
        61 => "Ken",
        62 => "Cloud",
        63 => "Corrin",
        64 => "Bayonetta",
        65 => "Inkling",
        66 => "Ridley",
        67 => "Simon",
        68 => "Richter",
        69 => "King K. Rool",
        70 => "Isabelle",
        71 => "Incineroar",
        72 => "Mii Fighter",
        73 => "Mii Swordsman",
        74 => "Mii Gunner",
        75 => "Ice Climbers", // popo
        76 => "Ice Climbers", // nana
        77 => "Giga Bowser",
        78 => "miienemyf",
        79 => "miienemys",
        80 => "miienemyg",
        81 => "Pirahna Plant",
        82 => "Joker",
        83 => "Hero",
        84 => "Banjo & Kazooie",
        85 => "Terry",
        86 => "Byleth",
        87 => "Min-Min",
        88 => "Steve",
        89 => "Sephiroth",
        90 => "Pyra/Mythra", // Pyra
        91 => "Pyra/Mythra", // Mythra
        92 => "Kazuya",
        93 => "Sora",
        110 => "Ice Climbers",
        111 => "Squirtle",
        112 => "Ivysaur",
        113 => "Charizard",
        114 => "Pokemon Trainer",
        _ => "unknown",
    }
}


lazy_static::lazy_static! {
    pub static ref FIGHTERS: std::sync::Mutex<Vec<FighterInfo>> = std::sync::Mutex::new(Vec::new());
    pub static ref MOTION_LIST_HASHES: std::sync::Mutex<HashMap<u64, String>> = std::sync::Mutex::new(HashMap::new());
    pub static ref FRONT_VEC: std::sync::Mutex<Vec3> = std::sync::Mutex::new(Vec3 {
        x: 0.0,
        y: 0.0,
        z: -1.0
    });
}

pub fn get_label(hash: &u64) -> String {
match MOTION_LIST_HASHES.lock().unwrap().get(hash) {
        Some(res) => res.clone(),
        None => {
            println!("Could not find label for {:#x} !", hash);
            format!("{:#x}", hash)
        }
    }
}

pub fn get_hash(label: String) -> u64 {
    match label.strip_prefix("0x") {
        Some(x) => {
            u64::from_str_radix(x, 16).unwrap()
        },
        None => smash::hash40(&label)
    }
}

#[derive(Default)]
pub struct FighterStrings {
    pub current_fighter_name:  String,
    pub current_animation_frame: String,
}

pub struct FighterInfo {
    pub fighter_kind: i32,
    pub entry_id: i32,
    pub bo: u64,
    pub boma: u64,
    pub common: u64,
    pub color: u8,
    pub gui_strings: FighterStrings,
    pub position: [f32; 3],
    pub is_pos_locked: bool,
    pub scale: f32,
    pub current_motion_idx: i32,
    pub current_motion_hash: u64,
    pub current_frame: f32,
    pub max_frame: f32,
    pub rate: f32,
    pub force_overwrite_rate: bool,
    pub motion_enabled: bool,
    pub all_motions: CStringArray,
    pub rotation: [f32; 3],
    pub header_open: bool
}

impl FighterInfo {
    pub fn create(entry_id: i32) -> Option<FighterInfo> {
        unsafe {
            match get_fighter_common_from_entry_id(entry_id as _) {
                Some(fighter) => {
                    let kind = smash::app::utility::get_kind(&mut *fighter.module_accessor);
                    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
                    let path = format!("fighter/{}/motion/body/c{:02}/motion_list.bin", kind_to_folder_name(kind), color);
                    let mut all_motions: Vec<String> = vec![];
                    let mut data: Option<Vec<u8>> = None;
                    let mut enable_motion = true;

                    match std::fs::read(std::path::PathBuf::from("mods:").join(&path)) {
                        Ok(res) => data = Some(res),
                        Err(err) => {
                            println!("Couldn't read from mods:/ mount (Reason: {:?}), falling back to arc:/", err);
                            match std::fs::read(std::path::PathBuf::from("arc:").join(&path)) {
                                Ok(res) => data = Some(res),
                                Err(err) => {
                                    println!("Couldn't read from arc:/ mount (Reason: {:?}), falling back to LoadedArc", err);
                                    match resource::arc().get_file_contents(smash_arc::hash40(&path), smash_arc::Region::None) {
                                        Ok(res) => data = Some(res),
                                        Err(err) => {
                                            println!("Couldn't read from LoadedArc (Reason: {:?}), disabling motion options", err);
                                        }
                                    }
                                },
                            }
                        },
                    }

                    match data {
                        Some(res) => {
                            let mut reader = std::io::Cursor::new(res);
                            match motion_lib::read_stream(&mut reader) {
                                Ok(motion_list) => {
                                    for key in motion_list.list.keys() {
                                        all_motions.push(get_label(&key.0));
                                    }
                                },
                                Err(err) => {
                                    println!("Failed to parse {} !", path);
                                }
                            }
                        },
                        None => {}
                    };

                    let mut motion_options: CStringArray = CStringArray::new(vec![String::from("Temporary")]).unwrap();
                    let mut motion_enabled = false;
                    match CStringArray::new(all_motions) {
                        Ok(motions) => {
                            motion_options = motions;
                            motion_enabled = true;
                        },
                        Err(err) => println!("Failed to create motion options CStringArray (Reason: {:?})", err),
                    };

                    Some(FighterInfo {
                        fighter_kind: kind,
                        entry_id: entry_id,
                        bo: fighter.battle_object as _,
                        boma: fighter.module_accessor as _,
                        common: (fighter as *mut L2CFighterCommon) as _,
                        current_motion_idx: 0,
                        color: color as _,
                        gui_strings: FighterStrings::default(),
                        position: [0.0, 0.0, 0.0],
                        is_pos_locked: false,
                        scale: 1.0,
                        all_motions: motion_options,
                        current_motion_hash: 0,
                        current_frame: 0.0,
                        max_frame: 0.0,
                        rate: 0.0,
                        force_overwrite_rate: false,
                        motion_enabled: motion_enabled,
                        rotation: [0.0, 0.0, 0.0],
                        header_open: false
                    })
                },
                None => None,
            }
        }
    }

    pub fn get_name(&mut self) -> &String {
        if self.gui_strings.current_fighter_name.is_empty() {
            let name = get_character_name(self.fighter_kind);
            self.gui_strings.current_fighter_name = format!("Entry ID: {} - Fighter: {} - Color: c{:02}\0", self.entry_id, name, self.color);
        }
        &self.gui_strings.current_fighter_name
    }

    pub fn get_position(&mut self) -> *const f32 {
        unsafe {
            let pos = ((FighterManager::get_fighter_pos(singletons::FighterManager() as _, FighterEntryID(self.entry_id), 0)) as *const u64) as *const f32;
            self.position[0] = *pos;
            self.position[1] = *(pos.add(1));
            self.position[2] = *(pos.add(2));
            self.position.as_ptr()
        }
    }

    pub fn set_position(&mut self) {
        unsafe {
            StatusModule::set_situation_kind(self.boma as _, SituationKind(*SITUATION_KIND_AIR), true);
            GroundModule::correct(self.boma as _, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            
            let current_motion = MotionModule::motion_kind(self.boma as _);
            let current_frame = MotionModule::frame(self.boma as _);
            let current_rate = MotionModule::rate(self.boma as _);

            let pos = ((FighterManager::get_fighter_pos(singletons::FighterManager() as _, FighterEntryID(self.entry_id), 0)) as *const u64) as *const f32;
            FighterManager::set_position_lock(singletons::FighterManager() as _, FighterEntryID(self.entry_id), true);
            PostureModule::set_pos(self.boma as _, self.position.as_ptr() as _);
            
            MotionModule::change_motion(self.boma as _, smashline::Hash40::new_raw(current_motion), 0.0, 1.0, false, 0.0, false, false);
            MotionModule::set_rate(self.boma as _, current_rate);
            MotionModule::set_frame_sync_anim_cmd(self.boma as _, current_frame, true, false, false);

            self.is_pos_locked = true;
        }
    }

    pub fn set_lock_state(&self) {
        unsafe {
            FighterManager::set_position_lock(singletons::FighterManager() as _, FighterEntryID(self.entry_id), self.is_pos_locked);
        }
    }

    pub fn get_rotation(&mut self) -> *const f32 {
        unsafe {
            let rot = (PostureModule::rot(self.boma as _, 0) as *const u64) as *const f32;
            self.rotation[0] = *rot;
            self.rotation[1] = *(rot.add(1));
            self.rotation[2] = *(rot.add(2));
            self.rotation.as_ptr()
        }
    }

    pub fn set_rotation(&self) {
        unsafe {
            PostureModule::set_rot(self.boma as _, self.rotation.as_ptr() as _, 0);
        }
    }

    pub fn get_current_motion(&mut self) -> *const i32 {
        unsafe {
            let current_motion = MotionModule::motion_kind(self.boma as _);
            if current_motion != self.current_motion_hash {
                self.current_motion_hash = current_motion;
                let curr_hash = get_label(&current_motion);
                self.current_motion_idx = self.all_motions.iter().position(|x| x.clone().into_string().unwrap() == curr_hash).unwrap_or(0) as _;
                self.current_frame = MotionModule::frame(self.boma as _);
                self.max_frame = MotionModule::end_frame(self.boma as _);
                self.rate = MotionModule::rate(self.boma as _);
            }
            &self.current_motion_idx
        }
    }

    pub fn set_current_motion(&mut self) {
        unsafe {
            let current_motion = &self.all_motions[self.current_motion_idx as usize];
            let hash = get_hash(current_motion.clone().into_string().unwrap());
            let set_rate = if self.force_overwrite_rate { Some(self.rate) } else { None };
            MotionModule::change_motion(self.boma as _, smashline::Hash40::new_raw(hash), 0.0, 1.0, false, 0.0, false, false);
            if let Some(rate) = set_rate {
                MotionModule::set_rate(self.boma as _, rate);
            }
            self.current_frame = MotionModule::frame(self.boma as _);
            self.max_frame = MotionModule::end_frame(self.boma as _);
            self.rate = MotionModule::rate(self.boma as _);
        }
    }

    pub fn get_scale(&mut self) -> *const f32 {
        unsafe {
            self.scale = PostureModule::scale(self.boma as _);
            &self.scale
        }
    }

    pub fn set_scale(&mut self) {
        unsafe {
            PostureModule::set_scale(self.boma as _, self.scale, true);
        }
    }

    pub fn get_current_frame(&mut self) -> *const f32 {
        unsafe {
            self.current_frame = MotionModule::frame(self.boma as _);
            &self.current_frame
        }
    }
    
    pub fn set_current_frame(&mut self) {
        unsafe {
            MotionModule::set_frame_sync_anim_cmd(self.boma as _, self.current_frame, true, false, false);
            // MotionModule::set_frame(self.boma as _, self.current_frame, true);
        }
    }

    pub fn get_current_rate(&mut self) -> *const f32 {
        unsafe {
            self.rate = MotionModule::rate(self.boma as _);
            &self.rate
        }
    }
    
    pub fn set_current_rate(&mut self) {
        unsafe {
            MotionModule::set_rate(self.boma as _, self.rate);
        }
    }
}

pub const position_text: &str = "Position\0";
pub const scale_text: &str = "Scale\0";
pub const current_frame_text: &str = "Current Frame\0";
pub const current_rate_text: &str = "Current Rate\0";
pub const current_motion_text: &str = "Current Motion\0";
pub const position_locked_text: &str = "Is Position Locked\0";
pub const rotation_text: &str = "Rotation\0";
pub const freecam_text: &str = "Freecam\0";
pub const invert_mouse_hori_text: &str = "Invert Mouse (Horizontal)\0";
pub const invert_mouse_verti_text: &str = "Invert Mouse (Vertical)\0";
pub const mouse_sens_text: &str = "Mouse Sensitivity\0";
pub const base_movement_speed_text: &str = "Movement Speed (Base)\0";
pub const modifier_movement_speed_text: &str = "Movement Speed (Modifier)\0";
pub const locked_cam_text: &str = "Camera Locked\0";
pub const is_paused_text: &str = "Is Paused\0";
pub const give_final_text: &str = "Give Final Smash\0";
pub const fov_text: &str = "FOV\0";
pub const distance_text: &str = "Distance\0";
pub const camera_tools_text: &str = "Camera\0";
pub const overwrite_rate_text: &str = "Overwrite Rate\0";

// pub const fighters_name: [&str; 97] = ["bayonetta", "brave", "buddy", "captain", "chrom", "cloud", "daisy", "dedede", "demon", "diddy", "dolly", "donkey", "duckhunt", "edge", "eflame", "element", "elight", "falco", "fox", "gamewatch", "ganon", "gaogaen", "gekkouga", "ike", "inkling", "jack", "kamui", "ken", "kirby", "koopa", "koopag", "koopajr", "krool", "link", "littlemac", "lucario", "lucas", "lucina", "luigi", "mario", "mariod", "marth", "master", "metaknight", "mewtwo", "miienemyf", "miienemyg", "miienemys", "miifighter", "miigunner", "miiswordsman", "murabito", "nana", "ness", "packun", "pacman", "palutena", "peach", "pfushigisou", "pichu", "pickel", "pikachu", "pikmin", "pit", "pitb", "plizardon", "popo", "ptrainer", "ptrainer_low", "purin", "pzenigame", "reflet", "richter", "ridley", "robot", "rockman", "rosetta", "roy", "ryu", "samus", "samusd", "sheik", "shizue", "shulk", "simon", "snake", "sonic", "szerosuit", "tantan", "toonlink", "trail", "wario", "wiifit", "wolf", "yoshi", "younglink", "zelda"];

pub fn kind_to_folder_name(kind: i32) -> String {
    unsafe {
        let offset = offsets::offset_to_addr(0x4f80e20) as *const u64;
        let res = *(offset.add(kind as _));
        skyline::from_c_str(res as _)
    }
}

unsafe extern "C" fn draw(){
    if CURRENT_GAME_STATE == GameState::GAME_IN_PROGRESS && is_photo_camera() {
        if ENABLE_FREE_CAM && !LOCKED_CAM {
            imgui_api::imgui_smash_show_mouse(false);
            return;
        }

        imgui_api::imgui_smash_show_mouse(true);
        let disp_size = (*igGetIO()).DisplaySize;
        
        let window_width = get_fixed_width(500.0, disp_size.x);
        let window_y_pos = get_fixed_height(700.0, disp_size.y) ;
        let window_height = (disp_size.y - window_y_pos) - (get_fixed_height(14.0, disp_size.y));
        let windwow_x_pos = get_fixed_width(SOURCE_DISP_WIDTH - window_width, disp_size.x);
        igSetNextWindowPos(ImVec2{ x: windwow_x_pos, y: window_y_pos }, ImGuiCond_FirstUseEver as _, ImVec2{ x: 0.0, y: 0.0 });
        igSetNextWindowSize(ImVec2{ x: window_width, y: window_height }, ImGuiCond_FirstUseEver as _);
    
        let mut flags = 0;
        flags |= ImGuiWindowFlags_NoCollapse;
    
        let mut open = true;
        if(!igBegin(tool_title.as_ptr() as _, &mut open as *mut bool, flags as _)){
            igEnd();
            return;
        }
        
        if igCollapsingHeader_TreeNodeFlags(camera_tools_text.as_ptr() as _, ImGuiTreeNodeFlags_Framed as _) {
            if igCheckbox(freecam_text.as_ptr() as _, &mut ENABLE_FREE_CAM) {
                let front = &mut FRONT_VEC.lock().unwrap();
                front.x = 0.0;
                front.y = 0.0;
                front.z = -1.0;
                CameraMeleePhotoController::reset();
            }
    
            
            igCheckbox(locked_cam_text.as_ptr() as _, &mut LOCKED_CAM);
            if igCheckbox(is_paused_text.as_ptr() as _, &mut IS_PAUSED) {
                *(offsets::offset_to_addr(0x52b82ec) as *mut u8) = if IS_PAUSED { 4 } else { 0 };
            }
    
            igCheckbox(invert_mouse_hori_text.as_ptr() as _, &mut INVERT_MOUST_HORI);
            igCheckbox(invert_mouse_verti_text.as_ptr() as _, &mut INVERT_MOUST_VERTI);
            
            igDragFloat(mouse_sens_text.as_ptr() as _, &mut MOUSE_SENS, 0.01, 0.01, 0.5, std::ptr::null(), 0);
            igDragFloat(base_movement_speed_text.as_ptr() as _, &mut BASE_MOVEMENT_SPEED, 0.01, 0.01, 50.0, std::ptr::null(), 0);
            igDragFloat(modifier_movement_speed_text.as_ptr() as _, &mut MODIFIER_MOVEMENT_SPEED, 0.01, 0.01, 50.0, std::ptr::null(), 0);
            
            igDragFloat(fov_text.as_ptr() as _, &mut CURRENT_FOV, 0.001, 0.00001, 3.5, "%.6f\0".as_ptr() as _, 0);
            if igButton("Reset FOV\0".as_ptr() as _, ImVec2_c { x: 0.0, y: 0.0 }) {
                CURRENT_FOV = DEFAULT_FOV;
            }
            
            igDragFloat(distance_text.as_ptr() as _, &mut CURRENT_DISTANCE, 1.0, 0.01, 1000.0, std::ptr::null(), 0);
            if igButton("Reset Distance\0".as_ptr() as _, ImVec2_c { x: 0.0, y: 0.0 }) {
                CURRENT_DISTANCE = DEFAULT_DISTANCE;;
            }
        }


        igSeparator();
        let mut fighters = FIGHTERS.lock().unwrap();
        for x in 0..fighters.len() {
            let mut fighter: &mut FighterInfo = &mut fighters[x];
            
            if igCollapsingHeader_TreeNodeFlags(fighter.get_name().as_ptr() as _, ImGuiTreeNodeFlags_Framed as _) {
                igPushID_Int(x as _);
                if(igDragFloat3(position_text.as_ptr() as _, fighter.get_position() as _, 1.0, -10000.0, 10000.0, std::ptr::null(), 0)) {
                    fighter.set_position();
                }
                if igCheckbox(position_locked_text.as_ptr() as _, &mut (fighter.is_pos_locked)) {
                    fighter.set_lock_state();
                }
                if(igDragFloat3(rotation_text.as_ptr() as _, fighter.get_rotation() as _, 1.0,-360.0, 360.0, std::ptr::null(), 0)) {
                    fighter.set_rotation();
                }
                if igDragFloat(scale_text.as_ptr() as _, fighter.get_scale() as _, 0.01, 0.1, 50.0, std::ptr::null(), 0) {
                    fighter.set_scale();
                }
    
                igSpacing();
    
                if fighter.motion_enabled {
                    if igCombo_Str_arr(current_motion_text.as_ptr() as _, fighter.get_current_motion() as _, fighter.all_motions.as_ptr() as _, fighter.all_motions.len() as _, 200) {
                        fighter.set_current_motion();
                    }
                }
                
                if igSliderFloat(current_frame_text.as_ptr() as _, fighter.get_current_frame() as _, 0.0, fighter.max_frame, std::ptr::null(), 0) {
                    fighter.set_current_frame();
                }
                
                if igSliderFloat(current_rate_text.as_ptr() as _, fighter.get_current_rate() as _, 0.0, 10.0, std::ptr::null(), 0) {
                    fighter.set_current_rate();
                }

                igCheckbox(overwrite_rate_text.as_ptr() as _, &mut (fighter.force_overwrite_rate));
                
                
                if igButton(give_final_text.as_ptr() as _, ImVec2_c { x: 0.0, y: 0.0 }) {
                    smash::app::lua_bind::FighterManager::set_final(
                        singletons::FighterManager() as _, 
                        FighterEntryID(fighter.entry_id as _), 
                        smash::app::FighterAvailableFinal { _address: *(smash::lib::lua_const::FighterAvailableFinal::DEFAULT) as u8 },
                        0
                    );
                }
                igPopID();
            }
            igSeparator();
        }
    
        igEnd();
    }

}

static mut CURRENT_GAME_STATE: GameState = GameState::WAITING_FOR_GAME_START;
static mut ENABLE_FREE_CAM: bool = false;
static mut LOCKED_CAM: bool = false;
static mut IS_PAUSED: bool = false;
static mut INVERT_MOUST_HORI: bool = true;
static mut INVERT_MOUST_VERTI: bool = false;
static mut MOUSE_SENS: f32 = 0.01;
static mut BASE_MOVEMENT_SPEED: f32 = 1.0;
static mut MODIFIER_MOVEMENT_SPEED: f32 = 3.0;
static mut PRESSED_LEFT_ALT: bool = false;
static mut PRESSED_CTRL: bool = false;
static mut CURRENT_FOV: f32 = 0.5235988;
static mut CURRENT_DISTANCE: f32 = 0.01;
const DEFAULT_FOV: f32 = 0.5235988;
const DEFAULT_DISTANCE: f32 = 0.01;

pub unsafe fn update_state(state: &GameState){
    println!("{:?}", state);
    match state {
        GameState::WAITING_FOR_GAME_START => {
            //
            if *(offsets::offset_to_addr(0x53040f0) as *const u32) == 0x2010000 && is_ready_go(singletons::FighterManager() as _) {
                CURRENT_GAME_STATE = GameState::GAME_STARTED;
            }
        },
        GameState::GAME_STARTED =>  {
            ENABLE_FREE_CAM = false;
            let mut fighters = FIGHTERS.lock().unwrap();
            fighters.clear();
            for x in 0..FighterManager::entry_count(singletons::FighterManager() as _) {
                match FighterInfo::create(x) {
                    Some(fighter) => {
                        fighters.push(fighter);
                    },
                    None => {
                        fighters.clear();
                        CURRENT_GAME_STATE = GameState::WAITING_FOR_GAME_START;
                        return;
                    },
                }
            }
            CURRENT_GAME_STATE = GameState::GAME_IN_PROGRESS;
        },
        GameState::GAME_IN_PROGRESS => {
            if is_result_mode(singletons::FighterManager() as _) {
                CURRENT_GAME_STATE = GameState::WAITING_FOR_GAME_START;
            }
        },
    };
}

unsafe extern "C" fn setup_imgui_context(imgui_ctx: *mut u64){
    igSetCurrentContext(imgui_ctx as _);
}

fn parse_text_file(file_path: &std::path::PathBuf, hashmap: &mut HashMap<u64, String>){
    if !file_path.exists() {
        return;
    }

    let text_file_string = std::fs::read_to_string(file_path).unwrap();
    for line in text_file_string.split("\n") {
        let hash = smash_arc::hash40(line).0;
        if hashmap.contains_key(&hash){
            continue;
        }
        hashmap.insert(hash, line.to_string());
    }
}


#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.x * other.z) - (self.z * other.x),
            z: (self.x * other.y) - (self.y * other.z)
        }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let mag = self.magnitude();
        Vec3 {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

// This is a heavily trimmed down version of the proper struct ThatNintendoNerd documented, and major thanks to him for that.
// Source: https://github.com/ThatNintendoNerd/camera_free/blob/main/src/app/camera/camera_melee_photo_controller.rs#L9-L76
#[repr(C)]
pub struct CameraMeleePhotoController {
    _0x0: [u8; 0x3D0],
    pub pos: Vec3, // 0x3D0 + 12 = 0x3DC
    pub unk: f32, // 0x3DC + 4 = 0x3E0
    pub pitch: f32, // 0x3E0 + 4 = 0x3E4
    pub yaw: f32, // 0x3E4 + 4 = 0x3E8
    pub roll: f32, // 0x3E8 + 4 = 0x3EC
    pub unk2: u32, // 0x3EC + 8 = 0x3F4
    pub unk3: u32, // 0x3EC + 8 = 0x3F4
    pub distance: f32, // 0x3F4 + 4 = 0x3F8
    pub fov: f32, // 0x3F8 + 4 = 0x3FC
}

impl CameraMeleePhotoController {
    pub fn get_instance() -> *mut CameraMeleePhotoController {
        unsafe {
            let camera = *(crate::offsets::offset_to_addr(0x52b7f00) as *const u64);
            let ptr = *((*(camera as *const u64) + 0x10) as *const u64);
            let photo_mode_cam_ptr = *((ptr + 0x60) as *const u64);
            (photo_mode_cam_ptr as *mut CameraMeleePhotoController)
        }
    }

    pub fn reset() {
        unsafe {
            let cam = &mut *CameraMeleePhotoController::get_instance();
            CURRENT_FOV = DEFAULT_FOV;
            CURRENT_DISTANCE = CURRENT_DISTANCE;
            cam.pos.x = 0.0;
            cam.pos.y = 0.0;
            cam.pos.z = 100.0;
            cam.pitch = 0.0;
            cam.yaw = 0.0;
            cam.roll = 0.0;
            cam.distance = CURRENT_DISTANCE;
            cam.fov = CURRENT_FOV
        }
    }
}

#[skyline::hook(offset = 0x2c9b9c, inline)]
pub fn lobotomize_cpu(ctx: &mut InlineCtx) {
    ctx.registers[0].set_w(-1_i32 as u32);
}

#[skyline::hook(offset = 0x13c14e0)]
unsafe fn camera_maybe(param_1: u64, mut param_2: u32) {
    *((param_1 + 0x16) as *mut u64) = 1;
    original!()(param_1, param_2);
}

#[skyline::hook(offset = 0x13c15e8, inline)]
unsafe fn enable_camera_inline(ctx: &mut InlineCtx) {
    ctx.registers[8].set_w(1);
}

#[skyline::hook(offset = 0x50f980, inline)]
unsafe fn let_camera_start(ctx: &mut InlineCtx) {
    *((ctx.registers[9].x() + 0xc9c) as *mut u64) = 0;
}

#[skyline::main(name = "ingame-dev")]
pub fn main() {
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => match info.payload().downcast_ref::<String>() {
                Some(s) => &s[..],
                None => "Box<Any>",
            },
        };

        let err_msg = format!("ingame-dev has panicked at '{}', {}", msg, location);
        skyline::error::show_error(
            69,
            "ingame-dev has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));

    skyline::install_hook!(lobotomize_cpu);
    skyline::install_hooks!(
        camera_maybe,
        enable_camera_inline,
        let_camera_start
    );
    unsafe {
        imgui_api::imgui_setup_context(setup_imgui_context);
        imgui_api::imgui_smash_add_on_draw_frame(draw as _);
    }

    parse_text_file(&std::path::PathBuf::from_str("sd:/Labels.txt").unwrap(), &mut *(MOTION_LIST_HASHES.lock().unwrap()));

    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(10));
        loop {
            std::thread::sleep(std::time::Duration::from_millis(1000 / 60)); // Poll every frame
            unsafe {
                if CURRENT_GAME_STATE == GameState::GAME_IN_PROGRESS {
                    IS_PAUSED = *(offsets::offset_to_addr(0x52b82ec) as *mut u8) == 4;
                    
                    
                    if is_photo_camera() {
                        let mouse = MouseState::get_current_state();
                        let keyboard = KeyboardState::get_current_state();
                        if keyboard.modifier_pressed(KeyboardModifier::Control) {
                            if !PRESSED_CTRL {
                                IS_PAUSED = !IS_PAUSED;
                                *(offsets::offset_to_addr(0x52b82ec) as *mut u8) = if IS_PAUSED { 4 } else { 0 };
                                PRESSED_CTRL = true;
                            }
                        } else {
                            PRESSED_CTRL = false;
                        }
                        
                        if ENABLE_FREE_CAM {        
                            if keyboard.modifier_pressed(KeyboardModifier::LeftAlt) {
                                if !PRESSED_LEFT_ALT {
                                    LOCKED_CAM = !LOCKED_CAM;
                                    PRESSED_LEFT_ALT = true;
                                }
                            } else {
                                PRESSED_LEFT_ALT = false;
                            }
        
                            let photo_mode_cam = &mut *CameraMeleePhotoController::get_instance();
                            photo_mode_cam.distance = CURRENT_DISTANCE;
                            photo_mode_cam.fov = CURRENT_FOV;
        
                            if LOCKED_CAM {
                                continue;
                            }
        
        
                            let up = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
                            let front = &mut FRONT_VEC.lock().unwrap();
                            
                            let delta_x = mouse.delta_x as f32;
                            let delta_y = mouse.delta_y as f32;
        
                            let mouse_pos_info = Vec3 { x: if INVERT_MOUST_HORI { delta_x * -1.0 } else { delta_x }, y: if INVERT_MOUST_HORI { delta_y * -1.0 } else { delta_y }, z: 0.0 } * MOUSE_SENS;
                            photo_mode_cam.yaw += mouse_pos_info.x;
                            photo_mode_cam.pitch += mouse_pos_info.y;
        
                            if mouse.delta_x != 0 || mouse.delta_y != 0 {
                                let direction = Vec3 {
                                    x: photo_mode_cam.pitch.cos() * photo_mode_cam.yaw.sin(),
                                    y: photo_mode_cam.pitch.sin(),
                                    z: photo_mode_cam.yaw.cos() * photo_mode_cam.pitch.cos()
                                };
        
                                let dir = direction.normalize();
                                front.x = dir.x * -1.0;
                                front.y = dir.y;
                                front.z = dir.z * -1.0;
                            }
        
                            if photo_mode_cam.pitch.abs() > 89.0_f32.to_radians() {
                                photo_mode_cam.pitch = 89.0_f32.to_radians() * photo_mode_cam.pitch.signum();
                            }
        
                            let mut scaler: f32 = BASE_MOVEMENT_SPEED;
                            if keyboard.modifier_pressed(KeyboardModifier::Shift) {
                                scaler = MODIFIER_MOVEMENT_SPEED;
                            }
                            let speed = 0.4 * scaler;
                            let mut position = photo_mode_cam.pos;
                            if keyboard.key_pressed(KeyboardKey::W) {
                                photo_mode_cam.pos += (speed * **front);
                            } 
                            if keyboard.key_pressed(KeyboardKey::S) {
                                photo_mode_cam.pos -= (speed * **front);
                            }
                            if keyboard.key_pressed(KeyboardKey::A) {
                                photo_mode_cam.pos -= front.cross(&up).normalize() * speed;
                            }
                            if keyboard.key_pressed(KeyboardKey::D) {
                                photo_mode_cam.pos += front.cross(&up).normalize() * speed;
                            }
                        }
                    } else {
                        ENABLE_FREE_CAM = false;
                    }
                }
            }
        }
    });

    std::thread::spawn(|| {
        std::thread::sleep(std::time::Duration::from_secs(2));
        
        loop {
            std::thread::sleep(std::time::Duration::from_millis(1000 / 16));
            unsafe { update_state(&CURRENT_GAME_STATE); }
        }
    });
}
