import {
    PublicKey,
    SystemProgram,
    SYSVAR_RENT_PUBKEY,
    TransactionInstruction,
} from "@solana/web3.js";
import BN from "bn.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { getu64Buffer } from "./helpers";
import {
    BOOTH_FEE,
    EXCHANGE_RATE_A_TO_B,
    FEE_DECIMALS,
    Instruction,
    RATE_DECIMALS,
} from "./const";

export type CreateEbParams = {
    adminKey: PublicKey;
    ebKey: PublicKey;
    vaultAKey: PublicKey;
    vaultBKey: PublicKey;
    oracleKey: PublicKey;
};

export type DepositEbParams = {
    adminKey: PublicKey;
    vaultAKey: PublicKey;
    vaultBKey: PublicKey;
    donorAKey: PublicKey;
    donorBKey: PublicKey;
    amountA: bigint;
    amountB: bigint;
};

export type WithdrawEbParams = {
    adminKey: PublicKey;
    vaultAKey: PublicKey;
    vaultBKey: PublicKey;
    receiverAKey: PublicKey;
    receiverBKey: PublicKey;
};

export type CloseEbParams = {
    adminKey: PublicKey;
    ebKey: PublicKey;
    oracleKey: PublicKey;
    vaultAKey: PublicKey;
    vaultBKey: PublicKey;
    receiverAKey: PublicKey;
    receiverBKey: PublicKey;
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
    ORACLE: (mintAPK: PublicKey, mintBPK: PublicKey, ownerPK: PublicKey) => [
        ownerPK.toBuffer(),
        mintAPK.toBuffer(),
        mintBPK.toBuffer(),
    ],
    EXCHANGE_BOOTH: (oraclePK: PublicKey) => [oraclePK.toBuffer()],
    VAULT: (ownerPK: PublicKey, mintPK: PublicKey) => [
        ownerPK.toBuffer(),
        mintPK.toBuffer(),
    ],
};

export class ExchangeBoothProgram {
    private readonly mintAKey: PublicKey;
    private readonly mintBKey: PublicKey;
    private readonly programId: PublicKey;

    constructor(
        mintAKey: PublicKey,
        mintBKey: PublicKey,
        programId: PublicKey
    ) {
        this.mintAKey = mintAKey;
        this.mintBKey = mintBKey;
        this.programId = programId;
    }

    public initialize({
        adminKey,
        ebKey,
        vaultAKey,
        vaultBKey,
        oracleKey,
    }: CreateEbParams) {
        const createEbIxData = Buffer.concat([
            new Uint8Array([Instruction.Initialize]),
            getu64Buffer(EXCHANGE_RATE_A_TO_B),
            Buffer.from(new Uint8Array(new BN(RATE_DECIMALS).toArray("le", 1))),
            getu64Buffer(BigInt(BOOTH_FEE)),
            Buffer.from(new Uint8Array(new BN(FEE_DECIMALS).toArray("le", 1))),
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
                { pubkey: this.mintAKey, isSigner: false, isWritable: false },
                { pubkey: this.mintBKey, isSigner: false, isWritable: false },
                { pubkey: vaultAKey, isSigner: false, isWritable: true },
                { pubkey: vaultBKey, isSigner: false, isWritable: true },
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
        vaultAKey,
        vaultBKey,
        donorAKey,
        donorBKey,
        amountA,
        amountB,
    }: DepositEbParams) {
        const depositIxData = Buffer.concat([
            new Uint8Array([Instruction.Deposit]),
            getu64Buffer(amountA),
            getu64Buffer(amountB),
        ]);
        return new TransactionInstruction({
            programId: this.programId,
            keys: [
                { pubkey: adminKey, isSigner: true, isWritable: true },
                { pubkey: vaultAKey, isSigner: false, isWritable: true },
                { pubkey: vaultBKey, isSigner: false, isWritable: true },
                {
                    pubkey: TOKEN_PROGRAM_ID,
                    isSigner: false,
                    isWritable: false,
                },
                { pubkey: donorAKey, isSigner: false, isWritable: true },
                { pubkey: donorBKey, isSigner: false, isWritable: true },
            ],
            data: depositIxData,
        });
    }

    public withdrow({
        adminKey,
        vaultAKey,
        vaultBKey,
        receiverAKey,
        receiverBKey,
    }: WithdrawEbParams) {
        return new TransactionInstruction({
            keys: [
                { pubkey: adminKey, isSigner: true, isWritable: true },
                { pubkey: vaultAKey, isSigner: false, isWritable: true },
                { pubkey: vaultBKey, isSigner: false, isWritable: true },
                { pubkey: receiverAKey, isSigner: false, isWritable: true },
                { pubkey: receiverBKey, isSigner: false, isWritable: true },
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
        vaultAKey,
        vaultBKey,
        receiverAKey,
        receiverBKey,
    }: CloseEbParams) {
        return new TransactionInstruction({
            keys: [
                { pubkey: adminKey, isSigner: true, isWritable: true },
                { pubkey: ebKey, isSigner: false, isWritable: true },
                { pubkey: vaultAKey, isSigner: false, isWritable: true },
                { pubkey: vaultBKey, isSigner: false, isWritable: true },
                { pubkey: this.mintAKey, isSigner: false, isWritable: true },
                { pubkey: this.mintBKey, isSigner: false, isWritable: true },
                { pubkey: receiverAKey, isSigner: false, isWritable: true },
                { pubkey: receiverBKey, isSigner: false, isWritable: true },
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
                { pubkey: this.mintAKey, isSigner: false, isWritable: false },
                { pubkey: this.mintBKey, isSigner: false, isWritable: false },
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
