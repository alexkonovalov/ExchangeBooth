import { Buffer } from 'buffer';
import BN from "bn.js";

function serializeVec(bytes: Buffer) {
    const length = Buffer.from(new Uint8Array((new BN(bytes.length)).toArray("le", 4)));
    return Buffer.concat([length, bytes]);
}

export function getMessageVecBuffer(message: string) {
    const buffer = Buffer.from(message);
    return serializeVec(buffer);
}

export function getF64Buffer(val: number) {
    let fa = new Float64Array(1);
    fa[0]= val;
    return Buffer.from(fa.buffer);
  }
  