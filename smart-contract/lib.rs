// Importation des fonctionnalités de base d'Anchor qui facilitent le développement sur Solana.
use anchor_lang::prelude::*;
// Importation des modules nécessaires pour travailler avec les tokens SPL, notamment pour la création d'un Mint.
use anchor_spl::token::{self, Mint};

// Déclaration de l'ID du programme, qui sera utilisé pour identifier ce smart contract sur la blockchain Solana.
declare_id!("9ZamzZ8816xXW3RG2bejfu8zu5LEPSs7dQ4V342JeVF8");

// Définition d'une constante pour la taille maximale autorisée du champ `song_id`.
// Cela permet de s'assurer que la chaîne ne dépasse pas une taille déterminée.
const MAX_SONG_ID_LEN: usize = 64;

#[program] // Indique à Anchor que le module suivant contient les instructions du programme.
pub mod hello_anchor {
    // On importe tout ce qui se trouve dans le module parent.
    use super::*;

    /// Fonction d'initialisation du compte `Fees`.
    /// Elle reçoit en paramètres le contexte (`ctx`) qui contient les comptes nécessaires et la valeur initiale des frais (`_fees`).
    pub fn initialize(ctx: Context<Initialize>, _fees: u64) -> Result<()> {
        // On récupère une référence mutable vers le compte `fees` contenu dans le contexte.
        let fees_account = &mut ctx.accounts.fees;
        // On assigne la valeur initiale des frais au champ `value` du compte `Fees`.
        fees_account.value = _fees;
        // On assigne le propriétaire du compte Fees avec l'adresse du signataire (celui qui initie la transaction).
        fees_account.owner = ctx.accounts.owner.key();
        // Affichage d'un message dans les logs de la transaction pour confirmer l'initialisation avec la valeur et le propriétaire.
        msg!(
            "Initialized fees to: {} by owner: {}!",
            _fees,
            fees_account.owner
        );
        // Retourne Ok pour indiquer que l'exécution s'est terminée avec succès.
        Ok(())
    }

    /// Fonction permettant de mettre à jour la valeur des frais.
    /// Seul le propriétaire enregistré dans le compte `Fees` peut modifier cette valeur.
    pub fn set_fees(ctx: Context<SetFees>, new_fees: u64) -> Result<()> {
        // Récupération mutable du compte Fees depuis le contexte.
        let fees_account = &mut ctx.accounts.fees;
        // Vérification que le signataire (owner) est bien le propriétaire enregistré dans le compte Fees.
        if fees_account.owner != ctx.accounts.owner.key() {
            // Si le signataire n'est pas le propriétaire, renvoie une erreur `IllegalOwner`.
            return Err(ProgramError::IllegalOwner.into());
        }
        // Mise à jour du champ `value` avec la nouvelle valeur des frais.
        fees_account.value = new_fees;
        // Affichage d'un message dans les logs pour indiquer que la mise à jour a bien été effectuée par le propriétaire.
        msg!(
            "Fees updated by owner {} to {}.",
            ctx.accounts.owner.key(),
            new_fees
        );
        // Retourne Ok pour signaler le succès de l'opération.
        Ok(())
    }

    /// Fonction qui crée un coffre-fort (vault) pour l'artiste.
    /// Le vault est une adresse dérivée (PDA) qui servira notamment pour gérer les droits d'auteur.
    pub fn create_vault(ctx: Context<CreateVault>) -> Result<()> {
        // Récupération mutable du compte Vault depuis le contexte.
        let vault = &mut ctx.accounts.vault;
        // Initialisation du champ `artist` dans le compte Vault avec l'adresse du signataire (l'artiste).
        vault.artist = ctx.accounts.artist.key();
        // Affichage d'un message de confirmation indiquant que le Vault a été créé pour l'artiste.
        msg!("Vault created for artist: {}", vault.artist);
        // Retourne Ok pour signaler que l'opération s'est bien déroulée.
        Ok(())
    }

