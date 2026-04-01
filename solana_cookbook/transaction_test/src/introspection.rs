use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    instruction::{Instruction, get_instruction_relative, load_instruction_at},
    msg,
    pubkey::Pubkey,
    sysvar::instructions::{self, Instructions},
};

// 程序入口
entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("指令自省示例开始");

    // ====================== 1. 获取指令系统账户（必须）
    // 对应文档：Sysvar: Instructions
    let instruction_sysvar = accounts.last().ok_or(solana_program::program_error::ProgramError::NotEnoughAccountKeys)?;
    
    // 校验账户是否是正确的指令自省系统账户
    instructions::check_id(instruction_sysvar.key)?;

    // ====================== 2. 获取当前指令在交易里的索引
    // 对应文档：load_current_index_checked
    let current_index = Instructions::load_current_index_checked(instruction_sysvar)?;
    msg!("当前指令索引: {}", current_index);

    // ====================== 3. 读取当前指令（自省核心）
    // 对应文档：load_instruction_at_checked
    let current_ix: Instruction = Instructions::load_instruction_at_checked(current_index as usize, instruction_sysvar)?;
    msg!("当前指令目标程序 ID: {}", current_ix.program_id);
    msg!("当前指令数据长度: {}", current_ix.data.len());

    // ====================== 4. 读取上一条指令（遍历交易指令）
    if current_index > 0 {
        let prev_ix = Instructions::load_instruction_at_checked((current_index - 1) as usize, instruction_sysvar)?;
        msg!("上一条指令程序 ID: {}", prev_ix.program_id);
    }

    // ====================== 5. 读取下一条指令
    // 可以循环遍历整个交易的所有指令
    let next_index = current_index + 1;
    if let Ok(next_ix) = Instructions::load_instruction_at_checked(next_index as usize, instruction_sysvar) {
        msg!("下一条指令程序 ID: {}", next_ix.program_id);
    }

    msg!("指令自省示例结束");
    Ok(())
}