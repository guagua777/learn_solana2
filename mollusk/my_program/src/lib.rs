use solana_account_info::{next_account_info, AccountInfo};
use solana_program_error::ProgramError;
use solana_pubkey::Pubkey;
use solana_msg::msg;
use solana_program_entrypoint::{entrypoint, ProgramResult};


entrypoint!(process_instruction);


pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello, Solana!");
    let accounts_iter = &mut accounts.iter();

    let first_account = next_account_info(accounts_iter)?;

    if !first_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut data = first_account.try_borrow_mut_data().unwrap();
    data[0] ^= 1;

    Ok(())
}