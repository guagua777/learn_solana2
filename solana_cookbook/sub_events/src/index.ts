import { Connection, PublicKey } from "@solana/web3.js";

// 立即调用的异步函数表达式
(async () => {

    console.log("listen start ....")

  // 使用Connection模块，连接到本地节点
  const connecting = new Connection("http://127.0.0.1:8899/", "confirmed");

  // 创建一个公钥对象，要监听的目标账户
  const publicKeyString = "Do1VuFXYouUwVAM2WKiJW5vR4QmesSpmFMVrG7ejQ7mw";
  const walletPublicKey = new PublicKey(publicKeyString);

  // 注册账户变化监听器，当账户的状态发生变化时，打印出最新的账户信息
  connecting.onAccountChange(
    walletPublicKey,
    (updatedAccountInfo, context) => {
      console.log("Updated account info: ", updatedAccountInfo);
    },
    { commitment: "confirmed" }
  );

})();