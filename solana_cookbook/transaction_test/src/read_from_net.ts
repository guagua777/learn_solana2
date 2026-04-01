import { Keypair, Connection, LAMPORTS_PER_SOL } from "@solana/web3.js";

const keypair = Keypair.generate();
console.log(`Public Key: ${keypair.publicKey}`);

const connection = new Connection("http://localhost:8899", "confirmed");

// Funding an address with SOL automatically creates an account
const signature = await connection.requestAirdrop(
  keypair.publicKey,
  LAMPORTS_PER_SOL
);
await connection.confirmTransaction(signature, "confirmed");

const accountInfo = await connection.getAccountInfo(keypair.publicKey);
console.log(JSON.stringify(accountInfo, null, 2));

// owner 字段显示拥有该账户的程序。对于钱包账户，所有者始终为 System Program，其地址为 11111111111111111111111111111111。

// System Program 
// Token Program
// loader program

// executable 字段被设置为 true，表示账户的 data 字段包含可执行代码。

// 每个 program account 都由其 loader program 拥有。在此示例中，owner 是 BPFLoader2 程序。



import { Connection, PublicKey } from "@solana/web3.js";

const connection = new Connection(
  "https://api.mainnet.solana.com",
  "confirmed"
);
const address = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
const accountInfo = await connection.getAccountInfo(address);

console.log(
  JSON.stringify(
    accountInfo,
    (key, value) => {
      if (key === "data" && value && value.length > 1) {
        return [
          value[0],
          "...truncated, total bytes: " + value.length + "...",
          value[value.length - 1]
        ];
      }
      return value;
    },
    2
  )
);