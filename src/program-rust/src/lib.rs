// kgs added

use byteorder::{ByteOrder, LittleEndian};
use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint,
  entrypoint::ProgramResult,
  msg,
  program_error::ProgramError,
  pubkey::Pubkey,
};
use std::mem;

entrypoint!(process_instruction);

fn process_instruction(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  _instruction_data: &[u8],
) -> ProgramResult {
  msg!("Helloworld Rust program entrypoint");
  let accounts_iter = &mut accounts.iter();
  let account = next_account_info(accounts_iter)?;
  if account.owner != program_id {
    msg!("Greeted account does not have the correct program id");
    return Err(ProgramError::IncorrectProgramId);
  }

  if account.try_data_len()? < mem::size_of::<u32>() {
    msg!("Greeted account data length too small for u32");
    return Err(ProgramError::InvalidAccountData);
  }

  let mut data = account.try_borrow_mut_data()?;
  let mut num_greets = LittleEndian::read_u32(&data);
  num_greets += 1;
  LittleEndian::write_u32(&mut data[0..], num_greets);
  msg!("Hello!");
  Ok(())
}