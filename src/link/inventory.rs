use system::tp::GAME_INFO;
use super::super::arrayvec::ArrayVec;

extern "C" {
    #[link_name="execItemGet(u8)"]
    fn item_get(item_id: u8);
}

fn get_state() -> InventoryState {
    use super::item::ItemIdentifier::*;
    let mut state = InventoryState::default();
    let inventory = Inventory::get_inventory();
    inventory
        .item_wheel
        .slot
        .into_iter()
        .enumerate()
        .for_each(|(i, id)| {
            match (*id).into() {
                DoubleClawshots => state.double_clawshots_slot = i,
                DominionRod => state.dominion_rod_slot = i,
                BallChain => state.ball_chain_slot = i,
                Spinner => state.spinner_slot = i,
                HerosBow => state.heros_bow_slot = i,
                IronBoots => state.iron_boots_slot = i,
                Boomerang => state.boomerang_slot = i,
                Lantern => state.lantern_slot = i,
                Slingshot => state.slingshot_slot = i,
                Clawshot => state.clawshot_slot = i,
                BombBag1 => state.bomb_bag1_1_slot = i,
                BombBag2 => state.bomb_bag1_2_slot = i,
                BombBag3 => state.bomb_bag1_3_slot = i,
                Bottle1 => state.bottle_1_slot = i,
                Bottle2 => state.bottle_2_slot = i,
                Bottle3 => state.bottle_3_slot = i,
                Bottle4 => state.bottle_4_slot = i,
                Hawkeye => state.hawkeye_slot = i,
                Oocco => state.ooccoo_slot = i,
                Letter => state.letter_slot = i,
                IliaQuest => state.ilia_quest_slot = i,
                Rod => state.fishing_rod_slot = i,
                AncientSkyBook => state.sky_book_slot = i,
                Empty => {}
            };
        });
    state
}

struct InventoryState {
    pub double_clawshots_slot: usize,
    pub dominion_rod_slot: usize,
    pub ball_chain_slot: usize,
    pub spinner_slot: usize,
    pub heros_bow_slot: usize,
    pub iron_boots_slot: usize,
    pub boomerang_slot: usize,
    pub lantern_slot: usize,
    pub slingshot_slot: usize,
    pub clawshot_slot: usize,
    pub fishing_rod_slot: usize,
    pub hawkeye_slot: usize,
    pub bomb_bag1_1_slot: usize,
    pub bomb_bag1_2_slot: usize,
    pub bomb_bag1_3_slot: usize,
    pub bottle_1_slot: usize,
    pub bottle_2_slot: usize,
    pub bottle_3_slot: usize,
    pub bottle_4_slot: usize,
    pub sky_book_slot: usize,
    pub ilia_quest_slot: usize,
    pub ooccoo_slot: usize,
    pub letter_slot: usize,
}

impl Default for InventoryState {
    fn default() -> InventoryState {
        InventoryState {
            double_clawshots_slot: 0xFF,
            dominion_rod_slot: 0xFF,
            ball_chain_slot: 0xFF,
            spinner_slot: 0xFF,
            heros_bow_slot: 0xFF,
            iron_boots_slot: 0xFF,
            boomerang_slot: 0xFF,
            lantern_slot: 0xFF,
            slingshot_slot: 0xFF,
            clawshot_slot: 0xFF,
            fishing_rod_slot: 0xFF,
            hawkeye_slot: 0xFF,
            bomb_bag1_1_slot: 0xFF,
            bomb_bag1_2_slot: 0xFF,
            bomb_bag1_3_slot: 0xFF,
            bottle_1_slot: 0xFF,
            bottle_2_slot: 0xFF,
            bottle_3_slot: 0xFF,
            bottle_4_slot: 0xFF,
            sky_book_slot: 0xFF,
            ilia_quest_slot: 0xFF,
            ooccoo_slot: 0xFF,
            letter_slot: 0xFF,
        }
    }
}

#[repr(C)]
pub struct Inventory {
    pub item_values: ItemValues,
    pub item_wheel: ItemWheel,
    _p0: [u8; 3],
    pub rupee_cs_flags: u8,
    _p1: [u8; 28],
    pub arrow_count: u8, // 62aC
    pub bomb_bag_1_amnt: u8,
    pub bomb_bag_2_amnt: u8,
    pub bomb_bag_3_amnt: u8,
}

