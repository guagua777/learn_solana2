// 导入必要的依赖库
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_compute_budget_interface::ComputeBudgetInstruction;
use solana_sdk::{
    instruction::Instruction, // 交易指令类型
    native_token::LAMPORTS_PER_SOL, // SOL和Lamports之间的转换常量
    pubkey::Pubkey, // 公钥类型
    signature::Keypair, // 密钥对类型
    signer::Signer, // 签名者 trait
    transaction::Transaction, // 交易类型
};
use solana_system_interface::instruction::transfer; // 系统转账指令

/// 模拟交易并返回消耗的计算单元
/// 
/// # 参数
/// - `client`: RPC客户端实例
/// - `instructions`: 要模拟的交易指令列表
/// - `payer_key`: 交易支付者的公钥
/// 
/// # 返回值
/// - 成功时返回模拟消耗的计算单元数量
async fn get_simulation_compute_units(
    client: &RpcClient,
    instructions: &[Instruction],
    payer_key: &Pubkey,
) -> anyhow::Result<u64> {
    // 获取最新的区块哈希
    let recent_blockhash = client.get_latest_blockhash().await?;

    // 为模拟创建计算预算指令
    let simulation_instructions = vec![
        // 设置计算单元价格为100 microLamports
        ComputeBudgetInstruction::set_compute_unit_price(100),
        // 设置较高的计算单元限制，确保模拟能够完成
        ComputeBudgetInstruction::set_compute_unit_limit(400_000),
    ];

    // 组合模拟指令和实际指令
    let mut all_instructions = simulation_instructions;
    all_instructions.extend_from_slice(instructions);

    // 创建模拟交易
    let mut transaction = Transaction::new_with_payer(&all_instructions, Some(payer_key));
    transaction.message.recent_blockhash = recent_blockhash;

    // 模拟交易执行
    match client.simulate_transaction(&transaction).await {
        Ok(simulation) => {
            // 打印模拟结果
            println!("Simulated Transaction: {:#?}", simulation);

            // 检查模拟是否出错
            if let Some(err) = simulation.value.err {
                eprintln!("Simulation error: {:?}", err);
                return Ok(200_000); // 返回默认值作为 fallback
            }

            // 返回消耗的计算单元数量，如果没有则返回默认值
            Ok(simulation.value.units_consumed.unwrap_or(200_000))
        }
        Err(error) => {
            // 处理模拟过程中的错误
            eprintln!("Error during simulation: {}", error);
            Ok(200_000) // 返回默认值作为 fallback
        }
    }
}

/// 根据模拟结果构建优化的交易，包含计算预算指令
/// 
/// # 参数
/// - `client`: RPC客户端实例
/// - `instructions`: 实际交易指令列表
/// - `signer`: 签名者密钥对
/// - `priority_fee`: 优先级费用（以microLamports为单位）
/// 
/// # 返回值
/// - 成功时返回构建好的交易对象
async fn build_optimal_transaction(
    client: &RpcClient,
    instructions: &[Instruction],
    signer: &Keypair,
    priority_fee: u64, // in microLamports
) -> anyhow::Result<Transaction> {
    // 获取支付者公钥
    let payer_pubkey = &signer.pubkey();
    
    // 并行执行两个异步操作：
    // 1. 模拟交易获取计算单元消耗
    // 2. 获取最新的区块哈希
    let compute_units_future = get_simulation_compute_units(client, instructions, payer_pubkey);
    let blockhash_future = client.get_latest_blockhash();

    // 等待两个异步操作完成
    let (compute_units_result, recent_blockhash_result) = 
        tokio::join!(compute_units_future, blockhash_future);

    // 提取模拟的计算单元消耗结果
    // 这里使用 ? 操作符处理可能的错误
    let compute_units = compute_units_result?;
    // 提取最新的区块哈希
    let recent_blockhash = recent_blockhash_result?;

    // 准备最终的交易指令列表
    let mut final_instructions = Vec::new();

    // 添加优先级费用指令
    final_instructions.push(ComputeBudgetInstruction::set_compute_unit_price(
        priority_fee,
    ));

    // 添加计算单元限制，增加10%的余量以确保交易能够成功执行
    let units_with_margin = (compute_units as f64 * 1.1) as u64;
    println!("Compute Units Simulated: {}", compute_units);
    println!("Compute Units with extra 10% margin: {}", units_with_margin);
    final_instructions.push(ComputeBudgetInstruction::set_compute_unit_limit(
        units_with_margin as u32,
    ));

    // 添加实际的交易指令
    final_instructions.extend_from_slice(instructions);

    // 构建交易并使用签名者的私钥签名
    let mut transaction = Transaction::new_with_payer(&final_instructions, Some(&signer.pubkey()));
    transaction.sign(&[signer], recent_blockhash);

    Ok(transaction)
}

// 主函数，使用tokio运行时执行异步代码
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 创建RPC客户端，连接到本地Solana节点
    let client = RpcClient::new_with_commitment(
        String::from("http://localhost:8899"),
        CommitmentConfig::confirmed(),
    );

    // 生成发送方和接收方的密钥对
    let sender = Keypair::new();
    let recipient = Keypair::new();
    let amount = LAMPORTS_PER_SOL / 2; // 转账金额：0.5 SOL

    // 向发送方空投1个SOL
    let signature = client
        .request_airdrop(&sender.pubkey(), LAMPORTS_PER_SOL)
        .await?;

    // 确认空投交易
    let latest_blockhash = client.get_latest_blockhash().await?;
    client
        .confirm_transaction_with_spinner(
            &signature,
            &latest_blockhash,
            CommitmentConfig::confirmed(),
        )
        .await?;

    // 创建转账指令
    let transfer_instruction = transfer(&sender.pubkey(), &recipient.pubkey(), amount);

    // 设置优先级费用为1 microLamport
    let priority_fee = 1; // microLamports

    // 构建优化的交易
    let transaction = 
        build_optimal_transaction(&client, &[transfer_instruction], &sender, priority_fee).await?;

    // 发送交易并等待确认
    let transaction_signature = client.send_and_confirm_transaction(&transaction).await?;

    // 打印交易签名
    println!("Transaction sent: {}", transaction_signature);

    Ok(())
}