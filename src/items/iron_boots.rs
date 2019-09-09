extern "C" {
    #[link_name = "daAlinkHIO_magneBoots_c0::m"]
    pub static mut IRONBOOTS: IronBoots;
}

#[repr(C)]
pub struct IronBoots {
    _p0: [u8; 0x04],                // 8038E7F4
    switch_speed: u16,              // 8038E7F8
    _p1: [u8; 0x02],                // 8038E7FA
    switch_speed2: u8,              // 8038E7FC this single byte seem to affect switching speed as well?
    _p3: [u8; 0x07],                // 8038E7FD
    wait_after_switch: u16,         // 8038E804
    _p4: [u8; 0x02],                // 8038E806
    weight1: u8,                    // 8038E808 can't figure out this one :(
    weight2: u8,                    // 8038E809
    _p5: [u8; 0x02],                // 8038E80A
    speed: f32,                     // 8038E80C affects speed but doesn't appear to be float?
    stepping_anim_speed: u16,       // 8038E810
    _p6: [u8; 0x02],                // 8038E812
    ess_anim_speed: u16,            // 8038E814
    _p7: [u8; 0x12],                // 8038E816
    magnet_strength: u8,            // 8038E828
    _p8: [u8; 0x17],                // 8038E829
}