impl Inventory {
    pub fn get_inventory() -> &'static mut Inventory {
        unsafe { &mut GAME_INFO.inventory }
    }

    pub fn get_by_slot_id(&self, slot_id: usize) -> u8 {
        match slot_id {
            DOUBLE_CLAWSHOT_ID_VALUE => self.item_values.value[DOUBLE_CLAWSHOT_ID_VALUE],
            DOMINION_ROD_ID_VALUE => self.item_values.value[DOMINION_ROD_ID_VALUE],
            BALL_AND_CHAIN_ID_VALUE => self.item_values.value[BALL_AND_CHAIN_ID_VALUE],
            SPINNER_ID_VALUE => self.item_values.value[SPINNER_ID_VALUE],
            HEROS_BOW_ID_VALUE => self.item_values.value[HEROS_BOW_ID_VALUE],
            IRON_BOOTS_ID_VALUE => self.item_values.value[IRON_BOOTS_ID_VALUE],
            GALE_BOOMERANG_ID_VALUE => self.item_values.value[GALE_BOOMERANG_ID_VALUE],
            LANTERN_ID_VALUE => self.item_values.value[LANTERN_ID_VALUE],
            SLINGSHOT_ID_VALUE => self.item_values.value[SLINGSHOT_ID_VALUE],
            CLAWSHOT_ID_VALUE => self.item_values.value[CLAWSHOT_ID_VALUE],
            FISHING_ROD_ID_VALUE => self.item_values.value[FISHING_ROD_ID_VALUE],
            HAWKEYE_ID_VALUE => self.item_values.value[HAWKEYE_ID_VALUE],
            BOMB_BAG_1_ID_VALUE => self.item_values.value[BOMB_BAG_1_ID_VALUE],
            BOMB_BAG_2_ID_VALUE => self.item_values.value[BOMB_BAG_2_ID_VALUE],
            BOMB_BAG_3_ID_VALUE => self.item_values.value[BOMB_BAG_3_ID_VALUE],
            BOTTLE_1_ID_VALUE => self.item_values.value[BOTTLE_1_ID_VALUE],
            BOTTLE_2_ID_VALUE => self.item_values.value[BOTTLE_2_ID_VALUE],
            BOTTLE_3_ID_VALUE => self.item_values.value[BOTTLE_3_ID_VALUE],
            BOTTLE_4_ID_VALUE => self.item_values.value[BOTTLE_4_ID_VALUE],
            SKY_BOOK_ID_VALUE => self.item_values.value[SKY_BOOK_ID_VALUE],
            ILIA_QUEST_ID_VALUE => self.item_values.value[ILIA_QUEST_ID_VALUE],
            OOCCOO_ID_VALUE => self.item_values.value[OOCCOO_ID_VALUE],
            LETTER_ID_VALUE => self.item_values.value[LETTER_ID_VALUE],
            _ => 0xFF
        }
    }

    pub fn set_by_slot_id(&mut self, slot_id: usize, item_id: u8) {
        let state = get_state();
        let inv_slot = match slot_id {
            DOUBLE_CLAWSHOT_ID_VALUE => state.double_clawshots_slot,
            DOMINION_ROD_ID_VALUE => state.dominion_rod_slot,
            BALL_AND_CHAIN_ID_VALUE => state.ball_chain_slot,
            SPINNER_ID_VALUE => state.spinner_slot,
            HEROS_BOW_ID_VALUE => state.heros_bow_slot,
            IRON_BOOTS_ID_VALUE => state.iron_boots_slot,
            GALE_BOOMERANG_ID_VALUE => state.boomerang_slot,
            LANTERN_ID_VALUE => state.lantern_slot,
            SLINGSHOT_ID_VALUE => state.slingshot_slot,
            CLAWSHOT_ID_VALUE => state.clawshot_slot,
            FISHING_ROD_ID_VALUE => state.fishing_rod_slot,
            HAWKEYE_ID_VALUE => state.hawkeye_slot,
            BOMB_BAG_1_ID_VALUE => state.bomb_bag1_1_slot,
            BOMB_BAG_2_ID_VALUE => state.bomb_bag1_2_slot,
            BOMB_BAG_3_ID_VALUE => state.bomb_bag1_3_slot,
            BOTTLE_1_ID_VALUE => state.bottle_1_slot,
            BOTTLE_2_ID_VALUE =>state.bottle_2_slot,
            BOTTLE_3_ID_VALUE =>state.bottle_3_slot,
            BOTTLE_4_ID_VALUE =>state.bottle_4_slot,
            SKY_BOOK_ID_VALUE => state.sky_book_slot,
            ILIA_QUEST_ID_VALUE => state.ilia_quest_slot,
            OOCCOO_ID_VALUE => state.ooccoo_slot,
            LETTER_ID_VALUE => state.letter_slot,
            _ => {slot_id}
        };

        report!("setting values slot {} to {:X}", slot_id, item_id);

        if item_id == 0xFF {
            self.remove_item(inv_slot);
        } else if inv_slot == 0xFF {
                self.add_item(item_id);
        }
        self.item_values.value[slot_id] = item_id;
    }

    fn remove_item(&mut self, wheel_slot: usize) {
        let mut new_wheel = ArrayVec::from(self.item_wheel.slot);
        if let Some(_) = new_wheel.pop_at(wheel_slot) {
            new_wheel.push(0xFF);
            new_wheel.into_iter().enumerate().for_each(|(i, id)| {
                self.item_wheel.slot[i] = id;
            });
        }
    }

    fn add_item(&mut self, item_id: u8) {
        unsafe { item_get(item_id); }
    }
}

