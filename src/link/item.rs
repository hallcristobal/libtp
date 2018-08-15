macro_rules! items {
    ($($name:ident: $id:expr)*) => {
        $(
            pub const $name: u8 = $id;
        )*
    }
}

items! {
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
    EMPTY_BOMB_BAG: 0x50
    REGULAR_BOMBS: 0x70
    WATER_BOMBS: 0x71
    BOMBLINGS: 0x72
    OOCCOO: 0x25
    OOCCOO_JR: 0x27
    OOCCOOS_NOTE: 0x2D
    AURUS_MEMO: 0x90
    ASHEIS_SKETCH: 0x2D
    FISHING_ROD: 0x4A
    FISHING_ROD_BEE_LARVA: 0x5B
    FISHING_ROD_CORAL_EARRING: 0x5C
    FISHING_ROD_WORM: 0x5D
    FISHING_ROD_EARRING_BEE_LARVA: 0x5E
    FISHING_ROD_EARRING_WORM: 0x5F
    RENARDOS_LETTER: 0x80
    INVOICE: 0x81
    WOODEN_STATUE: 0x82
    ILIAS_CHARM: 0x83
    HORSE_CALL: 0x84
    SKY_BOOK_EMPTY: 0xE9
    SKY_BOOK_PARTLY_FILLED: 0xEA
    SKY_BOOK_FILLED: 0xEB
    SLINGSHOT: 0x4B
    LANTERN: 0x48
    BOOMERANG: 0x40
    IRON_BOOTS: 0x45
    HERO_S_BOW: 0x43
    CLAWSHOT: 0x44
    SPINNER: 0x41
    BALL_CHAIN: 0x42
    DOMINION_ROD: 0x46
    DOUBLE_CLAWSHOTS: 0x47
    HAWKEYE: 0x3E
}

pub enum ItemIdentifier {
    Slingshot = 0x17,
    Lantern = 0x1,
    Boomerang = 0x0,
    IronBoots = 0x3,
    HeroSBow = 0x4,
    Clawshot = 0x9,
    Spinner = 0x2,
    BallChain = 0x6,
    DominionRod = 0x8,
    DoubleClawshots = 0xA,
    Bottle1 = 0xB,
    Bottle2 = 0xC,
    Bottle3 = 0xD,
    Bottle4 = 0xE,
    BombBag1 = 0xF,
    BombBag2 = 0x10,
    BombBag3 = 0x11,
    Hawkeye = 0x5,
    Oocco = 0x12,
    Letter = 0x13,
    IliaQuest = 0x15,
    AncientSkyBook = 0x16,
}
