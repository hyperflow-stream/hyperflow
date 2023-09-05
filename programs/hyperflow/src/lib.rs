use anchor_lang::prelude::*;

declare_id!("6khbWKLNioUkCUhEK22oZVd1sa1YKby1biTv5wWY2eo");

#[program]
pub mod hyperflow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
