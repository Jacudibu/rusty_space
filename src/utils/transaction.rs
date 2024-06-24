use bevy::prelude::Entity;
use std::sync::atomic::{AtomicU32, Ordering};

/// Unique ID that'll be used to match transactions between two inventories.
pub type TransactionId = u32;
static NEXT_TRANSACTION_ID: AtomicU32 = AtomicU32::new(0);

pub struct Transaction {
    id: TransactionId,
    buyer: Entity,
    seller: Entity,
    amount: u32,
}

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
