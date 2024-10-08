use anchor_lang::prelude::*;
use anchor_lang::AccountSerialize;
use anchor_spl::token::{self, Mint, TokenAccount};
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
        msg!(
            "Fees updated by owner: {} to {}",
            ctx.accounts.owner.key(),
            new_fees
        );
        Ok(())
    }

    // Initialise un coffre-fort pour l'artiste (PDA)
    pub fn create_vault(ctx: Context<CreateVault>) -> Result<()> {
        msg!("Vault created for artist: {}", ctx.accounts.artist.key());
        Ok(())
    }

    // Crée des droits d'auteur pour une œuvre spécifique
    pub fn create_copyright(
        ctx: Context<CreateCopyright>,
        song_id: String,
        price: u64,
    ) -> Result<()> {
        let copyright = &mut ctx.accounts.copyright;
        copyright.artist = ctx.accounts.artist.key(); // On ajoute .key() quand on parle d'adresse, ici celle de l'artiste
        copyright.song_id = song_id;
        copyright.price = price;
        copyright.vault = ctx.accounts.vault.key(); // Ici, celle du PDA (Vault)
        copyright.distribution_token = ctx.accounts.distribution_token_mint.key(); // Et ici, celle des tokens
        copyright.modification_token = ctx.accounts.modification_token_mint.key();
        msg!(
            "Created copyright for song: {} with price: {} by: {}",
            copyright.song_id,
            price,
            copyright.artist
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = 8 + 32)]
    // 8 bytes pour le discriminant, 32 bytes pour le Pubkey
    pub fees: Account<'info, Fees>, // Compte de frais
    #[account(mut)]
    pub owner: Signer<'info>, // Le signataire de la transaction
    pub system_program: Program<'info, System>, // Programme système
}

#[derive(Accounts)]
pub struct SetFees<'info> {
    pub owner: Signer<'info>, // Le signataire de la transaction (celui qui tente de modifier les frais)
    #[account(mut)]
    pub fees: Account<'info, Fees>, // Le compte des frais à mettre à jour
}

#[account]
pub struct Fees {
    pub value: u64,    // Valeur des frais
    pub owner: Pubkey, // Propriétaire du compte de frais
}

#[derive(Accounts)]
pub struct CreatePda<'info> {
    #[account(init, seeds = [b"vault".as_ref()], bump, payer = user, space = 8 + 32)]
    // Création de la PDA
    pub pda_account: Account<'info, Treasury>, // Compte de la PDA avec les données
    #[account(mut)]
    pub user: Signer<'info>, // Le signataire qui paie la transaction
    pub system_program: Program<'info, System>, // Programme système Solana
}

#[account]
pub struct Treasury {
    pub money: u64, // Nombre de tokens dans le compte
}

#[derive(Accounts)]
pub struct CreateVault<'info> {
    #[account(init, seeds = [b"vault", artist.key().as_ref()], bump, payer = artist, space = 8 + 32)]
    pub vault: Account<'info, Vault>, // PDA de l'artiste
    #[account(mut)]
    pub artist: Signer<'info>, // Artiste qui initialise le vault
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateCopyright<'info> {
    #[account(mut)]
    pub artist: Signer<'info>, // Artiste qui veut créer des droits
    #[account(mut, seeds = [b"vault", artist.key().as_ref()], bump)]
    pub vault: Account<'info, Vault>, // PDA de l'artiste

    #[account(init, payer = artist, space = 8 + 32 + 64 + 8 + 32 + 32)]
    pub copyright: Account<'info, CopyrightAccount>, // Compte pour les droits d'auteur

    // Mints pour les tokens de distribution et de modification
    #[account(init, payer = artist, mint::decimals = 0, mint::authority = artist)]
    pub distribution_token_mint: Account<'info, Mint>,

    #[account(init, payer = artist, mint::decimals = 0, mint::authority = artist)]
    pub modification_token_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,

    // Ajout du compte du programme de token SPL
    pub token_program: Program<'info, token::Token>, // Programme SPL Token
}

#[account]
pub struct Vault {
    pub artist: Pubkey, // L'adresse de l'artiste
}

#[account]
pub struct CopyrightAccount {
    pub artist: Pubkey,             // L'adresse de l'artiste qui possède les droits
    pub vault: Pubkey,              // Le vault (PDA) associé à cet artiste
    pub song_id: String,            // L'identifiant de l'œuvre
    pub price: u64,                 // Le prix des droits d'auteur
    pub distribution_token: Pubkey, // Mint du token pour les droits de distribution
    pub modification_token: Pubkey, // Mint du token pour les droits de modification
}
