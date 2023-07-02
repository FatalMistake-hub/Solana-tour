use anchor_lang::prelude::*;
pub mod constant;
pub mod states;
use crate::{constant::*, states::*};

declare_id!("8GUj8XTtQMoQNixfoSG4E6viD4LojLc4GCs6FrninZM2");

#[program]
pub mod clever_tour {
    use super::*;

    pub fn initialize_user(
        ctx: Context<InitializeUser>
    ) -> Result<()> {
        // Initialize user profile with default data
  
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.authority = ctx.accounts.authority.key();
        user_profile.last_tour = 0;
        user_profile.tour_count = 0;

        Ok(())
    }

    pub fn add_tour(
        ctx: Context<AddTour>, 
        orderId: String, 
        price: String,   
        tourTittle: String, 
        imageMain: String,   
        timeId: String, 
        userId: String, 
        orderDate: String, 

    ) -> Result<()> {
        let tour_account = &mut ctx.accounts.tour_account;
        let user_profile = &mut ctx.accounts.user_profile;

        // Fill contents with argument
        tour_account.authority = ctx.accounts.authority.key();
        tour_account.idx = user_profile.last_tour;
        tour_account.orderId = orderId;
        tour_account.tourTittle = tourTittle;
        tour_account.price = price;
        tour_account.imageMain = imageMain;
        tour_account.timeId = timeId;
        tour_account.userId = userId;
        tour_account.statusOrder = "SUCCESS".to_string();
        tour_account.orderDate = orderDate;

        // Increase tour idx for PDA
        user_profile.last_tour = user_profile.last_tour
            .checked_add(1)
            .unwrap();

        // Increase total tour count
        user_profile.tour_count = user_profile.tour_count
            .checked_add(1)
            .unwrap();

        Ok(())
    }
    pub fn update_tour(
        ctx: Context<UpdateTour>, 
        tour_idx: u8,
    ) -> Result<()> {
        let tour_account = &mut ctx.accounts.tour_account;
        tour_account.statusOrder = "USED".to_string();
        Ok(())
    }
}

    
#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<UserProfile>(),
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct AddTour<'info> {
    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        init,
        seeds = [TOUR_TAG, authority.key().as_ref(), &[user_profile.last_tour]],
        bump,
        payer = authority,
        space = 3905 + 8,
    )]
    pub tour_account: Box<Account<'info, TourAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
#[instruction(tour_idx: u8)]

pub struct UpdateTour<'info> {
    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        mut,
        seeds = [TOUR_TAG, authority.key().as_ref(), &[tour_idx].as_ref()],
        bump,
        has_one = authority,
    )]
    pub tour_account: Box<Account<'info, TourAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}