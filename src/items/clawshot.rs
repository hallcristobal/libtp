extern "C" {
    #[link_name = "daAlinkHIO_hookshot_c0::m"]
    pub static mut CLAWSHOT: Clawshot;
    #[link_name = "daAlink_c::checkHookshotStickBG(cBgS_PolyInfo&)"]
    pub static mut CLAWSHOT_BG: Clawshot_BG;
}

#[repr(C)]
pub struct Clawshot {
    _p0: [u8; 0x48],            // 8038E9C0
    pub extension_rate : f32,   // 8038EA08 
    pub speed: f32,             // 8038EA0C 
    pub retraction_rate: f32,   // 8038EA10 
    pub pull_rate: f32,         // 8038EA14 
    _p1: [u8; 0x1C],            // 8038EA18 
}

impl Clawshot {
    pub fn get_speed() -> &'static mut f32 {
        unsafe { &mut CLAWSHOT.speed }
    }

    pub fn get_extension_rate() -> &'static mut f32 {
        unsafe { &mut CLAWSHOT.extension_rate}
    }

    pub fn get_retraction_rate() -> &'static mut f32 {
        unsafe { &mut CLAWSHOT.retraction_rate}
    }

    pub fn get_pull_rate() -> &'static mut f32 {
        unsafe { &mut CLAWSHOT.pull_rate}
    }
}

#[repr(C)]
pub struct Clawshot_BG {
    _p0: [u8; 0x30],    // 801087B0
    pub is_target: u32, // 801087E0         
    _p1: [u8; 0x44],    // 801087E4
}

impl Clawshot_BG {
    pub fn get_background() -> &'static mut u32 {
        unsafe { &mut CLAWSHOT_BG.is_target }
    }
}