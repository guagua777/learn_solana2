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

// Get minimum balance for rent exemption
const mintRent = await getMinimumBalanceForRentExemptMint(connection);

// Get the associated token account address for the fee payer
const feePayerATA = getAssociatedTokenAddressSync(
  mint.publicKey,
  feePayer.publicKey,
  false, // allowOwnerOffCurve
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID
);

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

// Create account instruction
const createAccountInstruction = SystemProgram.createAccount({
  fromPubkey: sender.publicKey,
  newAccountPubkey: mint.publicKey,
  space: MINT_SIZE,
  lamports: mintRent,
  programId: TOKEN_PROGRAM_ID
});

// Initialize mint instruction
const initializeMintInstruction = createInitializeMintInstruction(
  mint.publicKey, // mint pubkey
  2, // decimals
  sender.publicKey, // mint authority
  sender.publicKey, // freeze authority
  TOKEN_PROGRAM_ID
);

// Create associated token account instruction for fee payer
const createFeePayerATA = createAssociatedTokenAccountInstruction(
  feePayer.publicKey, // payer
  feePayerATA, // associated token account address
  feePayer.publicKey, // owner
  mint.publicKey, // mint
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID
);

// Create associated token account instruction for sender
const createSenderATA = createAssociatedTokenAccountInstruction(
  feePayer.publicKey, // payer
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
  createFeePayerATA,
  createSenderATA,
  createRecipientATA,
  mintToInstruction
);

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

// Create transfer instruction to transfer tokens to the fee payer to cover the transaction fees
// For a real world application, you would need to determine the amount of tokens to transfer to the fee payer based on the transaction fees.
const transferFeePayerInstruction = createTransferInstruction(
  senderATA, // source
  feePayerATA, // destination
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
}).add(transferInstruction, transferFeePayerInstruction);

// Sign and send transfer transaction
const transactionSignature2 = await sendAndConfirmTransaction(
  connection,
  transferTransaction,
  [feePayer, sender]
);

console.log("Successfully transferred 0.5 tokens");
console.log("Transaction Signature:", transactionSignature2);

const feePayerTokenAccount = await getAccount(
  connection,
  feePayerATA,
  "confirmed",
  TOKEN_PROGRAM_ID
);
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
  "Fee Payer balance:",
  Number(feePayerTokenAccount.amount) / 100,
  "tokens"
);
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