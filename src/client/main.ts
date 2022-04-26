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
import { createMint, getAccount, getMint, getOrCreateAssociatedTokenAccount, mintTo, TOKEN_PROGRAM_ID, } from '@solana/spl-token';

import * as borsh from 'borsh';
import os from 'os';
import fs from 'mz/fs';
import path from 'path';
import yaml from 'yaml';
import { getMessageVecBuffer, getF64Buffer } from './commands';
import BN from 'bn.js';

const PROGRAM_PATH = path.resolve(__dirname, '../../dist/program');
const KEYS_PATH = path.resolve(__dirname, '../../dist/keys');
const MINT1_SO_PATH = path.join(KEYS_PATH, 'mint1.so');
const MINT2_SO_PATH = path.join(KEYS_PATH, 'mint2.so');

const PROGRAM_SO_PATH = path.join(PROGRAM_PATH, 'exchange_booth.so');
const PROGRAM_KEYPAIR_PATH = path.join(PROGRAM_PATH, 'exchange_booth-keypair.json');

/**
 * The state of a greeting account managed by the hello world program
 */
 class GreetingAccount {
  counter = 0;
 // data = Uint8Array.of( 0, 0, 0, 0, 0, 0, 0, 0 );
  data = [0, 0, 0, 0, 0, 0, 0, 0];
  constructor(fields: {counter: number, data: [8]} | undefined = undefined) {
    if (fields) {
      this.counter = fields.counter;
      this.data = fields.data;
    }
  }
}

type InstructionType = 0 | 1 | 2 | 3;

/**
 * Borsh schema definition for greeting accounts
 */
const GreetingSchema = new Map([
  [GreetingAccount, {kind: 'struct', fields: [['counter', 'u32'],
 ['data', [8]]
]}],
]);

/**
 * The expected size of each greeting account.
 */
const GREETING_SIZE = borsh.serialize(
  GreetingSchema,
  new GreetingAccount(),
).length;

async function getConfig(): Promise<any> {
  // Path to Solana CLI config file
  const CONFIG_FILE_PATH = path.resolve(
    os.homedir(),
    '.config',
    'solana',
    'cli',
    'config.yml',
  );
  const configYml = await fs.readFile(CONFIG_FILE_PATH, {encoding: 'utf8'});
  return yaml.parse(configYml);
}

export async function createKeypairFromFile(
  filePath: string,
): Promise<Keypair> {
  const secretKeyString = await fs.readFile(filePath, {encoding: 'utf8'});
  console.log('secretKeyString', secretKeyString);

  const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
  return Keypair.fromSecretKey(secretKey);
}

class Assignable {
  constructor(properties: { [x: string]: any; x?: number; y?: number; z?: string; q?: number[]; }) {
      Object.keys(properties).map((key) => {
        let me: any = this;
        me[key] = properties[key];
      });
  }
}

class Test extends Assignable { }

async function main() {
  //const value = new Test({ x: 255, y: 20, z: '123', q: [1, 2, 3] });
  //const schema = new Map([[Test, { kind: 'struct', fields: [['x', 'u8'], ['y', 'u64'], ['z', 'string'], ['q', [3]]] }]]);

  // const value = new Test({ x: 255 });
  // const schema = new Map([[Test, { kind: 'struct', fields: [['x', 'u32']] }]]);

  // const buffer = borsh.serialize(schema, value);
  // console.log('buffer::', buffer);

  console.log("Running solana RPC program...");

  const config = await getConfig();
  console.log('config:', config);

  let myKeypair = await createKeypairFromFile(config.keypair_path);
  console.log('myKeypair', config.json_rpc_url);
  const connection = new Connection(config.json_rpc_url, 'confirmed');

  const version = await connection.getVersion();
  console.log('Connection to cluster established:', config.json_rpc_url, version);


  var args = process.argv.slice(2);
  const ix = parseInt(args[0]) as InstructionType;

  await callProgram(connection, ix);
}

async function doAirdrop (connection: Connection) {
  const config = await getConfig();
  let myKeypair = await createKeypairFromFile(config.keypair_path);
  let programKeypair = await createKeypairFromFile(PROGRAM_KEYPAIR_PATH);
  let programId = programKeypair.publicKey;
  console.log('programId', programId);
  const sig = await connection.requestAirdrop(
      myKeypair.publicKey,
      1000000000
  );
  let airdropResponse = await connection.confirmTransaction(sig);
  console.log('airdropResponse', airdropResponse);
}

