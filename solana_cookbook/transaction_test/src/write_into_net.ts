import {
  LAMPORTS_PER_SOL,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
  Keypair,
  Connection
} from "@solana/web3.js";

const connection = new Connection("http://localhost:8899", "confirmed");

const sender = new Keypair();
const receiver = new Keypair();

const signature = await connection.requestAirdrop(
  sender.publicKey,
  LAMPORTS_PER_SOL
);
await connection.confirmTransaction(signature, "confirmed");

const transferInstruction = SystemProgram.transfer({
  fromPubkey: sender.publicKey,
  toPubkey: receiver.publicKey,
  lamports: 0.01 * LAMPORTS_PER_SOL
});


// 创建一个交易并将指令添加到交易中。在此示例中，我们创建了一个包含单个指令的交易。然而，您可以向一个交易中添加多个指令。
const transaction = new Transaction().add(transferInstruction);


// 签署并发送 交易 到网络。发送者 的 keypair 需要包含在签名者数组中，以授权从其账户转移 SOL。
const transactionSignature = await sendAndConfirmTransaction(
  connection,
  transaction,
  [sender]
);

console.log("Transaction Signature:", `${transactionSignature}`);

const senderBalance = await connection.getBalance(sender.publicKey);
const receiverBalance = await connection.getBalance(receiver.publicKey);

console.log("Sender Balance:", `${senderBalance}`);
console.log("Receiver Balance:", `${receiverBalance}`);





import {
  Connection,
  Keypair,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
  LAMPORTS_PER_SOL
} from "@solana/web3.js";
import {
  MINT_SIZE,
  TOKEN_2022_PROGRAM_ID,
  createInitializeMint2Instruction,
  getMinimumBalanceForRentExemptMint,
  getMint
} from "@solana/spl-token";

const connection = new Connection("http://localhost:8899", "confirmed");

const wallet = new Keypair();
// Fund the wallet with SOL
const signature = await connection.requestAirdrop(
  wallet.publicKey,
  LAMPORTS_PER_SOL
);
await connection.confirmTransaction(signature, "confirmed");

// Generate keypair to use as address of mint account
const mint = new Keypair();


// 计算 mint 账户所需的最小 lamports。 getMinimumBalanceForRentExemptMint 函数用于计算 mint 账户数据需要分配多少 lamport。
// Calculate lamports required for rent exemption
const rentExemptionLamports =
  await getMinimumBalanceForRentExemptMint(connection);



// 第一个指令会调用 System Program 的 createAccount 指令，用于：
// 分配存储铸币数据所需的 字节数。
// 从钱包中 转移 lamports 以资助新账户。
// 将账户的 所有权 分配给 Token Extensions Program。
// Instruction to create new account with space for new mint account
const createAccountInstruction = SystemProgram.createAccount({
  fromPubkey: wallet.publicKey,
  newAccountPubkey: mint.publicKey,
  space: MINT_SIZE,
  lamports: rentExemptionLamports,
  programId: TOKEN_2022_PROGRAM_ID
});


// 第二个指令会调用 Token Extensions Program 的 createInitializeMint2Instruction 指令，以以下数据初始化 mint 账户：
// 2 位小数
// 钱包 作为铸币权限和冻结权限
// Instruction to initialize mint account
const initializeMintInstruction = createInitializeMint2Instruction(
  mint.publicKey,
  2, // decimals
  wallet.publicKey, // mint authority
  wallet.publicKey, // freeze authority
  TOKEN_2022_PROGRAM_ID
);

// 将两个指令添加到一个交易中。这确保了账户创建和初始化是原子操作。（要么两个指令都成功，要么都失败。）
// 这种方法在构建复杂的 Solana 交易时很常见，因为它保证所有指令一起执行。
// Build transaction with instructions to create new account and initialize mint account
const transaction = new Transaction().add(
  createAccountInstruction,
  initializeMintInstruction
);


// 签署并发送交易。需要两个签名：
// 钱包 账户签署，作为 交易费用 和账户创建的付款方
// 铸币账户 签署，授权其地址用于新账户
const transactionSignature = await sendAndConfirmTransaction(
  connection,
  transaction,
  [
    wallet, // payer
    mint // mint address keypair
  ]
);

console.log("Transaction Signature:", `${transactionSignature}`);

const mintData = await getMint(
  connection,
  mint.publicKey,
  "confirmed",
  TOKEN_2022_PROGRAM_ID
);
console.log(
  "Mint Account:",
  JSON.stringify(
    mintData,
    (key, value) => {
      // Convert BigInt to String
      if (typeof value === "bigint") {
        return value.toString();
      }
      // Handle Buffer objects
      if (Buffer.isBuffer(value)) {
        return `<Buffer ${value.toString("hex")}>`;
      }
      return value;
    },
    2
  )
);