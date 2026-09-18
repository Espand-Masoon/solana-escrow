use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct NewTrade<'info> {
    #[account(init, payer=party_1, space= 8 + Trade::INIT_SPACE, seeds=[party_1.key().as_ref(), party_2.key().as_ref()], bump)]
    trade: Account<'info, Trade>,
    #[account(mut)]
    party_1: Signer<'info>,
    party_2: SystemAccount<'info>,
    system_program: Program<'info, System>,
}

pub fn create(ctx: Context<NewTrade>) -> Result<()> {
    msg!("New trade created. Address: {:?}", ctx.accounts.trade.key());
    msg!("Party 1: {:?}", ctx.accounts.party_1);
    msg!("Party 2: {:?}", ctx.accounts.party_2);
    Ok(())
}

