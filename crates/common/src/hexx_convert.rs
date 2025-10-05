pub trait HexxConvert {
    fn convert(self) -> bevy::prelude::Vec2;
}

impl HexxConvert for hexx::Vec2 {
    fn convert(self) -> bevy::prelude::Vec2 {
        bevy::prelude::Vec2::new(self.x, self.y)
    }
}

pub trait HexxConvertBack {
    fn convert(self) -> hexx::Vec2;
}

impl HexxConvertBack for bevy::prelude::Vec2 {
    fn convert(self) -> hexx::Vec2 {
        hexx::Vec2::new(self.x, self.y)
    }
}
