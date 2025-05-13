use crate::game_data::ItemId;

#[derive(Copy, Clone, Debug)]
pub enum ExchangeWareData {
    Buy(ItemId, u32),
    Sell(ItemId, u32),
}
