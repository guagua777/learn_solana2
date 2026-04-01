use anchor_lang::prelude::*;

declare_id!("A6XM2rk9TWojBnAwX2v7L11ATJ7QKY3UXey6D7Ycxhq6");

// 这是一个极简的 Solana 计数器合约
// 功能：初始化计数器 + 点击数字 +1
use anchor_lang::prelude::*;

// 程序 ID（会自动匹配你的项目）
// declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod surfpool_demo {
    use super::*;

    // 初始化计数器
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        msg!("计数器初始化成功！当前值：{}", counter.count);
        Ok(())
    }

    // 点击 +1
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count += 1;
        msg!("计数器 +1！当前值：{}", counter.count);
        Ok(())
    }
}

// 账户定义
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

// 数据结构
#[account]
pub struct Counter {
    pub count: u64,
}