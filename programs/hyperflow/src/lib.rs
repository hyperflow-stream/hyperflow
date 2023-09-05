use anchor_lang::prelude::*;
use instructions::*;
use state::*;

declare_id!("6khbWKLNioUkCUhEK22oZVd1sa1YKby1biTv5wWY2eo");

pub mod instructions;
pub mod state;

#[program]
pub mod hyperflow {
    use super::*;

    pub fn set_up_token_balance(
        ctx: Context<SetTokenBalance>,
        amount: u64,
    ) -> Result<()> {
        print!("Setting up token balance");
        hyperflow::balance::set_up_token_balance(ctx, amount)
    }

    pub fn reset_delegation(
        ctx: Context<ResetDelegation>,
        amount: u64,
    ) -> Result<()> {
        print!("Resetting delegation");
        hyperflow::balance::reset_delegation(ctx, amount)
    }

    pub fn setup_stream_aggregate (
        ctx: Context<SetupStreamAggregate>,
        payee: Pubkey,
        per_second_rate: u64,
    ) -> Result<()> {
        print!("Setting up stream aggregate");
        hyperflow::stream::setup_stream_aggregate(ctx, payee, per_second_rate)
    }

    pub fn add_stream (
        ctx: Context<AddStream>,
    ) -> Result<()> {
        print!("Adding stream");
        hyperflow::stream::add_stream(ctx)
    }

    pub fn withdraw_from_stream (
        ctx: Context<WithdrawFromStream>,
        amount: u64,
    ) -> Result<()> {
        print!("Withdrawing from stream");
        hyperflow::transact::withdraw_from_stream(ctx, amount)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
