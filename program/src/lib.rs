use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    pubkey::Pubkey,
};

entrypoint!(method_commands);

fn method_commands(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Iterating over accounts is safer than indexing them
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hi
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Account does not have the correct Program ID");
        return Err(ProgramError::IncorrectProgramId);
    }
    
    msg!("Hi, Solana!");

    msg!("Account: {:#?}", account);

    msg!("Adding 1 to counter");

    let mut counter = Counter::try_from_slice(&account.data.borrow())?;
    counter.count += 1;
    counter.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Counter is at: {}", counter.count);

    Ok(())
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Counter {
    pub count: u32,
}