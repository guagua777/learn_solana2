use bip39::{Mnemonic, Language};
use pbkdf2::{hmac::Hmac, pbkdf2};
use sha2::Sha512;
use hex;
use solana_sdk::signature::{keypair_from_seed, Signer, Keypair};
use solana_client::rpc_client::RpcClient;

// 配置常量
const RPC_URL: &str = "http://localhost:8899";
const PASSPHRASE: &str = "fyx"; // 可选密码短语，留空表示无密码
const MNEMONIC_WORD_COUNT: usize = 12;

fn main() {
    // 1. 生成助记词
    let mnemonic = Mnemonic::generate_in(Language::English, MNEMONIC_WORD_COUNT).unwrap();
    let phrase = mnemonic.to_string(); // 获取助记词短语
    println!("助记词: {}", phrase);

    // 2. 通过助记词生成种子
    let salt = format!("mnemonic{}", PASSPHRASE);
    let mut seed = [0u8; 64]; // 种子是 64 字节
    pbkdf2::<Hmac<Sha512>>(phrase.as_bytes(), salt.as_bytes(), 2048, &mut seed);
    println!("种子: {}", hex::encode(seed));

    // 3: 使用种子生成密钥对
    let keypair = keypair_from_seed(&seed[..32]).expect("生成密钥对失败");
    println!("公钥: {}", hex::encode(keypair.pubkey().to_bytes()));
    println!("公钥 (base58): {}", keypair.pubkey().to_string());
    // 注意：在生产环境中不应打印私钥
    // println!("私钥: {}", hex::encode(keypair.secret().to_bytes()));

    // 4. 查询余额
    let rpc_client = RpcClient::new(RPC_URL);
    match rpc_client.get_balance(&keypair.pubkey()) {
        Ok(balance) => {
            println!("账户余额: {} lamports", balance);
            println!("账户余额: {} SOL", balance as f64 / 1_000_000_000.0);
        }
        Err(e) => println!("查询余额失败: {:?}", e),
    }
}