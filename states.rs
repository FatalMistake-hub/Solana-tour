use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct UserProfile {
    pub authority: Pubkey,
    pub last_tour: u8,
    pub tour_count: u8,
}

/// Size 2605 = 32 + 1 + 4 + 256 + 4 + 2048 + 4 + 256
#[account]
#[derive(Default)]
pub struct TourAccount {
    pub authority: Pubkey,   // 32
    pub idx: u8,             // 1
    pub orderId: String, // 4 + 256
    pub price: String, // 4 + 256
    pub tourTittle: String,   // 4 + 2048
    pub imageMain: String,   // 4 + 256
    pub timeId: String,
    pub userId: String,
    pub statusOrder: String,
    pub orderDate: String,
}

