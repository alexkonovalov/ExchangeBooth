import { Keypair } from "@solana/web3.js";
import os from "os";
import fs from "mz/fs";
import path from "path";
import yaml from "yaml";
import BN from "bn.js";

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

function serializeVec(bytes: Buffer) {
    const length = Buffer.from(
        new Uint8Array(new BN(bytes.length).toArray("le", 4))
    );
    return Buffer.concat([length, bytes]);
}

export function getMessageVecBuffer(message: string) {
    const buffer = Buffer.from(message);
    return serializeVec(buffer);
}

export function getF64Buffer(val: number) {
    let fa = new Float64Array(1);
    fa[0] = val;
    return Buffer.from(fa.buffer);
}
