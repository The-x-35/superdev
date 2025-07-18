use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

// Test functions for seeds
fn get_seed1() -> &'static [u8] {
    b"prefix"
}

fn get_seed2(authority: &Pubkey) -> &[u8] {
    authority.as_ref()
}

#[program]
mod test_seeds {
    use super::*;

    pub fn initialize_square_brackets(ctx: Context<InitializeSquareBrackets>, start: u64) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.authority = *ctx.accounts.authority.key;
        counter.count = start;
        Ok(())
    }

    pub fn initialize_parentheses(ctx: Context<InitializeParentheses>, start: u64) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.authority = *ctx.accounts.authority.key;
        counter.count = start;
        Ok(())
    }

    pub fn initialize_curly_braces(ctx: Context<InitializeCurlyBraces>, start: u64) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.authority = *ctx.accounts.authority.key;
        counter.count = start;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        Ok(())
    }
}

// Test 1: Square brackets []
#[derive(Accounts)]
pub struct InitializeSquareBrackets<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 48,
        seeds = [get_seed1(), get_seed2(&authority.key())],
        bump
    )]
    pub counter: Account<'info, Counter>,

    pub system_program: Program<'info, System>,
}

// Test 2: Parentheses ()
#[derive(Accounts)]
pub struct InitializeParentheses<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 48,
        seeds = (get_seed1(), get_seed2(&authority.key())),
        bump
    )]
    pub counter: Account<'info, Counter>,

    pub system_program: Program<'info, System>,
}

// Test 3: Curly braces {}
#[derive(Accounts)]
pub struct InitializeCurlyBraces<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 48,
        seeds = {get_seed1(), get_seed2(&authority.key())},
        bump
    )]
    pub counter: Account<'info, Counter>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [get_seed1(), get_seed2(&authority.key())],
        bump,
        has_one = authority
    )]
    pub counter: Account<'info, Counter>,
}

#[account]
pub struct Counter {
    pub authority: Pubkey,
    pub count: u64,
}
