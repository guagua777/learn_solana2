// 先在 Cargo.toml 加依赖
// [dependencies]
// solana-sdk = "1.17"

use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
};

use solana_system_interface::instruction::transfer;

use solana_sdk::{
    native_token::LAMPORTS_PER_SOL,
};

fn main() {
    // 1. 造两个测试钱包
    let from = Keypair::new();
    let to = Keypair::new();
    let amount = 1_000_000_000; // 1 SOL

    // 2. 调用你好奇的 transfer 函数
    let transfer_ix = transfer(
        &from.pubkey(),
        &to.pubkey(),
        amount,
    );

    // ====================== 开始打印！======================
    println!("=== 【完整 Instruction 真实结构】===\n");

    // 1. 打印合约地址（program_id）
    println!("1. program_id（合约地址）:");
    println!("   {}", transfer_ix.program_id);
    println!("   固定是系统合约地址：1111...1111\n");

    // 2. 打印账户列表（accounts）
    println!("2. accounts（账户列表）:");
    for (i, acc) in transfer_ix.accounts.iter().enumerate() {
        println!("   账户{}: pubkey={}, is_signer={}, is_writable={}",
            i, acc.pubkey, acc.is_signer, acc.is_writable
        );
    }
    println!();

    // 3. 打印指令数据（data 字节数组）
    println!("3. data（原始字节数组 &[u8]）:");
    println!("   {:?}\n", transfer_ix.data);

    // 在代码中使用
    if let Some(amount) = parse_transfer_instruction_data(&transfer_ix.data) {
        println!("   Transfer amount: {} lamports ({} SOL)", 
                amount, 
                amount as f64 / LAMPORTS_PER_SOL as f64);
    } else {
        println!("   Failed to parse transfer instruction data");
    }

    println!("=== 结论：完全符合 合约地址+账户列表+数据 格式！===");
}


// 解析系统转账指令的数据
fn parse_transfer_instruction_data(data: &[u8]) -> Option<u64> {
    // 系统转账指令的数据格式：
    // 第1字节: 指令类型 (0x02 表示转账)
    // 第2-9字节: 转账金额 (little-endian 格式)
    
    if data.len() < 9 {
        return None;
    }
    
    // 检查指令类型是否为转账
    if data[0] != 2 {
        return None;
    }
    
    // 解析转账金额
    let amount = u64::from_le_bytes(data[1..9].try_into().unwrap());
    Some(amount)
}

