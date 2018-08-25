macro_rules! items {
    ($($name:ident: $id:expr)*) => {
        $(
            pub const $name: u8 = $id;
        )*
    }
}

items! {
    // Bottles
    EMPTY_BOTTLE: 0x60
    RED_POTION: 0x61
    GREEN_POTION: 0x62
    BLUE_POTION: 0x63
    MILK: 0x64
    MILK_HALF: 0x65
    LANTERN_OIL: 0x66
    WATER: 0x67
    NASTY_SOUP: 0x6A
    HOT_SPRINGWATER: 0x6B
    FAIRY: 0x6C
    FAIRY_TEARS: 0x73
    WORM: 0x74
    BEE_LARVA: 0x76
    RARE_CHU_JELLY: 0x77
    RED_CHU_JELLY: 0x78
    BLUE_CHU_JELLY: 0x79
    GREEN_CHU_JELLY: 0x7A
    YELLOW_CHU_JELLY: 0x7B
    PURPLE_CHU_JELLY: 0x7C
    SIMPLE_SOUP: 0x7D
    GOOD_SOUP: 0x7E
    SUPERB_SOUP: 0x7F
    BLACK_CHU_JELLY: 0x9F
    // Bombs
    EMPTY_BOMB_BAG: 0x50
    REGULAR_BOMBS: 0x70
    WATER_BOMBS: 0x71
    BOMBLINGS: 0x72
    //Ooccoo
    OOCCOO: 0x25
    OOCCOO_JR: 0x27
    OOCCOOS_NOTE: 0x2D
    // Letter
    AURUS_MEMO: 0x90
    ASHEIS_SKETCH: 0x2D
    // Rod
    FISHING_ROD: 0x4A
    FISHING_ROD_BEE_LARVA: 0x5B
    FISHING_ROD_CORAL_EARRING: 0x5C
    FISHING_ROD_WORM: 0x5D
    FISHING_ROD_EARRING_BEE_LARVA: 0x5E
    FISHING_ROD_EARRING_WORM: 0x5F
    // Ilia's Quest
    RENARDOS_LETTER: 0x80
    INVOICE: 0x81
    WOODEN_STATUE: 0x82
    ILIAS_CHARM: 0x83
    HORSE_CALL: 0x84
    // City Quest
    SKY_BOOK_EMPTY: 0xE9
    SKY_BOOK_PARTLY_FILLED: 0xEA
    SKY_BOOK_FILLED: 0xEB
    // Clawshot
    CLAWSHOT: 0x44
    DOUBLE_CLAWSHOTS: 0x47
    SLINGSHOT: 0x4B
    LANTERN: 0x48
    BOOMERANG: 0x40
    IRON_BOOTS: 0x45
    HEROS_BOW: 0x43
    SPINNER: 0x41
    BALL_CHAIN: 0x42
    DOMINION_ROD: 0x46
    HAWKEYE: 0x3E
    EMPTY: 0xFF
}

pub enum ItemIdentifier {
    DoubleClawshots = 0xA,
    DominionRod = 0x8,
    BallChain = 0x6,
    Spinner = 0x2,
    HerosBow = 0x4,
    IronBoots = 0x3,
    Boomerang = 0x0,
    Lantern = 0x1,
    Slingshot = 0x17,
    Clawshot = 0x9,
    BombBag1 = 0xF,
    BombBag2 = 0x10,
    BombBag3 = 0x11,
    Bottle1 = 0xB,
    Bottle2 = 0xC,
    Bottle3 = 0xD,
    Bottle4 = 0xE,
    Hawkeye = 0x5,
    Oocco = 0x12,
    Letter = 0x13,
    IliaQuest = 0x15,
    Rod = 0x14,
    AncientSkyBook = 0x16,
    Empty = 0xFF,
}

