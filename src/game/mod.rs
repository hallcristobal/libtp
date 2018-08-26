pub mod controller;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flag(pub *mut u8, pub u8);

impl Flag {
	pub fn from_ref(src: &mut u8, mask: u8) -> Flag {
        Flag(src as *mut u8, 1 << mask)
	}

    pub fn activate(self) {
        let Flag(addr, value) = self;
        let ptr = unsafe {
			&mut *addr
		};
		*ptr |= value;
    }

    pub fn deactivate(self) {
        let Flag(addr, value) = self;
        let ptr = unsafe {
			&mut *addr
		};
		*ptr &= 0xFF ^ value;
    }

    pub fn is_active(self) -> bool {
        let Flag(addr, mask) = self;
        let value = unsafe {
			&mut *addr
		};
        *value & mask != 0
    }

    pub fn toggle(self) {
        if self.is_active() {
            self.deactivate()
        } else {
            self.activate()
        }
    }
}
