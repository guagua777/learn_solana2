1. 参考：
    1. https://solana.com/zh/developers/cookbook
    2. bilibili.com/video/BV1KScLeUEUm/
2. 例子：https://github.com/solana-developers/program-examples
3. 指令的 data 字段是一个字节数组，用于告知程序要调用哪个函数，并为该函数提供参数。 
4. 并将钱包地址设置为 token account 的 owner。每个钱包可以拥有多个相同代币（mint）的 token account
5. 请注意，每个 token account 的数据都包含一个 owner 字段，用于标识谁拥有该 token account 的权限。这与基础 Account 类型中指定的程序 owner 不同，后者对于所有 token account 来说都是 Token Program。
6. Solana 允许一个钱包为同一个 mint 拥有多个 token account，只是日常使用中大家默认用唯一的 ATA，所以看起来像“只有一个”。
7. 🟢 ATA（常用）
钱包 + mint → 1 个（唯一）

👉 这是“默认账户”

🔴 普通 Token Account（底层）
钱包 + mint → 无限多个

👉 这是“真实能力”
8. token account 是普通账户，不是唯一映射
9. Solana runtime：

👉 完全不关心 ATA

👉 它只认：

token account（任何符合结构的账户）
10. Token Program 本身不负责“创建账户”，它只负责“初始化账户”
🧠 Step 1：创建普通账户（System Program）

👉 分配空间 + 转租金（lamports）

🧠 Step 2：初始化为 token account（Token Program）

👉 写入：

mint
owner
状态

👉 合在一起才是一个完整的 token account
11. 创建普通的代币账户：https://beta.solpg.io/660ce716cffcf4b13384d010
如果要用新的 keypair 创建一个新的 Token Account，而不是使用 Associated Token Account 地址，需要发送包含两个指令的交易。以下是在 Solana Playground 上的 Javascript 示例。

System Program 创建一个新账户，为 Token Account 数据分配空间，并将所有权转移给 Token Program。

Token Program 将数据初始化为 Token Account。
12. 创建ATA关联代币账户：https://beta.solpg.io/660ce868cffcf4b13384d011
Associated Token Program 使用 跨程序调用 来：

调用 System Program 使用提供的 PDA 作为地址创建新账户
调用 Token Program 初始化 Token Account 数据
13. https://www.anchor-lang.com/docs/tokens/basics/mint-tokens
    1.  mint账户使用普通账户（即在椭圆曲线上的账户）
    2.  mint账户使用PDA账户
