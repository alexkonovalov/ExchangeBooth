/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */

import {
    Keypair,
    Connection,
    PublicKey,
    LAMPORTS_PER_SOL,
    SystemProgram,
    SYSVAR_RENT_PUBKEY,
    TransactionInstruction,
    Transaction,
    sendAndConfirmTransaction,
} from '@solana/web3.js';
import { createMint,
    getAccount,
    getMint,
    getOrCreateAssociatedTokenAccount,
    burn,
    mintTo,
    closeAccount,
    TOKEN_PROGRAM_ID,
    transfer
} from '@solana/spl-token';

import * as borsh from 'borsh';
import os from 'os';

import fs from 'mz/fs';
import path from 'path';
import yaml from 'yaml';
import { getF64Buffer } from './helpers';
import * as yargs from 'yargs'
import { hideBin } from "yargs/helpers";
import BN from 'bn.js';


export type InstructionType = 0 | 1 | 2 | 3 | 4;

export type CreateEbParams = { 
    ownerKey: PublicKey,
    ebKey: PublicKey,
    vault1Key: PublicKey,
    vault2Key: PublicKey,
    oracleKey: PublicKey,
    tokenRate: number,
}

export type DepositEbParams = { 
    ownerKey: PublicKey,
    vault1Key: PublicKey,
    vault2Key: PublicKey,
    donor1Key: PublicKey,
    donor2Key: PublicKey,
    amount1: number,
    amount2: number
}

export type WithdrawEbParams = { 
    ownerKey: PublicKey,
    vault1Key: PublicKey,
    vault2Key: PublicKey,
    receiver1Key: PublicKey,
    receiver2Key: PublicKey,
}

export type CloseEbParams = {
    ownerKey: PublicKey,
    ebKey: PublicKey,
    oracleKey: PublicKey,
    vault1Key: PublicKey,
    vault2Key: PublicKey,
    receiver1Key: PublicKey,
    receiver2Key: PublicKey,
}

export type ExchangeParams = {
    ownerKey: PublicKey,
    oracleKey: PublicKey,
    receiverVaultKey: PublicKey,
    donorVaultKey: PublicKey,
    receiverKey: PublicKey,
    donorKey: PublicKey,
}

export const EB_PDA_SEED_GENERATORS = {
    ORACLE: (mint1PK: PublicKey, mint2PK: PublicKey, ownerPK: PublicKey) => [
        ownerPK.toBuffer(),
        mint1PK.toBuffer(),
        mint2PK.toBuffer(),
      ],
    EXCHANGE_BOOTH: (oraclePK: PublicKey) => [
        oraclePK.toBuffer(),
    ],
    VAULT1: (ownerPK: PublicKey, mint1PK: PublicKey) => [
        ownerPK.toBuffer(), mint1PK.toBuffer(),
    ],
    VAULT2: (ownerPK: PublicKey, mint2PK: PublicKey) => [
        ownerPK.toBuffer(), mint2PK.toBuffer(),
    ]
}

export class ExchangeBoothProgram {
    private readonly mint1Key: PublicKey;
    private readonly mint2Key: PublicKey;
    private readonly programId: PublicKey;

    constructor(mint1Key: PublicKey, mint2Key: PublicKey, programId: PublicKey) {
        this.mint1Key = mint1Key;
        this.mint2Key = mint2Key;
        this.programId = programId;
    }

    public create({ ownerKey: payerKey, ebKey, vault1Key, vault2Key, oracleKey }: CreateEbParams) {
        const createEbIxData = Buffer.concat([new Uint8Array([0]), getF64Buffer(0.5)]);
        return new TransactionInstruction({
          keys: [
            { pubkey: payerKey, isSigner: true, isWritable: true },
            { pubkey: ebKey, isSigner: false, isWritable: true },
            { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
            { pubkey: this.mint1Key, isSigner: false, isWritable: false },
            { pubkey: this.mint2Key, isSigner: false, isWritable: false },
            { pubkey: vault1Key, isSigner: false, isWritable: true },
            { pubkey: vault2Key, isSigner: false, isWritable: true },
            { pubkey: oracleKey, isSigner: false, isWritable: true },
            { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
            { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
          ],
          programId: this.programId,
          data: Buffer.from(createEbIxData),
        });
    }

    public deposit({ ownerKey, vault1Key, vault2Key, donor1Key, donor2Key, amount1, amount2 }: DepositEbParams) {
        const depositIxData = Buffer.concat([new Uint8Array([1]), getF64Buffer(amount1), getF64Buffer(amount2)]);
        return new TransactionInstruction({
            programId: this.programId,
            keys: [
                { pubkey: ownerKey, isSigner: true, isWritable: true },
                { pubkey: vault1Key, isSigner: false, isWritable: true },
                { pubkey: vault2Key, isSigner: false, isWritable: true },
                { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
                { pubkey: donor1Key, isSigner: false, isWritable: true },
                { pubkey: donor2Key, isSigner: false, isWritable: true },
            ],
            data: depositIxData,
        });
    }
    public withdrow({ ownerKey, vault1Key, vault2Key, receiver1Key, receiver2Key }: WithdrawEbParams) {
        return new TransactionInstruction({
            keys: [
              { pubkey: ownerKey, isSigner: true, isWritable: true },
              { pubkey: vault1Key, isSigner: false, isWritable: true },
              { pubkey: vault2Key, isSigner: false, isWritable: true },
              { pubkey: receiver1Key, isSigner: false, isWritable: true },
              { pubkey: receiver2Key, isSigner: false, isWritable: true },
              { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
            ],
            programId: this.programId,
            data:Buffer.concat([new Uint8Array([4])]),
          });
    }

    public close({ ownerKey, ebKey, oracleKey, vault1Key, vault2Key, receiver1Key, receiver2Key } : CloseEbParams) {
        return new TransactionInstruction({
            keys: [
              { pubkey: ownerKey, isSigner: true, isWritable: true },
              { pubkey: ebKey, isSigner: false, isWritable: true },
              { pubkey: vault1Key, isSigner: false, isWritable: true },
              { pubkey: vault2Key, isSigner: false, isWritable: true },
              { pubkey: this.mint1Key, isSigner: false, isWritable: true },
              { pubkey: this.mint2Key, isSigner: false, isWritable: true },
              { pubkey: receiver1Key, isSigner: false, isWritable: true },
              { pubkey: receiver2Key, isSigner: false, isWritable: true },
              { pubkey: oracleKey, isSigner: false, isWritable: true },
              { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
            ],
            programId: this.programId,
            data: Buffer.from(new Uint8Array([2])),
        });
    }

    public exchange({ ownerKey, oracleKey, receiverVaultKey, donorVaultKey, receiverKey, donorKey } :ExchangeParams) {
        return new TransactionInstruction({
            keys: [
              { pubkey:ownerKey, isSigner: true, isWritable: true },
              { pubkey: receiverVaultKey, isSigner: false, isWritable: true },
              { pubkey: donorVaultKey, isSigner: false, isWritable: true },
              { pubkey: receiverKey, isSigner: false, isWritable: true },
              { pubkey: donorKey, isSigner: false, isWritable: true },
              { pubkey: oracleKey, isSigner: false, isWritable: false },
              { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
            ],
            programId: this.programId,
            data:Buffer.concat([new Uint8Array([3]), getF64Buffer(2)]),
          });
    }
}