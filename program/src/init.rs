use blackgold_mint_api::prelude::*;
use steel::*;

/// Initializes the mint authority account.
pub fn process_init(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let clock = Clock::get()?;

    //* Destructure the accounts array into the expected 3 accounts.
    //* If fewer accounts are passed, return NotEnoughAccountKeys.
    let [signer_info, authority_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    //* Treasury Account (Signer)
    //* Must be a real signer because:
    //* - Only a wallet can pay rent for PDA creation.
    //* - Only a wallet can authorize creation of new accounts.
    //* - Programs and PDAs cannot sign.
    signer_info.is_signer()?;

    //* Mint Authority PDA Account
    //* Validations:
    //* 1. is_empty() → PDA must not already exist (we are creating it now).
    //* 2. is_writable() → PDA must be writable so we can allocate and initialize it.
    //* 3. has_seeds() → Confirms this account is the correct PDA derived from
    //*    [MINT_AUTHORITY] and owned by this program.
    //*    Prevents malicious accounts from being passed in place of the PDA.
    authority_info
        .is_empty()?
        .is_writable()?
        .has_seeds(&[MINT_AUTHORITY], &blackgold_mint_api::ID)?;

    //* System Program
    //* Must be the real system program because:
    //* - Only system program can create new accounts.
    //* - PDA creation requires invoking system_program::create_account.
    //* - Prevents attackers from passing a fake program ID.
    system_program.is_program(&system_program::ID)?;

    /*
    Solana automatically persists account changes.

    - create_program_account allocates and assigns the PDA.
    - as_account_mut gives a mutable reference to the PDA's data buffer.
    - Writing to the struct directly modifies the account's memory.
    - When the instruction returns Ok(()), the runtime commits all changes.

    No explicit "save" or "commit" is required. Account mutation is automatic.
    */

    // This function:
    //
    // creates the PDA
    //
    // allocates space
    //
    // assigns your program as owner
    //
    // initializes the account’s data buffer with zeros
    //
    // returns control back to your program

    // At this point, the PDA exists and is writable.
    // Initialize authority account.
    create_program_account::<MintAuthority>(
        authority_info,
        system_program,
        signer_info,
        &blackgold_mint_api::ID,
        &[MINT_AUTHORITY],
    )?;

    // This gives you a mutable reference to the PDA’s data buffer.
    // This is the key point:
    // You are now directly modifying the PDA’s memory.
    // No commit needed.
    let mint_authority = authority_info.as_account_mut::<MintAuthority>(&blackgold_mint_api::ID)?;

    // This writes directly into the PDA’s data buffer.
    // There is no intermediate copy.
    // There is no staging area.
    // There is no transaction log.
    // You are writing directly into the account’s memory. (Buffer)
    mint_authority.last_mint_at = clock.slot;

    // Instruction ends → Solana runtime persists changes
    // The Solana runtime:
    // takes the modified account buffers
    // writes them back to the ledger
    // finalizes the transaction
    // commits the updated account state
    Ok(())
}