    /// Fonction qui crée un compte de droits d'auteur (copyright) pour une œuvre.
    /// Elle crée également deux comptes Mint pour les tokens associés aux droits de distribution et de modification.
    ///
    /// Paramètres :
    /// - `song_id`: L'identifiant de l'œuvre (chaîne de caractères).
    /// - `price`: Le prix des droits d'auteur.
    pub fn create_copyright(
        ctx: Context<CreateCopyright>,
        song_id: String,
        price: u64,
    ) -> Result<()> {
        // Vérification que la longueur de la chaîne `song_id` ne dépasse pas la constante MAX_SONG_ID_LEN.
        if song_id.len() > MAX_SONG_ID_LEN {
            // Si c'est le cas, retourne une erreur personnalisée.
            return Err(ErrorCode::SongIdTooLong.into());
        }

        // Récupération mutable du compte de droits d'auteur.
        let copyright = &mut ctx.accounts.copyright;
        // Stocke l'adresse de l'artiste qui crée le droit d'auteur.
        copyright.artist = ctx.accounts.artist.key();
        // Stocke l'identifiant de la chanson dans le compte.
        copyright.song_id = song_id;
        // Stocke le prix associé aux droits d'auteur.
        copyright.price = price;
        // Stocke l'adresse du Vault (PDA) qui est lié à l'artiste.
        copyright.vault = ctx.accounts.vault.key();
        // Stocke l'adresse du Mint pour le token de distribution dans le compte.
        copyright.distribution_token = ctx.accounts.distribution_token_mint.key();
        // Stocke l'adresse du Mint pour le token de modification dans le compte.
        copyright.modification_token = ctx.accounts.modification_token_mint.key();
        // Affiche un message de confirmation dans les logs indiquant que les droits d'auteur ont été créés.
        msg!(
            "Created copyright for song: {} with price: {} by artist: {}",
            copyright.song_id,
            price,
            copyright.artist
        );
        // Retourne Ok pour signaler la réussite de l'opération.
        Ok(())
    }
}

/////////////////////////////////////////////////////
// Déclaration des contextes (structs) pour chaque instruction
/////////////////////////////////////////////////////

/// Contexte pour l'initialisation du compte Fees.
#[derive(Accounts)]
pub struct Initialize<'info> {
    // Le compte Fees à créer.
    // - `init` indique que le compte sera initialisé.
    // - `payer = owner` signifie que le compte `owner` paiera pour cette initialisation.
    // - `space = 8 + 8 + 32` réserve l'espace nécessaire : 8 octets pour le discriminant, 8 pour le u64 et 32 pour le Pubkey.
    #[account(init, payer = owner, space = 8 + 8 + 32)]
    pub fees: Account<'info, Fees>,
    // Le signataire qui initie l'opération et paie pour la création du compte.
    #[account(mut)]
    pub owner: Signer<'info>,
    // Référence au programme système de Solana, nécessaire pour créer des comptes.
    pub system_program: Program<'info, System>,
}

/// Contexte pour la mise à jour des frais.
#[derive(Accounts)]
pub struct SetFees<'info> {
    // Compte Fees à mettre à jour (doit être mutable).
    #[account(mut)]
    pub fees: Account<'info, Fees>,
    // Le signataire qui tente de mettre à jour les frais (doit être le propriétaire).
    pub owner: Signer<'info>,
}

/// Définition du compte Fees qui stocke les frais et le propriétaire.
#[account]
pub struct Fees {
    // Champ qui stocke la valeur des frais.
    pub value: u64,
    // Champ qui stocke l'adresse du propriétaire du compte.
    pub owner: Pubkey,
}

