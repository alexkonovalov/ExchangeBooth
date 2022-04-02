import { Buffer } from 'buffer';
import BN from "bn.js";

export function serializeVec(bytes: Buffer) {
    const length = Buffer.from(new Uint8Array((new BN(bytes.length)).toArray("le", 4)));
    return Buffer.concat([length, bytes]);
}

export function getMessageVec(message: string) {
    const buffer = Buffer.from(message);
    return serializeVec(buffer);
}