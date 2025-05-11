use hexx::Hex;

pub const CENTER: Hex = Hex::ZERO;
pub const RIGHT: Hex = Hex::new(1, 0);
pub const TOP_RIGHT: Hex = Hex::new(0, 1);
pub const BOTTOM_LEFT: Hex = Hex::new(0, -1);

pub const TOP_RIGHT_TOP_RIGHT: Hex = Hex::new(0, 2);
