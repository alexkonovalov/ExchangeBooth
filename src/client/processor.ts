import {
    Keypair,
    Connection,
    PublicKey,
    Transaction,
    sendAndConfirmTransaction,
} from '@solana/web3.js';
import { createMint,
    getOrCreateAssociatedTokenAccount,
} from '@solana/spl-token';

import path from 'path';

import * as yargs from 'yargs'
 import { hideBin } from "yargs/helpers";

import { closeAssociatedToken, mintAssociatedToken } from './service';
import { createKeypairFromFile, getConfig} from './helpers';
import { EB_PDA_SEED_GENERATORS, ExchangeBoothProgram, Instruction } from './exchange_booth';
import { Key } from 'mz/readline';

export class Processor {
    private readonly mint1Key: PublicKey;
    private readonly mint2Key: PublicKey;
    private readonly programId: PublicKey;
    private readonly connection: Connection;
    
    constructor(mint1Key: PublicKey, mint2Key: PublicKey, programId: PublicKey, connection: Connection) {
        this.mint1Key = mint1Key;
        this.mint2Key = mint2Key;
        this.programId = programId;
        this.connection = connection;
    }

    async process(ix: Instruction, ownerKeypair: Keypair): Promise<Transaction> {
        const mint1PK = this.mint1Key;
        const mint2PK = this.mint2Key;
        const programId = this.programId;
        const connection = this.connection;

        let ebProgram = new ExchangeBoothProgram(mint1PK, mint2PK, programId);

        const tokenAccount = await getOrCreateAssociatedTokenAccount(
            connection,
            ownerKeypair,
            mint1PK,
            ownerKeypair.publicKey
        );

        const token2Account = await getOrCreateAssociatedTokenAccount(
            connection,
            ownerKeypair,
            mint2PK,
            ownerKeypair.publicKey
        );

        const vault1Key = (await PublicKey.findProgramAddress(
            EB_PDA_SEED_GENERATORS.VAULT1(ownerKeypair.publicKey, mint1PK),
            programId
        ))[0];

        const vault2Key = (await PublicKey.findProgramAddress(
            EB_PDA_SEED_GENERATORS.VAULT1(ownerKeypair.publicKey, mint2PK),
            programId
        ))[0];

        let transaction = new Transaction();

        switch (ix) {
            case Instruction.Initialize: {
            const oracleKey = (await PublicKey.findProgramAddress(
                EB_PDA_SEED_GENERATORS.ORACLE(mint1PK, mint2PK, ownerKeypair.publicKey),
                programId
            ))[0];
        
            const ebKey = (await PublicKey.findProgramAddress(
                EB_PDA_SEED_GENERATORS.EXCHANGE_BOOTH(oracleKey),
                programId
            ))[0];

            transaction.instructions = [ebProgram.create({
                ownerKey: ownerKeypair.publicKey,
                ebKey,
                vault1Key,
                vault2Key,
                oracleKey,
                tokenRate: 0.5
            })];

            console.log('INITIALISED EXCHANGE BOOTH. ADDRESSES:', vault1Key.toBase58());
            console.log('vault:', vault1Key.toBase58());
            console.log('vault2:', vault2Key.toBase58());
            console.log('tokenAccount:', tokenAccount.address.toBase58());
            console.log('token2Account:', token2Account.address.toBase58());
            console.log('oracle:', oracleKey.toBase58());
            console.log('ebKey:', ebKey.toBase58());

            break;
            }
            case Instruction.Deposit: {
            transaction.instructions = [ebProgram.deposit({ 
                ownerKey: ownerKeypair.publicKey,
                vault1Key,
                vault2Key,
                donor1Key: tokenAccount.address,
                donor2Key: token2Account.address,
                amount1: 5,
                amount2: 5,
            })];
            break;
            }
            case Instruction.Close: {
            const oracleKey = (await PublicKey.findProgramAddress(
                EB_PDA_SEED_GENERATORS.ORACLE(mint1PK, mint2PK, ownerKeypair.publicKey),
                programId
            ))[0];

            const ebKey = (await PublicKey.findProgramAddress(
                EB_PDA_SEED_GENERATORS.EXCHANGE_BOOTH(oracleKey),
                programId
            ))[0];

            transaction.instructions = [ebProgram.close({ 
                ownerKey: ownerKeypair.publicKey,
                oracleKey,
                ebKey,
                receiver1Key: tokenAccount.address,
                receiver2Key: token2Account.address,
                vault1Key,
                vault2Key
            })];
            break;
            }
            case Instruction.Exchange: {
            const oracleKey = (await PublicKey.findProgramAddress(
                EB_PDA_SEED_GENERATORS.ORACLE(mint1PK, mint2PK, ownerKeypair.publicKey),
                programId
            ))[0];

            transaction.instructions = [ebProgram.exchange({
                ownerKey: ownerKeypair.publicKey,
                oracleKey,
                receiverVaultKey:vault2Key,
                donorVaultKey: vault1Key,
                receiverKey: tokenAccount.address,
                donorKey: token2Account.address,
            })];
            break;
            }
            case Instruction.Withdraw: {
            transaction.instructions = [ebProgram.withdrow({
                ownerKey: ownerKeypair.publicKey,
                vault1Key,
                vault2Key,
                receiver1Key: tokenAccount.address,
                receiver2Key: token2Account.address,
            })];
            break;
            }
        }
        return transaction;
    }
}