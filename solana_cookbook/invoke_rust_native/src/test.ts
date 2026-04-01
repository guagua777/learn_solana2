import * as path from "path";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction
} from "@solana/web3.js";
import { LiteSVM } from "litesvm";

test("sol transfer cpi", () => {
  const svm = new LiteSVM();

  const programId = PublicKey.unique();
  const programPath = path.join(__dirname, "program.so");
  svm.addProgramFromFile(programId, programPath);

  // Create sender and recipient
  const sender = new Keypair();
  const recipient = new Keypair();

  // Fund sender
  const amount = BigInt(LAMPORTS_PER_SOL);
  svm.airdrop(sender.publicKey, amount); // 1 SOL

  // Create instruction data buffer
  const transferAmount = amount / BigInt(2); // 0.5 SOL
  const instructionIndex = 0; // instruction index 0 for SolTransfer enum

  const data = Buffer.alloc(9); // 1 byte for instruction enum + 8 bytes for u64
  data.writeUInt8(instructionIndex, 0); // first byte identifies the instruction
  data.writeBigUInt64LE(transferAmount, 1); // remaining bytes are instruction arguments

  // Create instruction
  const instruction = new TransactionInstruction({
    programId,
    keys: [
      { pubkey: sender.publicKey, isSigner: true, isWritable: true },
      { pubkey: recipient.publicKey, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }
    ],
    data
  });

  // Create and send transaction
  const transaction = new Transaction().add(instruction);
  transaction.recentBlockhash = svm.latestBlockhash();
  transaction.sign(sender);

  svm.sendTransaction(transaction);

  // Check balances
  const recipientBalance = svm.getBalance(recipient.publicKey);
  const senderBalance = svm.getBalance(sender.publicKey);

  const transactionFee = BigInt(5000);
  expect(recipientBalance).toBe(transferAmount);
  expect(senderBalance).toBe(amount - transferAmount - transactionFee);
});