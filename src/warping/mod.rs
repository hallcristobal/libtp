pub mod entrance;
pub use self::entrance::Entrance;
use system::tp::GAME_INFO;

#[repr(C)]
#[derive(Clone)]
pub struct Warp {
	pub entrance: Entrance,	// 8040AFCE
	_p0: [u8; 2], 		    // 8040AFDA
	pub enabled: bool, 		// 8040AFDC
}

impl Warp {
	pub fn new(stage: &str, room: u8, spawn: u8, state: u8) -> Warp {
		Warp {
			entrance: Entrance::new(stage, room, spawn,  state),
			_p0: [0;2],
			enabled: true,
		}
	}

	pub fn last_warp() -> Warp {
		unsafe {
			GAME_INFO.warp.clone()
		}
	}

	pub fn execute(self) {
		unsafe {
			GAME_INFO.warp = self;
		}
	}
}
