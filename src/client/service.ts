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
import { getMessageVecBuffer, getF64Buffer } from './helpers';
import * as yargs from 'yargs'
import { hideBin } from "yargs/helpers";
import BN from 'bn.js';

export async function mintAssociatedToken({connection, payerKeypair, mintPK, amount }:{connection: Connection, payerKeypair: Keypair, mintPK: PublicKey, amount: number | bigint}) {
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
      payerKeypair.publicKey,
      amount
    );
  }
  
  export async function closeAssociatedToken({connection, mintPK, owner }: {
    connection: Connection,
    mintPK: PublicKey,
    owner: Keypair
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
  