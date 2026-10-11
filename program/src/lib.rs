mod init;
mod mint_blackgold;

use init::*;
use mint_blackgold::*;

use blackgold_mint_api::instruction::*;
use solana_security_txt::security_txt;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&blackgold_mint_api::ID, program_id, data)?;

    match ix {
        MintInstruction::Init => process_init(accounts, data)?,
        MintInstruction::MintToken => process_mint_blackgold(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);

security_txt! {
    name: "BLKG Mint",
    project_url: "https://blackgold.supply",
    contacts: "email:druscilla2024@gmail.com",
    policy: "https://github.com/regolith-labs/ore-mint/blob/master/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/regolith-labs/ore-mint"
}
