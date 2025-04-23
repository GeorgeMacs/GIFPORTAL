const anchor = require("@project-serum/anchor");

const main = async () => {
  console.log("Starting tests...");

  const provider = anchor.Provider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Gifportal

  const baseAccount = anchor.web3.Keypair.generate()
  const tx = await program.rpc.StartStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    },
    signers: [baseAccount]
  })
  console.log("Your transaction signature", tx)

  let account = await program.account.baseAccount.fetch(
    baseAccount.publicKey
  )
  console.log("GIF Count", account.totalGifs.toString())

  await program.rpc.addGif("https://media.giphy.com/media/v1.Y2lkPTc5MGI3NjExM3hlcm8xb3p4bmxpamk4andwN3BpaGoyaTU2NGc5dnc4bW82bHJpbSZlcD12MV9naWZzX3RyZW5kaW5nJmN0PWc/Y1GVcirpKOkWXwru97/giphy.gif", {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    }
  })

  account =await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log("GIF Count", account.totalGifs.toString())
  console.log("GIF List", account.gifList);
}
const runMain = async () => {
  try{
    await main()
    process.exit(0)
  } catch(error) {
    console.error(error)
    process.exit(1)
  }
}
runMain()