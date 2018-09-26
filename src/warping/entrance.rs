use system::memory::write_str;
#[repr(C)]
pub struct Entrance {
	pub stage: [u8; 8], 	// 8040AFCE
	_p0: u8, 			    // 8040AFD6
	pub spawn: u8, 			// 8040AFD7
	pub room: u8,  			// 8040AFD8
	pub state: u8, 			// 8040AFD9
}

impl Entrance {
	pub fn new(stage: &str, spawn: u8, room: u8, state: u8) -> Entrance {
		let mut entrance = Entrance {
			stage: [0;8],
			_p0: 0,
			spawn,
			room,
			state,
		};
		write_str(entrance.stage.as_mut_ptr(), stage);
		entrance
	}
}
