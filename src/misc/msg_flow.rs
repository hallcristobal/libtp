extern "C" {
    #[link_name = "dMsgFlow_c::query042(mesg_flow_node_branch*, fopAc_ac_c*, i32)"]
    pub fn query042() -> i32;
}

#[repr(C)]
pub struct mesg_flow_node_branch {
    // not sure how to find this structure
}

#[repr(C)]
pub struct fopAc_ac_c {
    // not sure how to find this structure
}

#[repr(C)]
pub struct Query042 {
    _p0: [u8; 0x58],        // 8024BFEC
    pub mesg1: u32,         // 8024C044
    _p1: [u8; 0x10],        // 8024C048
    pub mesg2: u32,         // 8024C058
    _p2: [u8; 0x0C],        // 8024C05C
    pub mesg3: u32,         // 8024C068
    _p3: [u8; 0x3C],        // 8024C06C
}

// is this a use case for a macro?
impl Query042 {
    pub fn get_mesg1() -> &'static mut u32 {
        unsafe { &mut QUERY042.mesg1 }
    }

    pub fn get_mesg2() -> &'static mut u32 {
        unsafe { &mut QUERY042.mesg2 }
    }

    pub fn get_mesg3() -> &'static mut u32 {
        unsafe { &mut QUERY042.mesg3 }
    }
}