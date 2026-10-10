use steel::*;

use crate::{
    consts::{MINT_ADDRESS, TREASURY_ADDRESS},
    instruction::*,
    state::*,
};

//^ Signer
// Why is signer needed?
// Because creating a PDA requires a payer
// And paying rent requires a real wallet
// PDAs cannot sign or pay fees

// signer = your treasury wallet
// signer pays rent for the MintAuthority PDA
// signer signs the transaction
// signer approves the creation of the PDA

//^ MintAuthority PDA.
// PDA(["authority"], program_id)
// Created using mint PDA seed and Program ID
// the mint authority of BLKG
// the signer for minting
// the enforcer of mint rules
// the owner of mint logic
// This PDA has no private key, so your program signs on its behalf using invoke_signed.

/*
INIT INSTRUCTION — QUICK REVISION NOTES

1. Solana programs CANNOT access accounts unless they are passed in the
   instruction. They cannot fetch, search, or hardcode usable accounts.

2. Signer MUST be passed because:
   - Only a real wallet can sign the transaction.
   - Only a real wallet can pay rent for PDA creation.
   - Programs cannot sign or pay fees.

3. Mint Authority PDA MUST be passed because:
   - PDAs cannot be created or initialized unless included in accounts[].
   - Program needs writable access to initialize the PDA.
   - Even though PDA address can be computed, the PDA account itself must be passed.

4. System Program MUST be passed because:
   - PDA creation requires invoking the system program.
   - Solana requires ALL invoked programs to be explicitly passed.

5. Hardcoding addresses inside the program is USELESS:
   - Program cannot open or use accounts unless they are passed.
   - A hardcoded pubkey is just a number, not an account.

6. The SDK builds the account list and passes it to the program.
   This is the ONLY way the program can see and use accounts.

Summary:
→ Signer = pays + signs
→ MintAuthority PDA = created + initialized
→ System Program = required for PDA creation
→ All accounts MUST be passed from SDK; programs cannot fetch them.
*/

/*
true  = this account MUST sign the transaction
false = this account does NOT sign the transaction

& - Only real wallets can be signers (true).
& - PDAs can NEVER sign (always false).
& - Programs (system program, token program) never sign (always false).

^ In init():
AccountMeta::new(signer, true)
* → Treasury wallet signs and pays rent.

AccountMeta::new(mint_authority_pda, false)
* → PDA is passed but does NOT sign.

AccountMeta::new_readonly(system_program::ID, false)
* → System program is invoked but does NOT sign.
*/

pub fn init(signer: Pubkey) -> Instruction {
    //* Generating mint authority address using mint authority PDA seed and Program ID
    let mint_authority_address = authority_pda().0;

    //1 =======
    // signer — Why is signer needed?
    // AccountMeta::new(signer, true)
    // signer = your treasury wallet
    // signer pays rent for the MintAuthority PDA
    // signer signs the transaction
    // signer approves the creation of the PDA

    // 2 ======
    // AccountMeta::new(authority_address, false) — Why is this passed?
    // Because your init() instruction must:
    //  create the MintAuthority PDA
    //  initialize it
    //  assign it as mint authority
    // Your on-chain process_init() will:
    // create the PDA
    // set it as mint authority of BLKG
    // store any initial state (if needed)
    // So the PDA must be passed as an account.

    // 3 ======
    // AccountMeta::new_readonly(system_program::ID, false) — Why system program?
    // Because creating a PDA requires:
    //  system_program::create_account
    //  paying rent
    //  allocating space
    //  assigning ownership
    // The system program performs these actions.
    // So you must include it.

    // Init {}.to_bytes() — What does this do?
    // This serializes your instruction:
    // discriminator = 0 (Init)
    // no additional data
    // Your on-chain program receives:
    // [0]
    // And routes it to:
    // process_init(accounts, data)
    Instruction {
        program_id: crate::ID, //* Id of the program
        accounts: vec![
            AccountMeta::new(signer, true), //* Signer of this transaction (isSigner: true) -- the passing of true ==> true: This account MUST SIGN the transaction
            AccountMeta::new(mint_authority_address, false), //* Mint authority PDA to be created using this initialisation -- false: This account does NOT sign It is just passed as an account.
            // It can be writable or read-only.
            // PDAs are always false because they cannot sign.
            // Programs (system program, token program) are always false.
            // PDA does NOT sign
            // System program does NOT sign
            // SPL Token program does NOT sign
            AccountMeta::new_readonly(system_program::ID, false), //* System program to initilisation of mint authority PDA creation and transferring mint authority of the mint to newly created mint authority PDA
        ],
        data: Init {}.to_bytes(),
    }
}

