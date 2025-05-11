use bevy::prelude::Entity;
use std::sync::atomic::{AtomicU32, Ordering};

#[allow(dead_code)]
/// Unique ID that'll be used to match transactions between two inventories.
pub type TransactionId = u32;

#[allow(dead_code)]
static NEXT_TRANSACTION_ID: AtomicU32 = AtomicU32::new(0);

#[allow(dead_code)]
pub struct Transaction {
    pub id: TransactionId,
    pub buyer: Entity,
    pub seller: Entity,
    pub amount: u32,
}

#[allow(dead_code)]
impl Transaction {
    pub fn new(buyer: Entity, seller: Entity, amount: u32) -> Self {
        Self {
            id: NEXT_TRANSACTION_ID.fetch_add(1, Ordering::Relaxed),
            buyer,
            seller,
            amount,
        }
    }
}
