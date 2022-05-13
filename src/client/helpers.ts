import { Keypair } from "@solana/web3.js";
import os from "os";
import fs from "mz/fs";
import path from "path";
import yaml from "yaml";

export async function getConfig(): Promise<any> {
    // Path to Solana CLI config file
    const CONFIG_FILE_PATH = path.resolve(
        os.homedir(),
        ".config",
        "solana",
        "cli",
        "config.yml"
    );
    const configYml = await fs.readFile(CONFIG_FILE_PATH, { encoding: "utf8" });
    return yaml.parse(configYml);
}

export async function createKeypairFromFile(
    filePath: string
): Promise<Keypair> {
    const secretKeyString = await fs.readFile(filePath, { encoding: "utf8" });
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    return Keypair.fromSecretKey(secretKey);
}

export function getu64Buffer(val: bigint) {
    let fa = new BigUint64Array(1);
    fa[0] = val;
    return Buffer.from(fa.buffer);
}
