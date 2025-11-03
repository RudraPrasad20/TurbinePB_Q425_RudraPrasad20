import "dotenv/config";
import { createMint } from "@solana/spl-token";
import { Connection, Keypair } from "@solana/web3.js";
import bs58 from "bs58";

const keypair = Keypair.fromSecretKey(bs58.decode(process.env.WALLET!));
const connection = new Connection("https://api.devnet.solana.com", "confirmed")

async function Mint() {
    try {
        const mint = await createMint(
            connection,
            keypair,keypair.publicKey, null, 9
        )
        console.log("minted successfully", mint)
    } catch (error) {
        console.log(error)
    }
}

Mint()