impl From<u8> for ItemIdentifier {
    fn from(i: u8) -> ItemIdentifier {
        match i {
            0x0 => ItemIdentifier::Boomerang,
            0x1 => ItemIdentifier::Lantern,
            0x2 => ItemIdentifier::Spinner,
            0x3 => ItemIdentifier::IronBoots,
            0x4 => ItemIdentifier::HerosBow,
            0x5 => ItemIdentifier::Hawkeye,
            0x6 => ItemIdentifier::BallChain,
            0x8 => ItemIdentifier::DominionRod,
            0x9 => ItemIdentifier::Clawshot,
            0xA => ItemIdentifier::DoubleClawshots,
            0xB => ItemIdentifier::Bottle1,
            0xC => ItemIdentifier::Bottle2,
            0xD => ItemIdentifier::Bottle3,
            0xE => ItemIdentifier::Bottle4,
            0xF => ItemIdentifier::BombBag1,
            0x10 => ItemIdentifier::BombBag2,
            0x11 => ItemIdentifier::BombBag3,
            0x12 => ItemIdentifier::Oocco,
            0x13 => ItemIdentifier::Letter,
            0x14 => ItemIdentifier::Rod,
            0x15 => ItemIdentifier::IliaQuest,
            0x16 => ItemIdentifier::AncientSkyBook,
            0x17 => ItemIdentifier::Slingshot,
            0xFF => ItemIdentifier::Empty,
            _ => unreachable!(),
        }
    }
}

pub fn item_id_to_str(id: u8) -> &'static str {
    match id {
        EMPTY_BOTTLE => "Empty Bottle",
        RED_POTION => "Red Potion",
        GREEN_POTION => "Green Potion",
        BLUE_POTION => "Blue Potion",
        MILK => "Milk",
        MILK_HALF => "Milk Half",
        LANTERN_OIL => "Lantern Oil",
        WATER => "Water",
        NASTY_SOUP => "Nasty Soup",
        HOT_SPRINGWATER => "Hot Springwater",
        FAIRY => "Fairy",
        FAIRY_TEARS => "Fairy Tears",
        WORM => "Worm",
        BEE_LARVA => "Bee Larva",
        RARE_CHU_JELLY => "Rare Chu Jelly",
        RED_CHU_JELLY => "Red Chu Jelly",
        BLUE_CHU_JELLY => "Blue Chu Jelly",
        GREEN_CHU_JELLY => "Green Chu Jelly",
        YELLOW_CHU_JELLY => "Yellow Chu Jelly",
        PURPLE_CHU_JELLY => "Purple Chu Jelly",
        SIMPLE_SOUP => "Simple Soup",
        GOOD_SOUP => "Good Soup",
        SUPERB_SOUP => "Superb Soup",
        BLACK_CHU_JELLY => "Black Chu Jelly",
        EMPTY_BOMB_BAG => "Empty Bomb Bag",
        REGULAR_BOMBS => "Regular Bombs",
        WATER_BOMBS => "Water Bombs",
        BOMBLINGS => "Bomblings",
        OOCCOO => "Ooccoo",
        OOCCOO_JR => "Ooccoo Jr",
        OOCCOOS_NOTE => "Ooccoos Note",
        AURUS_MEMO => "Aurus Memo",
        ASHEIS_SKETCH => "Asheis Sketch",
        FISHING_ROD => "Fishing Rod",
        FISHING_ROD_BEE_LARVA => "Fishing Rod w/ Bee Larva",
        FISHING_ROD_CORAL_EARRING => "Fishing Rod w/ Coral Earring",
        FISHING_ROD_WORM => "Fishing Rod w/ Worm",
        FISHING_ROD_EARRING_BEE_LARVA => "Fishing Rod w/ Earring & Bee Larva",
        FISHING_ROD_EARRING_WORM => "Fishing Rod w/ Earring & Worm",
        RENARDOS_LETTER => "Renardo's Letter",
        INVOICE => "Invoice",
        WOODEN_STATUE => "Wooden Statue",
        ILIAS_CHARM => "Ilias Charm",
        HORSE_CALL => "Horse Call",
        SKY_BOOK_EMPTY => "Sky Book Empty",
        SKY_BOOK_PARTLY_FILLED => "Sky Book Partly Filled",
        SKY_BOOK_FILLED => "Sky Book Filled",
        SLINGSHOT => "Slingshot",
        LANTERN => "Lantern",
        BOOMERANG => "Boomerang",
        IRON_BOOTS => "Iron Boots",
        HEROS_BOW => "Hero S Bow",
        CLAWSHOT => "Clawshot",
        SPINNER => "Spinner",
        BALL_CHAIN => "Ball Chain",
        DOMINION_ROD => "Dominion Rod",
        DOUBLE_CLAWSHOTS => "Double Clawshots",
        HAWKEYE => "Hawkeye",
        _ => "",
    }
}
