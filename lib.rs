use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

declare_id!("Dub4Lj63Fqra4JDNoqjo6inzLUW6q8fHZ9fRpsaTu8Vi");

#[program]
mod hello_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, _fees: u64) -> Result<()> {
        ctx.accounts.fees.value = _fees; // Accède au compte `new_account` et initialise le champ `data` avec la valeur fournie.
        msg!("Changed fees to: {}!", fees); // Affiche un message dans les logs de la transaction pour confirmer la modification.
        Ok(())
    }

    pub fn set_fees(ctx: Context<SetFees>, newFees: u64) -> Result<()> {
        if ctx.accounts.token_account.amount > 0 {
            ctx.accounts.my_account.data = data;
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // We must specify the space in order to initialize an account.
    // First 8 bytes are default account discriminator,
    // next 8 bytes come from NewAccount.data being type u64.
    // (u64 = 64 bits unsigned integer = 8 bytes)
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub fees: Account<'info, Fees>,

}

#[account]
pub struct Fees {
    value: u64,
}
#[derive(Accounts)]
pub struct SetFees <'info> {
    pub owner: Account<'info, Signer>,
    #[account(mut)]
    pub fees: Account<'info, Fees>

}
