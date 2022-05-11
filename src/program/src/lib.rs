use processor::process_instruction;
use solana_program::entrypoint;
mod commands;
mod convert;
mod error;
mod processor;
mod state;

entrypoint!(process_instruction);
