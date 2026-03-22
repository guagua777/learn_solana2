use std::collections::HashMap;

use {
    mollusk_svm::Mollusk,
    solana_account::Account,
    solana_sdk::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    std::str::FromStr,
};
use mollusk_svm::account_store::AccountStore;
use mollusk_svm_result::{types::ProgramResult, Check};


#[test]
pub fn test_mollusk_instruction() {
    // let program_id = Pubkey::new_unique();
    let program_id = Pubkey::from_str("74aPmsMJt2zMUBAB5tDkguSkNHdoE138tDvCvdzvcrti").unwrap();
    let key1 = Pubkey::new_unique();
    let key2 = Pubkey::new_unique();

    let instruction = Instruction::new_with_bytes(
        program_id,
        &[],
        // 账户列表
        vec![
            // AccountMeta::new_readonly(key1, false), // 只读账户，并且不是签名账户
            AccountMeta::new_readonly(key1, true)
        ],
    );

    let accounts = vec![(key1, Account::default()),];

    let mollusk = Mollusk::new(&program_id, "my_program");

    // Execute the instruction and get the result.
    let result = mollusk.process_instruction(&instruction, &accounts[..]);

    println!("result is: {:?}", result);

    assert!(result.program_result.is_ok());
    assert!(result.program_result == ProgramResult::Success);

}




#[test]
/// 测试 Mollusk 库的 process_and_validate_instruction 方法
/// 该方法执行指令并验证执行结果是否符合预期
pub fn test_mollusk_with_check() {
    // 使用固定的程序 ID（注释掉的代码使用随机生成的程序 ID）
    // let program_id = Pubkey::new_unique();
    let program_id = Pubkey::from_str("74aPmsMJt2zMUBAB5tDkguSkNHdoE138tDvCvdzvcrti").unwrap();
    
    // 生成一个随机的密钥对，用于测试账户
    let key1 = Pubkey::new_unique();

    // 创建一个新的指令，包含：
    // 1. 程序 ID
    // 2. 空的指令数据
    // 3. 账户列表（包含一个只读且非签名的账户）
    let instruction = Instruction::new_with_bytes(
        program_id,
        &[],
        // 账户列表
        vec![
            AccountMeta::new_readonly(key1, false), // 只读账户，并且不是签名账户
            // AccountMeta::new_readonly(key1, true) // 注释掉的代码：只读账户，并且是签名账户
        ],
    );

    // 创建账户列表，包含 key1 和一个默认账户
    let accounts = vec![(key1, Account::default()),];

    // 创建 Mollusk 实例，指定程序 ID 和程序名称
    let mollusk = Mollusk::new(&program_id, "my_program");

    // 定义检查条件（以下是一些尝试的检查条件，最终使用的是最后一个定义的 checks）
    
    // 尝试 1: 检查执行是否成功
    // let checks = vec![
    // Check::success(),
    // Check::compute_units(system_processor::DEFAULT_COMPUTE_UNITS),
    // Check::account(&sender)
    //     .lamports(base_lamports - transfer_amount)
    //     .build(),
    // Check::account(&recipient)
    //     .lamports(base_lamports + transfer_amount)
    //     .build(),
    // ];

    // 尝试 2: 检查是否返回 MissingRequiredSignature 错误
    // let checks = vec![
    //     Check::err(solana_program_error::ProgramError::MissingRequiredSignature)
    // ];

    // 尝试 3: 检查是否返回 MissingRequiredSignature 错误，并且检查账户所有者是否为系统程序
    let system_program = Pubkey::default();
    
    let checks = vec![
        // 检查是否返回 MissingRequiredSignature 错误
        Check::err(solana_program_error::ProgramError::MissingRequiredSignature),
        // 检查 key1 账户的所有者是否为系统程序
        Check::account(&key1).owner(&system_program).build(),
    ];

    // 执行指令并验证结果是否符合检查条件
    let result = mollusk.process_and_validate_instruction(&instruction, &accounts[..], &checks);

    // 打印执行结果
    println!("result is: {:?}", result);

}