pub const GALE_BOOMERANG_ID_VALUE: usize = 0;
pub const LANTERN_ID_VALUE: usize = 1;
pub const SPINNER_ID_VALUE: usize = 2;
pub const IRON_BOOTS_ID_VALUE: usize = 3;
pub const HEROS_BOW_ID_VALUE: usize = 4;
pub const HAWKEYE_ID_VALUE: usize = 5;
pub const BALL_AND_CHAIN_ID_VALUE: usize = 6;
pub const DOMINION_ROD_ID_VALUE: usize = 8;
pub const CLAWSHOT_ID_VALUE: usize = 9;
pub const DOUBLE_CLAWSHOT_ID_VALUE: usize = 10;
pub const BOTTLE_1_ID_VALUE: usize = 11;
pub const BOTTLE_2_ID_VALUE: usize = 12;
pub const BOTTLE_3_ID_VALUE: usize = 13;
pub const BOTTLE_4_ID_VALUE: usize = 14;
pub const BOMB_BAG_1_ID_VALUE: usize = 15;
pub const BOMB_BAG_2_ID_VALUE: usize = 16;
pub const BOMB_BAG_3_ID_VALUE: usize = 17;
pub const OOCCOO_ID_VALUE: usize = 18;
pub const LETTER_ID_VALUE: usize = 19;
pub const FISHING_ROD_ID_VALUE: usize = 20;
pub const ILIA_QUEST_ID_VALUE: usize = 21;
pub const SKY_BOOK_ID_VALUE: usize = 22;
pub const SLINGSHOT_ID_VALUE: usize = 23;

#[repr(C)]
pub struct ItemValues {
    // pub gale_boomerang_id: u8,
    // pub lantern_id: u8,
    // pub spinner_id: u8,
    // pub iron_boots_id: u8,
    // pub hero_s_bow_id: u8,
    // pub hawkeye_id: u8,
    // pub ball_and_chain_id: u8,
    // _p0: u8,
    // pub dominion_rod_id: u8,
    // pub clawshot_id: u8,
    // pub double_clawshot_id: u8, 10
    // pub bottle_1_id: u8,
    // pub bottle_2_id: u8,
    // pub bottle_3_id: u8,
    // pub bottle_4_id: u8,
    // pub bomb_bag_1_id: u8,
    // pub bomb_bag_2_id: u8,
    // pub bomb_bag_3_id: u8,
    // pub ooccoo_id: u8,
    // pub auru_s_memo_ashei_s_sketch_id: u8,
    // pub fishing_rod_earring_id: u8,
    // pub horse_call_id: u8,
    // pub ancient_sky_book_id: u8,
    // pub slingshot_id: u8,
    // _p1: u8,
    pub value: [u8; 24],
}

#[repr(C)]
pub struct ItemWheel {
    pub slot: [u8; 24],
}
