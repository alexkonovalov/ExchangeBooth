use processor::process_instruction;
use solana_program::entrypoint;
mod commands;
mod error;
mod helpers;
mod processor;
mod state;

entrypoint!(process_instruction);
