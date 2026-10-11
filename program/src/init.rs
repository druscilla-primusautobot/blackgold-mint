use blackgold_mint_api::prelude::*;
use steel::*;

/// Initializes the authority account.
pub fn process_init(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let clock = Clock::get()?;

    let [signer_info, authority_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;

    authority_info
        .is_empty()?
        .is_writable()?
        .has_seeds(&[MINT_AUTHORITY], &blackgold_mint_api::ID)?;

    system_program.is_program(&system_program::ID)?;

    // Initialize authority account.
    create_program_account::<MintAuthority>(
        authority_info,
        system_program,
        signer_info,
        &blackgold_mint_api::ID,
        &[MINT_AUTHORITY],
    )?;

    let mint_authority = authority_info.as_account_mut::<MintAuthority>(&blackgold_mint_api::ID)?;
    mint_authority.last_mint_at = clock.slot;

    Ok(())
}