#[test]
/// 测试 Mollusk 库的 process_and_validate_instruction 方法
/// 该方法执行指令并验证执行结果是否符合预期
pub fn test_check_data() {
    // 使用固定的程序 ID（注释掉的代码使用随机生成的程序 ID）
    // let program_id = Pubkey::new_unique();
    let program_id = Pubkey::from_str("74aPmsMJt2zMUBAB5tDkguSkNHdoE138tDvCvdzvcrti").unwrap();
    
    // 生成一个随机的密钥对，用于测试账户
    let data_account = Pubkey::new_unique();
    let payer_account = Pubkey::new_unique();

    // 创建一个新的指令，包含：
    // 1. 程序 ID
    // 2. 空的指令数据
    // 3. 账户列表（包含一个只读且非签名的账户）
    let instruction = Instruction::new_with_bytes(
        program_id,
        &[],
        // 账户列表
        vec![
            AccountMeta::new(data_account, true),
            // AccountMeta::new_readonly(payer_account, true),
        ],
    );

    
    // 创建账户列表，包含 key1 和一个默认账户
    let accounts = vec![
        // 设定 data_account 账户的初始数据为 [1]，并分配 500000000 lamports
        (data_account, Account::new_data_with_space(500_000_000, &[0u8], 1, &program_id).unwrap()),
    ];

    // 创建 Mollusk 实例，指定程序 ID 和程序名称
    let mollusk = Mollusk::new(&program_id, "my_program2");
    
    let system_program = Pubkey::default();
    let checks = vec![
        Check::success(),
        Check::account(&data_account).owner(&program_id).build(),
        // Check::account(&data_account).owner(&system_program).build(),
        Check::account(&data_account).data(&[1]).build(),
    ];

    // 执行指令并验证结果是否符合检查条件
    let result = mollusk.process_and_validate_instruction(&instruction, &accounts[..], &checks);

    // 打印执行结果
    println!("result is: {:?}", result);

}



// Simple in-memory account store implementation
#[derive(Default)]
struct InMemoryAccountStore {
    accounts: HashMap<Pubkey, Account>,
}

impl AccountStore for InMemoryAccountStore {
    fn get_account(&self, pubkey: &Pubkey) -> Option<Account> {
        self.accounts.get(pubkey).cloned()
    }

    fn store_account(&mut self, pubkey: Pubkey, account: Account) {
        self.accounts.insert(pubkey, account);
    }
}



#[test]
pub fn test_check_data_context() {
    let program_id = Pubkey::from_str("74aPmsMJt2zMUBAB5tDkguSkNHdoE138tDvCvdzvcrti").unwrap();
    
    let data_account = Pubkey::new_unique();

    let instruction = Instruction::new_with_bytes(
        program_id,
        &[],
        // 账户列表
        vec![
            AccountMeta::new(data_account, true),
            // AccountMeta::new_readonly(payer_account, true),
        ],
    );


    let mut account_store = InMemoryAccountStore::default();
    account_store.store_account(data_account, Account::new_data_with_space(500_000_000, &[0u8], 1, &program_id).unwrap());

    // 创建 Mollusk 实例，指定程序 ID 和程序名称
    let mollusk = Mollusk::new(&program_id, "my_program2");    
    let context = mollusk.with_context(account_store);

    let checks = vec![
        Check::success(),
        Check::account(&data_account).owner(&program_id).build(),
        Check::account(&data_account).data(&[1]).build(),
    ];

    
    // 执行指令并验证结果是否符合检查条件
    let result = context.process_and_validate_instruction(&instruction, &checks);

    // 打印执行结果
    println!("result is: {:?}", result);




    let checks2 = vec![
        Check::success(),
        Check::account(&data_account).owner(&program_id).build(),
        Check::account(&data_account).data(&[2]).build(),
    ];
    let result = context.process_and_validate_instruction(&instruction, &checks2);

}