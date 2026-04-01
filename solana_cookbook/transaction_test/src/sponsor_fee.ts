import {
  Connection,
  Keypair,
  sendAndConfirmTransaction,
  SystemProgram,
  Transaction,
  LAMPORTS_PER_SOL
} from "@solana/web3.js";
import {
  createInitializeMintInstruction,
  MINT_SIZE,
  getMinimumBalanceForRentExemptMint,
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
  createAssociatedTokenAccountInstruction,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createMintToInstruction,
  createTransferInstruction
} from "@solana/spl-token";

// Fetch and display actual balances after transfer
import { getAccount } from "@solana/spl-token";

(async () => {

// Create connection to local validator
const connection = new Connection("http://localhost:8899", "confirmed");
const latestBlockhash = await connection.getLatestBlockhash();

// Generate a new keypair for the fee payer
const feePayer = Keypair.generate();

// Generate a new keypair for the sender
const sender = Keypair.generate();

// Generate a new keypair for the recipient
const recipient = Keypair.generate();

// Airdrop 1 SOL to fee payer
const airdropSignature = await connection.requestAirdrop(
  feePayer.publicKey,
  LAMPORTS_PER_SOL
);
await connection.confirmTransaction({
  blockhash: latestBlockhash.blockhash,
  lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
  signature: airdropSignature
});


// 为什么需要这一步
// Airdrop 0.1 SOL to sender for rent exemption
const senderAirdropSignature = await connection.requestAirdrop(
  sender.publicKey,
  LAMPORTS_PER_SOL / 10
);
await connection.confirmTransaction({
  blockhash: latestBlockhash.blockhash,
  lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
  signature: senderAirdropSignature
});

// Airdrop 0.1 SOL to recipient for rent exemption
const recipientAirdropSignature = await connection.requestAirdrop(
  recipient.publicKey,
  LAMPORTS_PER_SOL / 10
);
await connection.confirmTransaction({
  blockhash: latestBlockhash.blockhash,
  lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
  signature: recipientAirdropSignature
});

// Generate keypair to use as address of mint
const mint = Keypair.generate();

// mint账户的租金
// Get minimum balance for rent exemption
const mintRent = await getMinimumBalanceForRentExemptMint(connection);

// Get the associated token account address for the sender
const senderATA = getAssociatedTokenAddressSync(
  mint.publicKey,
  sender.publicKey,
  false, // allowOwnerOffCurve
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID
);

// Get the associated token account address for the recipient
const recipientATA = getAssociatedTokenAddressSync(
  mint.publicKey,
  recipient.publicKey,
  false, // allowOwnerOffCurve
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID
);



// 创建mint账户
// 这个为什么要从sender转租金?
// Create account instruction
const createAccountInstruction = SystemProgram.createAccount({
  fromPubkey: sender.publicKey,
  newAccountPubkey: mint.publicKey,
  space: MINT_SIZE,
  lamports: mintRent,
  programId: TOKEN_PROGRAM_ID
});

// 初始化mint账户
// Initialize mint instruction
const initializeMintInstruction = createInitializeMintInstruction(
  mint.publicKey, // mint pubkey
  2, // decimals
  sender.publicKey, // mint authority
  sender.publicKey, // freeze authority
  TOKEN_PROGRAM_ID
);


// 创建token账户
// Create associated token account instruction for sender
const createSenderATA = createAssociatedTokenAccountInstruction(
  sender.publicKey, // payer
  senderATA, // associated token account address
  sender.publicKey, // owner
  mint.publicKey, // mint
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID
);

// Create recipient's associated token account
const createRecipientATA = createAssociatedTokenAccountInstruction(
  feePayer.publicKey, // payer
  recipientATA, // associated token account address
  recipient.publicKey, // owner
  mint.publicKey, // mint
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID
);


// mint 到 senderATA中
// Create a separate transaction for minting tokens
// Create mint to instruction (mint 100 tokens = 1.00 with 2 decimals)
const mintAmount = 100;
const mintToInstruction = createMintToInstruction(
  mint.publicKey, // mint
  senderATA, // destination
  sender.publicKey, // authority
  mintAmount, // amount
  [], // multiSigners
  TOKEN_PROGRAM_ID // programId
);

// Create and sign transaction with mint creation and fee payer ATA creation
const transaction = new Transaction({
  feePayer: feePayer.publicKey,
  blockhash: latestBlockhash.blockhash,
  lastValidBlockHeight: latestBlockhash.lastValidBlockHeight
}).add(
  createAccountInstruction,
  initializeMintInstruction,
  createSenderATA,
  createRecipientATA,
  mintToInstruction
);
// Solana 交易结构里有专门的 feePayer 字段：
// 构造交易时指定 feePayer 公钥
// feePayer 必须对交易签名
// 链上从 feePayer 扣手续费，而非操作用户
// 支持所有交易类型：转账、Token 操作、DeFi、合约调用等

// 这个的签名里面为什么不需要recipient??
// Sign transaction
const transactionSignature = await sendAndConfirmTransaction(
  connection,
  transaction,
  [feePayer, sender, mint]
);

console.log("Transaction Signature:", transactionSignature);

// Create transfer instruction (transfer 50 tokens = 0.50 with 2 decimals)
const transferAmount = 50;
const transferInstruction = createTransferInstruction(
  senderATA, // source
  recipientATA, // destination
  sender.publicKey, // owner
  transferAmount, // amount
  [], // multiSigners
  TOKEN_PROGRAM_ID // programId
);

// Get a new blockhash for the transfer transaction
const transferBlockhash = await connection.getLatestBlockhash();

// Create transaction for token transfer
let transferTransaction = new Transaction({
  feePayer: feePayer.publicKey,
  blockhash: transferBlockhash.blockhash,
  lastValidBlockHeight: transferBlockhash.lastValidBlockHeight
}).add(transferInstruction);

// Sign and send transfer transaction
const transactionSignature2 = await sendAndConfirmTransaction(
  connection,
  transferTransaction,
  [feePayer, sender]
);

console.log("Successfully transferred 0.5 tokens");
console.log("Transaction Signature:", transactionSignature2);



const senderTokenAccount = await getAccount(
  connection,
  senderATA,
  "confirmed",
  TOKEN_PROGRAM_ID
);
const recipientTokenAccount = await getAccount(
  connection,
  recipientATA,
  "confirmed",
  TOKEN_PROGRAM_ID
);

console.log("=== Final Balances ===");
console.log(
  "Sender balance:",
  Number(senderTokenAccount.amount) / 100,
  "tokens"
);
console.log(
  "Recipient balance:",
  Number(recipientTokenAccount.amount) / 100,
  "tokens"
);

})();