async function callProgram (connection: Connection, ix: InstructionType) {
  const config = await getConfig();
  let myKeypair = await createKeypairFromFile(config.keypair_path);
  let programKeypair = await createKeypairFromFile(PROGRAM_KEYPAIR_PATH);
  let mint1Keypair = await createKeypairFromFile(MINT1_SO_PATH);
  let mint2Keypair = await createKeypairFromFile(MINT2_SO_PATH);
  let programId = programKeypair.publicKey;
  console.log('programId', programId);

  
  let buffer = Buffer.alloc(8);
  let data = new Uint8Array([44, 55, 66, 777, 1, 34, 9, 78]);
  buffer.fill(data);

  const GREETING_SEED = 'seeme11';
  const greetedPubkey = await PublicKey.createWithSeed(
    myKeypair.publicKey,
    GREETING_SEED,
    programId,
  );

  //console.log('//////Greeted pubkey Base58', greetedPubkey.toBase58());
  //console.log('//////Greeted pubkey is on curve', PublicKey.isOnCurve(greetedPubkey.toBytes()));
  //console.log('//////My Keypair is on curve', PublicKey.isOnCurve(myKeypair.publicKey.toBytes()));
  
  let storageCreationIntruction = SystemProgram.createAccountWithSeed({
    fromPubkey: myKeypair.publicKey,
    basePubkey: myKeypair.publicKey,
    seed: GREETING_SEED,
    newAccountPubkey: greetedPubkey,
    lamports: 10000000,
    space: GREETING_SIZE + 32,
    programId, 
  });

  let trans = new Transaction();
  const greet_key_2 = Keypair.generate();

  const sig = await connection.requestAirdrop(
      greet_key_2.publicKey,
      1000000000
  );

  let airdropResponse = await connection.confirmTransaction(sig);
  console.log('airdropResponse', airdropResponse);

  let echoData = getMessageVecBuffer("echo");

  const commandData = Buffer.concat([Buffer.from(new Uint8Array([1])),echoData]);

  const echoInstruction = new TransactionInstruction({
    keys: [
      { pubkey: greetedPubkey, isSigner: true, isWritable: true },
      { pubkey: myKeypair.publicKey, isSigner: false, isWritable: true },
      { pubkey: greet_key_2.publicKey, isSigner: true, isWritable: false }
    ],
    programId,
    data: commandData,
  });

  if (ix === 0) {
    await createMint(
      connection,
      myKeypair,
      myKeypair.publicKey,
      myKeypair.publicKey,
      9,
      mint1Keypair
    );
  
    await createMint(
      connection,
      myKeypair,
      myKeypair.publicKey,
      myKeypair.publicKey,
      9,
      mint2Keypair
    );
  }

  let mintInfo = await getMint(
    connection,
    mint1Keypair.publicKey,
  );
  console.log('mint info 1 ---', mintInfo);

  let mint2Info = await getMint(
    connection,
    mint2Keypair.publicKey,
  );
  console.log('mint2Info ---', mint2Info);

  const tokenAccount = await getOrCreateAssociatedTokenAccount(
    connection,
    myKeypair,
    mint1Keypair.publicKey,
    myKeypair.publicKey
  );

  const token2Account = await getOrCreateAssociatedTokenAccount(
    connection,
    myKeypair,
    mint2Keypair.publicKey,
    myKeypair.publicKey
  );

  if (ix === 0) {
    await mintTo(
      connection,
      myKeypair,
      mint1Keypair.publicKey,
      tokenAccount.address,
      myKeypair.publicKey,
      10 * Math.pow(10, 9)
    );

    await mintTo(
      connection,
      myKeypair,
      mint2Keypair.publicKey,
      token2Account.address,
      myKeypair.publicKey,
      10 * Math.pow(10, 9)
    );
  }

  const oracleKey = (await PublicKey.findProgramAddress(
    [
      myKeypair.publicKey.toBuffer(),
      mint1Keypair.publicKey.toBuffer(),
      mint2Keypair.publicKey.toBuffer(),
    ],
    programId
  ))[0];


  const ebKey = (await PublicKey.findProgramAddress(
    [
      oracleKey.toBuffer()
    ],
    programId
  ))[0];

  const vault1Key = (await PublicKey.findProgramAddress(
    [myKeypair.publicKey.toBuffer(), mint1Keypair.publicKey.toBuffer()],
    programId
  ))[0];

  const vault2Key = (await PublicKey.findProgramAddress(
    [myKeypair.publicKey.toBuffer(), mint2Keypair.publicKey.toBuffer()],
    programId
  ))[0];

  const createEbIxData = Buffer.concat([new Uint8Array([0]), getF64Buffer(0.5)]);
  const createEbInstruction = new TransactionInstruction({
    keys: [
      { pubkey: myKeypair.publicKey, isSigner: true, isWritable: true },
      { pubkey: ebKey, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      { pubkey: mint1Keypair.publicKey, isSigner: false, isWritable: false },
      { pubkey: mint2Keypair.publicKey, isSigner: false, isWritable: false },
      { pubkey: vault1Key, isSigner: false, isWritable: true },
      { pubkey: vault2Key, isSigner: false, isWritable: true },
      { pubkey: oracleKey, isSigner: false, isWritable: true },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false },
    ],
    programId,
    data: Buffer.from(createEbIxData),
  });

  const depositIxData = Buffer.concat([new Uint8Array([1]), getF64Buffer(5), getF64Buffer(5)]);

  console.log('depositIxData', createEbIxData);
  const depositInstruction = new TransactionInstruction({
    programId,
    keys: [
      { pubkey: myKeypair.publicKey, isSigner: true, isWritable: true },
      { pubkey: vault1Key, isSigner: false, isWritable: true },
      { pubkey: vault2Key, isSigner: false, isWritable: true },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: tokenAccount.address, isSigner: false, isWritable: true },
      { pubkey: token2Account.address, isSigner: false, isWritable: true },
    ],
    data: depositIxData,
  });

  console.log('\\\\vault:', vault1Key.toBase58());
  console.log('\\\\vault2:', vault2Key.toBase58());
  console.log('\\\\tokenAccount:', tokenAccount.address.toBase58());
  console.log('\\\\token2Account:', token2Account.address.toBase58());
  console.log('\\\\oracle:', oracleKey.toBase58());
  console.log('\\\\ebKey:', ebKey.toBase58());

  const closeEbInstruction = new TransactionInstruction({
    keys: [
      { pubkey: myKeypair.publicKey, isSigner: true, isWritable: true },
      { pubkey: ebKey, isSigner: false, isWritable: true },
      { pubkey: vault1Key, isSigner: false, isWritable: true },
      { pubkey: vault2Key, isSigner: false, isWritable: true },
      { pubkey: mint1Keypair.publicKey, isSigner: false, isWritable: true },
      { pubkey: mint2Keypair.publicKey, isSigner: false, isWritable: true },
      { pubkey: tokenAccount.address, isSigner: false, isWritable: true },
      { pubkey: token2Account.address, isSigner: false, isWritable: true },
      { pubkey: oracleKey, isSigner: false, isWritable: true },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    ],
    programId,
    data: Buffer.from(new Uint8Array([2])),
  });

  const exchangeInstruction = new TransactionInstruction({
    keys: [
      { pubkey: myKeypair.publicKey, isSigner: true, isWritable: true },
      { pubkey: vault1Key, isSigner: false, isWritable: true },
      { pubkey: vault2Key, isSigner: false, isWritable: true },
      { pubkey: token2Account.address, isSigner: false, isWritable: true },
      { pubkey: tokenAccount.address, isSigner: false, isWritable: true },
      { pubkey: oracleKey, isSigner: false, isWritable: false },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    ],
    programId,
    data:Buffer.concat([new Uint8Array([3]), getF64Buffer(1)]),
  });


  switch (ix) {
    case 0: {
      trans.instructions = [createEbInstruction];
      break;
    }
    case 1: {
      trans.instructions = [depositInstruction];
      break;
    }
    case 2: {
      trans.instructions = [closeEbInstruction];
      break;
    }
    case 3: {
      trans.instructions = [exchangeInstruction];
      break;
    }
  }

  await sendAndConfirmTransaction(
     connection,
     trans,
     [
        myKeypair
    ],
  );

  const greet_2 = await connection.getAccountInfo(myKeypair.publicKey);
}


main().then(
  () => process.exit(),
  err => {
    console.error(err);
    process.exit(-1);
  },
);