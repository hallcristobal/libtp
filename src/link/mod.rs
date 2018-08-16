pub mod item;
pub mod inventory;

pub use self::inventory::Inventory;

use system::tp::{self, GAME_INFO};

#[repr(C)]
pub struct Link {
	_p0: u8,				// C0
	heart_pieces: u8,		// C1
	_p1: u8,				// C2
	heart_quarters: u8,		// C3
	rupees: u16,			// C4-5
	_p2: [u8; 2],			// C6-7
	lamp_fuel: u16,			// C8-9
	_p3: u8,				// CA
	item_on_x: u8,			// CB
	item_on_y: u8,			// CC
	_p4: [u8; 2],			// CD
	slot_x_combo_item: u8,	// CF
	slot_y_combo_item: u8,	// D0
	_p5: [u8; 2],			// D1-2
	armor: u8,				// D3
	sword: u8,				// D4
	shield: u8,				// D5
	_p6: [u8; 3],			// D6-8
	wallet_upgrade: u8,		// D9
	_p7: [u8; 4],			// DA-D
	is_wolf: bool,			// DE
	_p8: u8					// DF
}

impl Link {
	pub fn get_link() -> &'static mut Link {
		unsafe { &mut GAME_INFO.link }
	}

	pub fn get_position() -> Option<&'static mut tp::LinkDebug> {
		tp::get_link_debug()
	}
}

