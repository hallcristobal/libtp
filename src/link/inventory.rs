#[repr(C)]
pub struct Inventory {
	pub item_values: ItemValues,
	pub item_wheel: ItemWheel,
	_p0: [u8; 32],
	pub arrow_count: u8, // 62aC
	pub bomb_bag_1_amnt: u8,
	pub bomb_bag_2_amnt: u8,
	pub bomb_bag_3_amnt: u8,
}

#[repr(C)] // 24
pub struct ItemValues{
	pub gale_boomerang_id: u8,
	pub lantern_id: u8,
	pub spinner_id: u8,
	pub iron_boots_id: u8,
	pub hero_s_bow_id: u8,
	pub hawkeye_id: u8,
	pub ball_and_chain_id: u8,
	pub item_slot_id: u8,
	pub dominion_rod_id: u8,
	pub clawshot_id: u8,
	pub double_clawshot_id: u8,
	pub bottle_1_id: u8,
	pub bottle_2_id: u8,
	pub bottle_3_id: u8,
	pub bottle_4_id: u8,
	pub bomb_bag_1_id: u8,
	pub bomb_bag_2_id: u8,
	pub bomb_bag_3_id: u8,
	pub ooccoo_id: u8,
	pub auru_s_memo_ashei_s_sketch_id: u8,
	pub fishing_rod_earring_id: u8,
	pub horse_call_id: u8,
	pub ancient_sky_book_id: u8,
	pub slingshot_id: u8,
}

#[repr(C)] // 24
pub struct ItemWheel {
	pub item_slot_1: u8,
	pub item_slot_2: u8,
	pub item_slot_3: u8,
	pub item_slot_4: u8,
	pub item_slot_5: u8,
	pub item_slot_6: u8,
	pub item_slot_7: u8,
	pub item_slot_8: u8,
	pub item_slot_9: u8,
	pub item_slot_10: u8,
	pub item_slot_11: u8,
	pub item_slot_12: u8,
	pub item_slot_13: u8,
	pub item_slot_14: u8,
	pub item_slot_15: u8,
	pub item_slot_16: u8,
	pub item_slot_17: u8,
	pub item_slot_18: u8,
	pub item_slot_19: u8,
	pub item_slot_20: u8,
	pub item_slot_21: u8,
	pub item_slot_22: u8,
	pub item_slot_23: u8,
	pub item_slot_24: u8,
}
