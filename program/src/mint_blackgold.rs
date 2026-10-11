use blackgold_mint_api::prelude::*;
use spl_token::amount_to_ui_amount;
use steel::*;

/// Mints Token to the treasury.
pub fn process_mint_token(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse data. (make mint token struct)
    let args = MintToken::try_from_bytes(data)?;

    let amount = u64::from_le_bytes(args.amount);

    // Load accounts.
    let clock = Clock::get()?;

    let [signer_info, authority_info, mint_info, to_info, token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    //* Signer account (treasury)
    signer_info.is_signer()?.has_address(&TREASURY_ADDRESS)?;

    //* Mint authority
    let mint_authority = authority_info.as_account_mut::<MintAuthority>(&blackgold_mint_api::ID)?;

    //* Mint address
    let mint = mint_info.has_address(&MINT_ADDRESS)?.as_mint()?;

    //* ATA PDA (destination)
    to_info.as_associated_token_account(&TREASURY_ADDRESS, &MINT_ADDRESS)?;

    //* Token program (spl token program)
    token_program.is_program(&spl_token::ID)?;

    //* Amount is in qty
    // Check max mint amount.
    //* If to be minted amount is greater than MAX LIMIT OF MINT amount throw error
    if amount > MAX_MINT_AMOUNT {
        return Err(trace(
            format!(
                "Cannot mint more than max allowed: {} > {}",
                amount_to_ui_amount(amount, TOKEN_DECIMALS),
                amount_to_ui_amount(MAX_MINT_AMOUNT, TOKEN_DECIMALS)
            )
            .as_str(),
            BlackgoldMintError::MaxAmountExceeded.into(),
        ));
    }

    // Check remaining max supply. By checking current supply and substracting it with max supply
    let remaining_supply = MAX_SUPPLY.saturating_sub(mint.supply());

    //* If to be minted amount is greater than remaining amount throw Error
    if amount > remaining_supply {
        return Err(trace(
            format!(
                "Cannot mint more than remaining supply: {} > {}",
                amount_to_ui_amount(amount, TOKEN_DECIMALS),
                amount_to_ui_amount(remaining_supply, TOKEN_DECIMALS)
            )
            .as_str(),
            BlackgoldMintError::MaxSupplyExceeded.into(),
        ));
    }

    // Check mint frequency.
    let slots_since = clock.slot.saturating_sub(mint_authority.last_mint_at);

    //* Check whether the minimum gap between the last mint time and current request time is OKAY - MIN_SLOTS_BETWEEN_MINT
    if slots_since < MIN_SLOTS_BETWEEN_MINT {
        return Err(trace(
            format!(
                "Must wait at least {} slots between mints: {} slots remaining",
                MIN_SLOTS_BETWEEN_MINT,
                MIN_SLOTS_BETWEEN_MINT.saturating_sub(slots_since)
            )
            .as_str(),
            BlackgoldMintError::MintFrequencyExceeded.into(),
        ));
    }

    // Update timestamp.
    //* Update last mint at with current slot
    mint_authority.last_mint_at = clock.slot;

    // Mint tokens to the treasury.
    mint_to_signed(
        mint_info,
        to_info,
        authority_info,
        token_program,
        amount,
        &[MINT_AUTHORITY],
    )?;

    Ok(())
}
