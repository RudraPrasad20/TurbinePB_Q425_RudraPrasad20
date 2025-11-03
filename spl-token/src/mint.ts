import "dotenv/config";
import { getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token"
import { Connection, Keypair, PublicKey } from "@solana/web3.js"
import bs58 from "bs58";

const keypair = Keypair.fromSecretKey(bs58.decode(process.env.WALLET!));

const connection = new Connection("https://api.devnet.solana.com", "confirmed")
const mint = new PublicKey("GHZ29nUjyQrLcfxFNUTtKbLxmrkW3Za7ysk9Xi6yofsc")

async function CreateMint() {
    try {
        const ata = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey,
            true // <-- allow owner off curve / force create
          );
          
        console.log(ata.address.toBase58())

        const tx = await mintTo(connection, keypair, mint, ata.address, keypair.publicKey, 100000000)
        console.log(tx)
    } catch (error) {
        console.log(error)
    }
}

CreateMint()