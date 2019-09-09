extern "C" {
    #[link_name = "g_dComIfAc_gameInfo"]
    pub static mut COMINFACTOR: Actor_Command; // not sure what com is assuming command
}

#[repr(C)]
pub struct Actor_Command {
    _p0: [u8; 0x04],        // 80450610
    pub freeze: bool,       // 80450614
    _p1: [u8; 0x03],        // 80450615
}

impl Actor_Command {
    pub fn check_frozen() -> &'static mut bool {
        unsafe { &mut COMINFACTOR.freeze }
    }
}