/// Contexte pour la création du Vault pour un artiste.
#[derive(Accounts)]
pub struct CreateVault<'info> {
    // Création d'un compte Vault en tant que PDA.
    // - `seeds = [b"vault", artist.key().as_ref()]` définit les seeds utilisés pour générer l'adresse.
    // - `bump` permet d'obtenir le bump associé à cette PDA.
    // - `payer = artist` signifie que l'artiste paie pour la création.
    // - `space = 8 + 32` réserve l'espace : 8 octets pour le discriminant et 32 pour le Pubkey de l'artiste.
    #[account(init, seeds = [b"vault", artist.key().as_ref()], bump, payer = artist, space = 8 + 32)]
    pub vault: Account<'info, Vault>,
    // Le signataire qui est l'artiste (et qui paie pour la création du Vault).
    #[account(mut)]
    pub artist: Signer<'info>,
    // Référence au programme système de Solana.
    pub system_program: Program<'info, System>,
}

/// Contexte pour la création d'un compte de droits d'auteur (copyright).
#[derive(Accounts)]
pub struct CreateCopyright<'info> {
    // Le signataire qui est l'artiste et qui crée les droits d'auteur.
    #[account(mut)]
    pub artist: Signer<'info>,
    // Vérification du Vault (PDA) associé à l'artiste.
    // La seed utilisée ici est la même que celle de `create_vault` pour garantir la cohérence.
    #[account(mut, seeds = [b"vault", artist.key().as_ref()], bump)]
    pub vault: Account<'info, Vault>,

    // Création d'un compte pour stocker les informations sur les droits d'auteur.
    // La taille allouée est calculée comme suit :
    //  - 8 octets pour le discriminant
    //  - 32 octets pour le Pubkey de l'artiste
    //  - 4 octets pour le préfixe de longueur du String + MAX_SONG_ID_LEN octets pour le `song_id`
    //  - 8 octets pour le prix (u64)
    //  - 32 octets pour le Pubkey du token de distribution
    //  - 32 octets pour le Pubkey du token de modification
    #[account(init, payer = artist, space = 8 + 32 + (4 + MAX_SONG_ID_LEN) + 8 + 32 + 32)]
    pub copyright: Account<'info, CopyrightAccount>,

    // Création du compte Mint pour le token de distribution.
    // - `mint::decimals = 0` indique qu'il n'y a pas de décimales.
    // - `mint::authority = artist` fixe l'artiste comme autorité pour émettre le token.
    #[account(init, payer = artist, mint::decimals = 0, mint::authority = artist)]
    pub distribution_token_mint: Account<'info, Mint>,

    // Création du compte Mint pour le token de modification, avec les mêmes paramètres.
    #[account(init, payer = artist, mint::decimals = 0, mint::authority = artist)]
    pub modification_token_mint: Account<'info, Mint>,

    // Référence au programme système, nécessaire pour la création de comptes.
    pub system_program: Program<'info, System>,
    // Sysvar contenant les informations sur le loyer (rent) pour la création de comptes.
    pub rent: Sysvar<'info, Rent>,
    // Référence au programme Token SPL, nécessaire pour gérer les tokens.
    pub token_program: Program<'info, token::Token>,
}

/// Définition du compte Vault qui stocke l'adresse de l'artiste propriétaire du coffre-fort.
#[account]
pub struct Vault {
    // Champ qui stocke l'adresse (Pubkey) de l'artiste.
    pub artist: Pubkey,
}

/// Définition du compte qui stocke les informations liées aux droits d'auteur.
#[account]
pub struct CopyrightAccount {
    // Adresse de l'artiste qui détient les droits.
    pub artist: Pubkey,
    // Adresse du Vault associé à cet artiste.
    pub vault: Pubkey,
    // Identifiant de l'œuvre (chanson) sous forme de chaîne de caractères.
    pub song_id: String,
    // Prix des droits d'auteur.
    pub price: u64,
    // Adresse du Mint pour le token de distribution.
    pub distribution_token: Pubkey,
    // Adresse du Mint pour le token de modification.
    pub modification_token: Pubkey,
}

/// Définition d'un enum pour les erreurs personnalisées du programme.
#[error_code]
pub enum ErrorCode {
    // Erreur retournée lorsque la longueur du `song_id` dépasse la taille maximale autorisée.
    #[msg("La longueur du song_id dépasse la limite autorisée.")]
    SongIdTooLong,
}
