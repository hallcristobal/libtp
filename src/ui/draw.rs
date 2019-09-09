extern "C" {
    #[link_name = "g_drawHIO"]
    pub static mut DRAW: Draw;
}

#[repr(C)]
pub struct Draw {
    _p0: [u8; 0x18],        // 8042EBC8
    pub hud: u16,           // 8042EBE0
    _p1: [u8; 0xF0E],       // 8042EBE2
}

impl Draw {
    pub fn get_hud() -> &'static mut u16 {
        unsafe { &mut DRAW.hud }
    }
}