use link::{Inventory, Link};
use warping::Warp;

#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
pub struct Momentum {
    _p0: [u8; 0x4F8],
    pub link_momentum: Vec3,
}

#[repr(C)]
pub struct GameInfo {
    pub link: Link,                                  // 804061c0
    _p0: [u8; 0x7C],                                 // 804061e0
    pub inventory: Inventory,                        // 8040625C
    _p1: [u8; 0x4D15],                               // 804062B9
    pub warp: Warp,                                  // 8040AFCE
    _p2: [u8; 0x190],                                // 8040AFDD
    pub freeze_game: u8,                             // 8040B16D
    _p3: [u8; 0x70A],                                // 8040B16E
    pub momentum_ptr: Option<&'static mut Momentum>, // 8040B878
    _p4: [u8; 0x726],                                // 8040B87C
    pub link_air_meter: u16,                         // 8040BFA2
}

#[repr(C)]
pub struct GlobalCounters {
    pub game_counter: u32,
    pub game_counter2: u32,
    pub non_menu_counter: u32,
}

#[repr(C)]
pub struct ZelAudio {
    _p0: [u8; 0x4C4],                                   // 803DBF4C
    pub time_hours: u8,                                 // 803DC410
    pub time_minutes: u8,                               // 803DC411
    _p1: [u8; 0xA42],                                   // 803DC412
    pub link_debug_ptr: Option<&'static mut LinkDebug>, // 803DCE54
}

#[repr(C)]
pub struct LinkDebug {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    _p0: [u8; 0xA],
    pub facing: u16,
    _p1: [u8; 0x44],
    pub speed: f32,
}

#[repr(C)]
pub struct LinkRollConstants {
    _p0: [u8; 0x48],     // 8038D7BC
    pub roll_factor: f32 // 8038D804
}

extern "C" {
    #[link_name = "g_Counter"]
    pub static mut GLOBAL_COUNTERS: GlobalCounters;
    #[link_name = "g_mDoAud_zelAudio"]
    pub static mut ZEL_AUDIO: ZelAudio;
    #[link_name = "g_dComIfG_gameInfo"]
    pub static mut GAME_INFO: GameInfo;
    #[link_name = "daAlinkHIO_frontRoll_c0::m"]
    pub static mut LINK_ROLL_CONSTANTS: LinkRollConstants;
    #[link_name = "sConsole"]
    pub static mut SCONSOLE: u8;
}

pub fn get_frame_count() -> u32 {
    unsafe { GLOBAL_COUNTERS.game_counter }
}

pub fn get_link_debug() -> Option<&'static mut LinkDebug> {
    unsafe {
        if let Some(ref mut link_debug) = ZEL_AUDIO.link_debug_ptr {
            Some(*link_debug)
        } else {
            None
        }
    }
}

pub fn get_link_momentum() -> Option<&'static mut Momentum> {
    unsafe {
        if let Some(ref mut link_momentum) = GAME_INFO.momentum_ptr {
            Some(*link_momentum)
        } else {
            None
        }
    }
}

pub fn boss_flags_value() -> &'static mut u8 {
    unsafe { &mut *(&mut SCONSOLE as *mut u8).offset(8) }
}
