import {
    PublicKey,
    SystemProgram,
    SYSVAR_RENT_PUBKEY,
    TransactionInstruction,
} from "@solana/web3.js";
import BN from "bn.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { getF64Buffer, getu64Buffer } from "./helpers";
import { Instruction } from "./const";

export type CreateEbParams = {
    adminKey: PublicKey;
    ebKey: PublicKey;
    vault1Key: PublicKey;
    vault2Key: PublicKey;
    oracleKey: PublicKey;
    tokenRate: number;
};

export type DepositEbParams = {
    adminKey: PublicKey;
    vault1Key: PublicKey;
    vault2Key: PublicKey;
    donor1Key: PublicKey;
    donor2Key: PublicKey;
    amount_a: bigint;
    amount_b: bigint;
};

export type WithdrawEbParams = {
    adminKey: PublicKey;
    vault1Key: PublicKey;
    vault2Key: PublicKey;
    receiver1Key: PublicKey;
    receiver2Key: PublicKey;
};

export type CloseEbParams = {
    adminKey: PublicKey;
    ebKey: PublicKey;
    oracleKey: PublicKey;
    vault1Key: PublicKey;
    vault2Key: PublicKey;
    receiver1Key: PublicKey;
    receiver2Key: PublicKey;
};

export type ExchangeParams = {
    userKey: PublicKey;
    adminKey: PublicKey;
    oracleKey: PublicKey;
    receiverVaultKey: PublicKey;
    donorVaultKey: PublicKey;
    receiverKey: PublicKey;
    donorKey: PublicKey;
    ebKey: PublicKey;
    amount: bigint;
};

export const EB_PDA_SEED_GENERATORS = {
    ORACLE: (mint1PK: PublicKey, mint2PK: PublicKey, ownerPK: PublicKey) => [
        ownerPK.toBuffer(),
        mint1PK.toBuffer(),
        mint2PK.toBuffer(),
    ],
    EXCHANGE_BOOTH: (oraclePK: PublicKey) => [oraclePK.toBuffer()],
    VAULT1: (ownerPK: PublicKey, mint1PK: PublicKey) => [
        ownerPK.toBuffer(),
        mint1PK.toBuffer(),
    ],
    VAULT2: (ownerPK: PublicKey, mint2PK: PublicKey) => [
        ownerPK.toBuffer(),
        mint2PK.toBuffer(),
    ],
};

export class ExchangeBoothProgram {
    private readonly mint1Key: PublicKey;
    private readonly mint2Key: PublicKey;
    private readonly programId: PublicKey;

    constructor(
        mint1Key: PublicKey,
        mint2Key: PublicKey,
        programId: PublicKey
    ) {
        this.mint1Key = mint1Key;
        this.mint2Key = mint2Key;
        this.programId = programId;
    }

    public initialize({
        adminKey,
        ebKey,
        vault1Key,
        vault2Key,
        oracleKey,
    }: CreateEbParams) {
        const exchangeRate = 100;
        const boothFee = 10;

        const rateDecimals = 2;
        const feeDecimals = 2;

        const createEbIxData = Buffer.concat([
            new Uint8Array([Instruction.Initialize]),
            getu64Buffer(BigInt(exchangeRate)),
            Buffer.from(new Uint8Array(new BN(rateDecimals).toArray("le", 1))),
            getu64Buffer(BigInt(boothFee)),
            Buffer.from(new Uint8Array(new BN(feeDecimals).toArray("le", 1))),
        ]);
        return new TransactionInstruction({
            keys: [
                { pubkey: adminKey, isSigner: true, isWritable: true },
                { pubkey: ebKey, isSigner: false, isWritable: true },
                {
                    pubkey: SystemProgram.programId,
                    isSigner: false,
                    isWritable: false,
                },
                { pubkey: this.mint1Key, isSigner: false, isWritable: false },
                { pubkey: this.mint2Key, isSigner: false, isWritable: false },
                { pubkey: vault1Key, isSigner: false, isWritable: true },
                { pubkey: vault2Key, isSigner: false, isWritable: true },
                { pubkey: oracleKey, isSigner: false, isWritable: true },
                {
                    pubkey: TOKEN_PROGRAM_ID,
                    isSigner: false,
                    isWritable: false,
                },
                {
                    pubkey: SYSVAR_RENT_PUBKEY,
                    isSigner: false,
                    isWritable: false,
                },
            ],
            programId: this.programId,
            data: Buffer.from(createEbIxData),
        });
    }

