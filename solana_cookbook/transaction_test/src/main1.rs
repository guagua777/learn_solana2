// 导入必要的依赖库
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, // 用于SOL和Lamports之间的转换
    signature::Keypair, // 用于生成密钥对
    signer::Signer, // 用于签名交易
    transaction::Transaction, // 用于创建和管理交易
};
use solana_system_interface::instruction::transfer; // 用于创建SOL转账指令



// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
#[tokio::main] // 使用tokio运行时执行异步代码
async fn main() -> anyhow::Result<()> {
    // 创建RPC客户端，连接到本地Solana节点
    let client = RpcClient::new_with_commitment(
        String::from("http://localhost:8899"), // 本地节点地址
        CommitmentConfig::confirmed(), // 使用confirmed确认级别
    );

    // 生成发送方和接收方的密钥对
    let from_keypair = Keypair::new();
    let to_keypair = Keypair::new();

    // 向发送方空投5个SOL
    let airdrop_signature = client
        .request_airdrop(&from_keypair.pubkey(), 5 * LAMPORTS_PER_SOL)
        .await?;

    // 等待空投交易确认
    loop {
        if client.confirm_transaction(&airdrop_signature).await? {
            break;
        }
    }

    // 转账前获取余额
    let from_balance_before = client.get_balance(&from_keypair.pubkey()).await?;
    let to_balance_before = client.get_balance(&to_keypair.pubkey()).await?;

    // 打印转账前的余额信息
    println!("Before transfer:");
    println!(
        "  From: {} ({} SOL)",
        from_keypair.pubkey(),
        from_balance_before as f64 / LAMPORTS_PER_SOL as f64
    );
    println!(
        "  To:   {} ({} SOL)",
        to_keypair.pubkey(),
        to_balance_before as f64 / LAMPORTS_PER_SOL as f64
    );


    // // 1. 创建转账指令（做什么）
    // let transfer_ix = transfer(...);
    // // 2. 打包成交易（装进信封，指定付款人）
    // let mut tx = Transaction::new_with_payer(&[transfer_ix], Some(&payer));
    // // 3. 签名（必须！）
    // tx.sign(&[from_keypair], recent_blockhash);    
    // // 4. 发送上链
    // send_transaction(...);

    // 创建转账指令
    // transfer函数参数：
    // 1. 发送方公钥
    // 2. 接收方公钥
    // 3. 转账金额（以Lamports为单位）
    let transfer_ix = transfer(
        &from_keypair.pubkey(),
        &to_keypair.pubkey(),
        LAMPORTS_PER_SOL, // 转账1个SOL
    );

    // 获取最新的区块哈希，用于交易签名
    let latest_blockhash = client.get_latest_blockhash().await?;
    
    // 创建交易，指定发送方为payer
    let mut transaction = Transaction::new_with_payer(&[transfer_ix], Some(&from_keypair.pubkey()));
    
    // 使用发送方的私钥对交易进行签名
    transaction.sign(&[&from_keypair], latest_blockhash);

    // 发送交易并等待确认
    let signature = client.send_and_confirm_transaction(&transaction).await?;
    println!("\nTransaction Signature: {}", signature);

    // 转账后获取余额
    let from_balance_after = client.get_balance(&from_keypair.pubkey()).await?;
    let to_balance_after = client.get_balance(&to_keypair.pubkey()).await?;

    // 打印转账后的余额信息
    println!("\nAfter transfer:");
    println!(
        "  From: {} ({} SOL)",
        from_keypair.pubkey(),
        from_balance_after as f64 / LAMPORTS_PER_SOL as f64
    );
    println!(
        "  To:   {} ({} SOL)",
        to_keypair.pubkey(),
        to_balance_after as f64 / LAMPORTS_PER_SOL as f64
    );

    Ok(())
}