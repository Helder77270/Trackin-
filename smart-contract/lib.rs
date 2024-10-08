use anchor_lang::prelude::*;
// use anchor_spl::token::TokenAccount;
use anchor_lang::AccountSerialize;
declare_id!("9ZamzZ8816xXW3RG2bejfu8zu5LEPSs7dQ4V342JeVF8");

#[program]
mod hello_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, _fees: u64) -> Result<()> {
        ctx.accounts.fees.value = _fees; // Accède au compte `new_account` et initialise le champ `data` avec la valeur fournie.
        msg!("Changed fees to: {}!", _fees); // Affiche un message dans les logs de la transaction pour confirmer la modification.
        Ok(())
    }

     pub fn set_fees(ctx: Context<SetFees>, new_fees: u64) -> Result<()> {
        // Vérifie que le signataire est bien le propriétaire du compte de frais
        if ctx.accounts.fees.owner != ctx.accounts.owner.key() {
            return Err(ProgramError::IllegalOwner.into());
        }

        // Mise à jour des frais
        ctx.accounts.fees.value = new_fees;
        msg!("Fees updated by owner: {} to {}", ctx.accounts.owner.key(), new_fees);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = 8 + 32)]  // 8 bytes pour le discriminant, 32 bytes pour le Pubkey
    pub fees: Account<'info, Fees>,  // Compte de frais
    #[account(mut)]
    pub owner: Signer<'info>,  // Le signataire de la transaction
    pub system_program: Program<'info, System>,  // Programme système
}

#[derive(Accounts)]
pub struct SetFees<'info> {
    pub owner: Signer<'info>,  // Le signataire de la transaction (celui qui tente de modifier les frais)
    #[account(mut)]
    pub fees: Account<'info, Fees>,  // Le compte des frais à mettre à jour
}

#[account]
pub struct Fees {
    pub value: u64,  // Valeur des frais
    pub owner: Pubkey,  // Propriétaire du compte de frais
}

#[derive(Accounts)]
pub struct CreatePda<'info> {
    #[account(init, seeds = [b"vault".as_ref()], bump, payer = user, space = 8 + 32)] // Création de la PDA
    pub pda_account: Account<'info, Treasury>, // Compte de la PDA avec les données
    #[account(mut)]
    pub user: Signer<'info>, // Le signataire qui paie la transaction
    pub system_program: Program<'info, System>, // Programme système Solana
}

#[account]
pub struct Treasury {
    pub money: u64, // Nombre de tokens dans le compte
}