//^ Treasury wallet
// Signs the mint request
//* (“I approve this mint.”)
// Treasury signs ==> because we decided mint requests must be approved

//^ MintAuthority PDA
// Executes the mint
//* (“I am authorized to mint.”)
// MintAuthority PDA mints ==> because it is the mint authority

//^ Mint account
// The BLKG token mint
//* (“This is the token definition.”)
// Mint is the token itself ==> (address of token BLKG)

//^ ATA (to_address)
// Where minted tokens go, a account areated from reference of both treasury and mint token, to create an account to store tokens into. this wallet owns this token and has an address to reference both
//* (“This is the treasury’s BLKG balance.”)
// ATA is always the same ==> because treasury always receives minted tokens
// ATA is calculated dynamically ==> for safety and correctness(We can hardcode it if needed but not need as its calculated on the fly no RPC needed)

//^ SPL Token Program
// The actual minting engine
//* (“I perform the mint.”)
// SPL Token Program ==> performs the mint
pub fn mint_blackgold(amount: u64) -> Instruction {
    //* Signer -- The treasury address to sign the mint request
    let signer_address = TREASURY_ADDRESS;

    //* To address -- to which the minted token to be saved. Treasury wallet and mint addres are used to calculate this PDA. its not our PDA it is spl associated accopunt pda
    //* It is generated for every signer (generator/signer) wallet and mint token. In our case the signer will be the same and the mint will be the same to all minted tokes will be saved to a same PDA
    //* It is a PDA owned by the SPL Associated Token Account Program
    //* ATA is deterministic -  ATA is a special account created from both addresses
    // So:
    // owner = TREASURY_ADDRESS
    // mint = MINT_ADDRESS
    // program_id = spl_associated_token_account::ID
    // It must be unique per (wallet, mint)
    // It must be controlled by the ATA program
    // ATA(TREASURY_ADDRESS, MINT_ADDRESS)
    // Every single minted BLKG goes into ATA (SPL Associated Token Account)
    // It is the single account that holds:
    // all minted BLKG
    // all future BLKG
    // all supply before distribution
    // all reserve tokens
    // PDA(["owner", "token_program", "mint"], associated_token_program)
    // ATA is a “special account” created from both addresses
    // ATA is the place where minted tokens are stored
    // ATA is NOT the wallet itself:  Wallet = can hold SOL, ATA = can hold SPL tokens (BLKG)
    // Wallet cannot hold tokens directly.
    // ATA is required.
    let to_address =
        spl_associated_token_account::get_associated_token_address(&signer_address, &MINT_ADDRESS); //* HOW THIS IS CALCULATED? AND WHAT IS THIS?

    //* Mint Authority Address -- address of account authoried to mint the token
    let mint_authority_address = authority_pda().0;

    //* Instruction of mint
    Instruction {
        program_id: crate::ID, //* ID of program
        accounts: vec![
            AccountMeta::new(signer_address, true),          //* Treasury
            AccountMeta::new(mint_authority_address, false), //* Mint Authority
            AccountMeta::new(MINT_ADDRESS, false),           //* Mint (token)
            AccountMeta::new(to_address, false), //* (Treasury Wallet) ↔ (BLKG Mint): “This wallet owns this token mint.”, determininstic address ATA PDA to store the minted token
            AccountMeta::new_readonly(spl_token::ID, false), //* the program that actually mints tokens.
        ],
        data: MintBlackgold {
            amount: amount.to_le_bytes(), //* Amount to mint
        }
        .to_bytes(),
    }
}
