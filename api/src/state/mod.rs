// -------------------------------
// mint_authority.rs module
// -------------------------------
//
// This module defines the on‑chain state and PDA used as the
// mint authority for the BlackGold token.
//
// The mint authority PDA is responsible for:
// - enforcing minting rules
// - storing last_mint_at slot
// - acting as the signer for SPL mint CPI via invoke_signed()
// - preventing unauthorized minting
//
// This file contains:
// 1. The BlackgoldAccount enum (account discriminator)
// 2. The PDA derivation function
// 3. Re‑exports for SDK usage
//

mod mint_authority;

pub use mint_authority::*;

use crate::consts::*;
use steel::*;

// ---------------------------------------------
// BlackgoldAccount enum
// ---------------------------------------------
//
// Each on‑chain account type gets a unique discriminator.
// This allows the Steel framework to deserialize accounts
// into the correct Rust struct.
//
// repr(u8) ensures the enum is stored as a single byte.
// MintAuthority = 100 is the discriminator for the PDA.
//
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum BlackgoldAccount {
    MintAuthority = 100,
}

// ---------------------------------------------
// mint_authority_pda()
// ---------------------------------------------
//
// Derives the PDA used as the mint authority.
//
// PDA rules:
// - PDAs have no private key
// - Programs sign for PDAs using invoke_signed()
// - PDAs must be passed from the SDK to be writable
//
// Seeds:
// - MINT_AUTHORITY (constant byte string)
// - Program ID (crate::ID)
//
// Returns:
// (pda_pubkey, bump)
//
pub fn mint_authority_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[MINT_AUTHORITY], &crate::ID)
}
