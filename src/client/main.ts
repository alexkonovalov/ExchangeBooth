import {
    Connection,
    sendAndConfirmTransaction,
    Transaction,
} from "@solana/web3.js";
import { createMint } from "@solana/spl-token";
import path from "path";
import * as yargs from "yargs";
import { hideBin } from "yargs/helpers";
import { closeAssociatedToken, mintAssociatedToken } from "./service";
import { createKeypairFromFile, getConfig } from "./helpers";
import { Processor } from "./processor";
import { MINT_A_DECIMALS, Instruction } from "./const";

const PROGRAM_PATH = path.resolve(__dirname, "../../dist/program");
const KEYS_PATH = path.resolve(__dirname, "../../dist/keys");
const MINT_A_SO_PATH = path.join(KEYS_PATH, "mint_a.so");
const MINT_B_SO_PATH = path.join(KEYS_PATH, "mint_b.so");
const USER_SO_PATH = path.join(KEYS_PATH, "user.so");

const PROGRAM_KEYPAIR_PATH = path.join(
    PROGRAM_PATH,
    "exchange_booth-keypair.json"
);

async function main() {
    console.log("Running solana RPC program...");
    const config = await getConfig();
    const connection = new Connection(config.json_rpc_url, "confirmed");
    const version = await connection.getVersion();
    console.log(
        "Connection to cluster established:",
        config.json_rpc_url,
        version
    );

    let adminKeypair = await createKeypairFromFile(config.keypair_path);
    let userKeypair = await createKeypairFromFile(USER_SO_PATH);
    let mintAKeypair = await createKeypairFromFile(MINT_A_SO_PATH);
    let mintBKeypair = await createKeypairFromFile(MINT_B_SO_PATH);

    const args = yargs
        .default(hideBin(process.argv))
        .option("ix", {
            description: "Run instruction",
            type: "number",
            requiresArg: true,
            demandOption: false,
        })
        .option("tokens:clear", {
            description: "Remove mint and token accounts",
            type: "boolean",
            requiresArg: false,
            demandOption: false,
        })
        .option("tokens:create", {
            description: "Create mint and token accounts",
            type: "boolean",
            requiresArg: false,
            demandOption: false,
        })
        .option("admin:airdrop", {
            description: "Request airdrop for admin",
            type: "boolean",
            requiresArg: false,
            demandOption: false,
        })
        .option("user:airdrop", {
            description: "Request airdrop for user",
            type: "boolean",
            requiresArg: false,
            demandOption: false,
        })
        .parseSync();

    switch (true) {
        case args.ix !== undefined: {
            let programKeypair = await createKeypairFromFile(
                PROGRAM_KEYPAIR_PATH
            );
            let programId = programKeypair.publicKey;

            const processor = new Processor(
                mintAKeypair.publicKey,
                mintBKeypair.publicKey,
                programId,
                connection
            );

            const instruction = args.ix as Instruction;
            const signerKeypair =
                instruction == Instruction.Exchange
                    ? userKeypair
                    : adminKeypair;

            const transactionInstruction = await processor.process(
                instruction,
                signerKeypair,
                adminKeypair.publicKey
            );

            let transaction = new Transaction();
            transaction.instructions = [transactionInstruction];

            await sendAndConfirmTransaction(connection, transaction, [
                signerKeypair,
            ]);

            break;
        }
        case args["admin:airdrop"]: {
            const sig = await connection.requestAirdrop(
                adminKeypair.publicKey,
                1000000000
            );
            await connection.confirmTransaction(sig);
            break;
        }
        case args["user:airdrop"]: {
            const sig = await connection.requestAirdrop(
                userKeypair.publicKey,
                1000000000
            );
            await connection.confirmTransaction(sig);
            break;
        }
        case args["tokens:create"]: {
            await mintAssociatedToken({
                connection,
                payerKeypair: adminKeypair,
                mintPK: mintAKeypair.publicKey,
                amount: 10 * Math.pow(10, MINT_A_DECIMALS),
                mintAuthority: adminKeypair,
            });
            await mintAssociatedToken({
                connection,
                payerKeypair: adminKeypair,
                mintPK: mintBKeypair.publicKey,
                amount: 10 * Math.pow(10, MINT_A_DECIMALS),
                mintAuthority: adminKeypair,
            });
            await mintAssociatedToken({
                connection,
                payerKeypair: userKeypair,
                mintPK: mintAKeypair.publicKey,
                amount: 10 * Math.pow(10, MINT_A_DECIMALS),
                mintAuthority: adminKeypair,
            });
            await mintAssociatedToken({
                connection,
                payerKeypair: userKeypair,
                mintPK: mintBKeypair.publicKey,
                amount: 10 * Math.pow(10, MINT_A_DECIMALS),
                mintAuthority: adminKeypair,
            });
            break;
        }
        case args["mints:create"]: {
            await createMint(
                connection,
                adminKeypair,
                adminKeypair.publicKey,
                adminKeypair.publicKey,
                MINT_A_DECIMALS,
                mintAKeypair
            );

            await createMint(
                connection,
                adminKeypair,
                adminKeypair.publicKey,
                adminKeypair.publicKey,
                MINT_A_DECIMALS,
                mintBKeypair
            );
            break;
        }
        case args["tokens:clear"]: {
            closeAssociatedToken({
                connection,
                mintPK: mintAKeypair.publicKey,
                owner: userKeypair,
            });
            closeAssociatedToken({
                connection,
                mintPK: mintBKeypair.publicKey,
                owner: userKeypair,
            });
            break;
        }
        default:
            throw Error("No args provided");
    }
}

main().then(
    () => process.exit(),
    (err) => {
        console.error(err);
        process.exit(-1);
    }
);
