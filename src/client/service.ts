import { Keypair, Connection, PublicKey } from "@solana/web3.js";
import {
    getOrCreateAssociatedTokenAccount,
    burn,
    mintTo,
    closeAccount,
} from "@solana/spl-token";

export async function mintAssociatedToken({
    connection,
    payerKeypair,
    mintPK,
    amount,
    mintAuthority,
}: {
    connection: Connection;
    payerKeypair: Keypair;
    mintPK: PublicKey;
    amount: number | bigint;
    mintAuthority: Keypair;
}) {
    const tokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        payerKeypair,
        mintPK,
        payerKeypair.publicKey
    );
    await mintTo(
        connection,
        payerKeypair,
        mintPK,
        tokenAccount.address,
        mintAuthority,
        amount
    );
}

export async function closeAssociatedToken({
    connection,
    mintPK,
    owner,
}: {
    connection: Connection;
    mintPK: PublicKey;
    owner: Keypair;
}) {
    const tokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        owner,
        mintPK,
        owner.publicKey
    );

    await burn(
        connection,
        owner,
        tokenAccount.address,
        mintPK,
        owner.publicKey,
        tokenAccount.amount
    );

    await closeAccount(
        connection,
        owner,
        tokenAccount.address,
        owner.publicKey,
        owner.publicKey
    );
}
