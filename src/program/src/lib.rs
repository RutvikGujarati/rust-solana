use solana_program::{
    account_info:: AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::PubKey, //  TODO: Replace with a proper Pubkey struct.
};

entrypoint!(process_instruction);// program's entry point

fn process_instruction(
    program_id:  & PubKey, // Public key of the invoking program.
    accounts:     & [AccountInfo],   // The transaction's input and output accounts
    instruction_data: &[u8],
) -> ProgramResult{
    msg!("Hi there from rust");
    OK(())
}