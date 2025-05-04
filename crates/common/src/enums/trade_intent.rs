/// What is our own role in an upcoming trade?
#[derive(Eq, PartialEq)]
pub enum TradeIntent {
    Buy,
    Sell,
}