    public deposit({
        adminKey,
        vault1Key,
        vault2Key,
        donor1Key,
        donor2Key,
        amount_a,
        amount_b,
    }: DepositEbParams) {
        const depositIxData = Buffer.concat([
            new Uint8Array([Instruction.Deposit]),
            getu64Buffer(amount_a),
            getu64Buffer(amount_b),
        ]);
        return new TransactionInstruction({
            programId: this.programId,
            keys: [
                { pubkey: adminKey, isSigner: true, isWritable: true },
                { pubkey: vault1Key, isSigner: false, isWritable: true },
                { pubkey: vault2Key, isSigner: false, isWritable: true },
                {
                    pubkey: TOKEN_PROGRAM_ID,
                    isSigner: false,
                    isWritable: false,
                },
                { pubkey: donor1Key, isSigner: false, isWritable: true },
                { pubkey: donor2Key, isSigner: false, isWritable: true },
            ],
            data: depositIxData,
        });
    }

    public withdrow({
        adminKey,
        vault1Key,
        vault2Key,
        receiver1Key,
        receiver2Key,
    }: WithdrawEbParams) {
        return new TransactionInstruction({
            keys: [
                { pubkey: adminKey, isSigner: true, isWritable: true },
                { pubkey: vault1Key, isSigner: false, isWritable: true },
                { pubkey: vault2Key, isSigner: false, isWritable: true },
                { pubkey: receiver1Key, isSigner: false, isWritable: true },
                { pubkey: receiver2Key, isSigner: false, isWritable: true },
                {
                    pubkey: TOKEN_PROGRAM_ID,
                    isSigner: false,
                    isWritable: false,
                },
            ],
            programId: this.programId,
            data: Buffer.from(new Uint8Array([Instruction.Withdraw])),
        });
    }

    public close({
        adminKey,
        ebKey,
        oracleKey,
        vault1Key,
        vault2Key,
        receiver1Key,
        receiver2Key,
    }: CloseEbParams) {
        return new TransactionInstruction({
            keys: [
                { pubkey: adminKey, isSigner: true, isWritable: true },
                { pubkey: ebKey, isSigner: false, isWritable: true },
                { pubkey: vault1Key, isSigner: false, isWritable: true },
                { pubkey: vault2Key, isSigner: false, isWritable: true },
                { pubkey: this.mint1Key, isSigner: false, isWritable: true },
                { pubkey: this.mint2Key, isSigner: false, isWritable: true },
                { pubkey: receiver1Key, isSigner: false, isWritable: true },
                { pubkey: receiver2Key, isSigner: false, isWritable: true },
                { pubkey: oracleKey, isSigner: false, isWritable: true },
                {
                    pubkey: TOKEN_PROGRAM_ID,
                    isSigner: false,
                    isWritable: false,
                },
            ],
            programId: this.programId,
            data: Buffer.from(new Uint8Array([Instruction.Close])),
        });
    }

    public exchange({
        userKey,
        adminKey,
        oracleKey,
        receiverVaultKey,
        donorVaultKey,
        receiverKey,
        donorKey,
        ebKey,
        amount,
    }: ExchangeParams) {
        return new TransactionInstruction({
            keys: [
                { pubkey: userKey, isSigner: true, isWritable: false },
                { pubkey: adminKey, isSigner: false, isWritable: false },
                { pubkey: receiverVaultKey, isSigner: false, isWritable: true },
                { pubkey: donorVaultKey, isSigner: false, isWritable: true },
                { pubkey: receiverKey, isSigner: false, isWritable: true },
                { pubkey: donorKey, isSigner: false, isWritable: true },
                { pubkey: oracleKey, isSigner: false, isWritable: false },
                { pubkey: ebKey, isSigner: false, isWritable: false },
                {
                    pubkey: TOKEN_PROGRAM_ID,
                    isSigner: false,
                    isWritable: false,
                },
            ],
            programId: this.programId,
            data: Buffer.concat([
                new Uint8Array([Instruction.Exchange]),
                getu64Buffer(amount),
            ]),
        });
    }
}
