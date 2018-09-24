pub mod entrance;
use self::entrance::Entrance;
use system::tp::GAME_INFO;

#[repr(C)]
pub struct Warp {
	entrance: Entrance,	// 8040AFCE
	_p0: [u8; 2], 		// 8040AFDA
	enabled: bool, 		// 8040AFDC
}

impl Warp {
	pub fn new(stage: &str, room: u8, state: u8, spawn: u8) -> Warp {
		Warp {
			entrance: Entrance::new(stage, spawn, room, state),
			_p0: [0;2],
			enabled: true,
		}
	}

	pub fn last_warp() -> &'static Warp {
		unsafe {
			&GAME_INFO.warp
		}
	}

	pub fn execute(self) {
		unsafe {
			GAME_INFO.warp = self;
		}
	}
}
