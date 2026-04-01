// Instruction index for the System Program's transfer instruction
let transfer_instruction_index: u32 = 2;

// Define the amount to transfer
let transfer_amount = LAMPORTS_PER_SOL / 100; // 0.01 SOL

// Create instruction data manually (12 bytes: 4 for u32 index + 8 for u64 lamports)
let mut instruction_data = Vec::with_capacity(12);
instruction_data.extend_from_slice(&transfer_instruction_index.to_le_bytes());
instruction_data.extend_from_slice(&transfer_amount.to_le_bytes());

// Manually create the transfer instruction
let transfer_instruction = Instruction {
    program_id: system_program::id(),
    accounts: vec![
        AccountMeta::new(sender.pubkey(), true), // from account, is signer and is writable
        AccountMeta::new(recipient.pubkey(), false), // to account, is not signer but is writable
    ],
    data: instruction_data,
};