extern "C" {
    #[link_name = "daAlinkHIO_hookshot_c0::m"]
    pub static mut CLAWSHOT: Clawshot;
    #[link_name = "daAlink_c::checkHookshotStickBG(cBgS_PolyInfo&)"]
    pub static mut CLAWSHOT_BG: Clawshot_BG;
}

#[repr(C)]
pub struct Clawshot {
    _p0: [u8; 0x04],                            // 8038E9C0
    pub arm_animation1: u16,                    // 8038E9C4 affects first person anim while firing
    _01: [u8; 0x02],                            // 8038E9C6
    pub arm_animation2: u16,                    // 8038E9C8 affects first person anim while firing
    _p2: [u8; 0x22],                            // 8038E9CA
    pub wall_hang_anim_speed: u16,              // 8038E9EC
    _p2: [u8; 0x02],                            // 8038E9EE
    pub wall_hang_anim: u16,                    // 8038E9F0 has something to do with wall anim but not speed specifically
    _p3: [u8; 0x06],                            // 8038EAF2 
    pub second_clawshot_wait_frames: u16,       // 8038EAF8 how long before you can use clawshot 2 when hanging from wall (ceiling is different) 
    _p4: [u8; 0x06],                            // 8038EAFA
    pub third_person_ready_anim_angle: u16,     // 8038EA00 data type is wrong probably
    pub third_person_ready_anim_speed: u16,     // 8038EA02 data type is wrong probably
    pub first_person_ready_anim_frames: f32,    // 8038EA04
    pub extension_rate : f32,                   // 8038EA08 
    pub speed: f32,                             // 8038EA0C 
    pub retraction_rate: f32,                   // 8038EA10 
    pub pull_rate: f32,                         // 8038EA14 
    _p5: [u8; 0x1C],                            // 8038EA18
}

impl Clawshot {
    pub fn get_arm_animation1() -> &'static mut u16 {
        unsafe { &mut CLAWSHOT.arm_animation1 }
    }

    pub fn get_arm_animation2() -> &'static mut u16 {
        unsafe { &mut CLAWSHOT.arm_animation2 }
    }

    pub fn get_wall_hang_anim_speed() -> &'static mut u16 {
        unsafe { &mut CLAWSHOT.wall_hang_anim_speed }
    }

    pub fn get_wall_hang_anim() -> &'static mut u16 {
        unsafe { &mut CLAWSHOT.wall_hang_anim }
    }

    pub fn get_second_clawshot_wait_frames() -> &'static mut u16 {
        unsafe { &mut CLAWSHOT.second_clawshot_wait_frames }
    }

    pub fn get_third_person_ready_anim_angle() () -> &'static mut u16 {
        unsafe { &mut CLAWSHOT.third_person_ready_anim_angle }
    }

    pub fn get_third_person_ready_anim_speed() () -> &'static mut u16 {
        unsafe { &mut CLAWSHOT.third_person_ready_anim_speed }
    }

    pub fn get_first_person_ready_anim_frames() -> &'static mut f32 {
        unsafe { &mut CLAWSHOT.first_person_ready_anim_frames}
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