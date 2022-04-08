/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */

import {
    Keypair,
    Connection,
    PublicKey,
    LAMPORTS_PER_SOL,
    SystemProgram,
    TransactionInstruction,
    Transaction,
    sendAndConfirmTransaction,
} from '@solana/web3.js';
import { createMint, getAccount, getMint, getOrCreateAssociatedTokenAccount, mintTo } from '@solana/spl-token';

import * as borsh from 'borsh';
import os from 'os';
import fs from 'mz/fs';
import path from 'path';
import yaml from 'yaml';
import { getMessageVec } from './commands';

const PROGRAM_PATH = path.resolve(__dirname, '../../dist/program');
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
  const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
  return Keypair.fromSecretKey(secretKey);
}

async function main() {
  console.log("Running solana RPC program...");

  const config = await getConfig();
  console.log('config:', config);

  let myKeypair = await createKeypairFromFile(config.keypair_path);
  console.log('rpcUrl', config.json_rpc_url);
  const connection = new Connection(config.json_rpc_url, 'confirmed');

  const version = await connection.getVersion();
  console.log('Connection to cluster established:', config.json_rpc_url, version);

  await callProgram(connection);
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

async function callProgram (connection: Connection) {
  const config = await getConfig();
  let myKeypair = await createKeypairFromFile(config.keypair_path);
  let programKeypair = await createKeypairFromFile(PROGRAM_KEYPAIR_PATH);
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

  console.log('//////Greeted pubkey Base58', greetedPubkey.toBase58());
  console.log('//////Greeted pubkey is on curve', PublicKey.isOnCurve(greetedPubkey.toBytes()));
  console.log('//////My Keypair is on curve', PublicKey.isOnCurve(myKeypair.publicKey.toBytes()));
  
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

  let echoData = getMessageVec("echo");
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


  const mintAuthority = Keypair.generate();
  const freezeAuthority = Keypair.generate();

  const mint = await createMint(
    connection,
    myKeypair,
    mintAuthority.publicKey,
    freezeAuthority.publicKey,
    9
  );

  let mintInfo = await getMint(
    connection,
    mint
  )
  
  console.log("MINT Key:", mint.toBase58());
  console.log("MINT Info:", mintInfo);

  const tokenAccount = await getOrCreateAssociatedTokenAccount(
    connection,
    myKeypair,
    mint,
    myKeypair.publicKey
  );

  await mintTo(
    connection,
    myKeypair,
    mint,
    tokenAccount.address,
    mintAuthority,
    100
  );

  const tokenAccountInfo = await getAccount(
    connection,
    tokenAccount.address
  )

  mintInfo = await getMint(
    connection,
    mint
  )

  console.log("tokenaccount", tokenAccount.address.toBase58());
  console.log("NEW MINT Info.supply", mintInfo.supply);
  console.log("NEW tokenAccountInfo Info.amount", tokenAccountInfo.amount);

  const account = await connection.getAccountInfo(tokenAccount.address);
  console.log('Token account:', account);
  console.log('Token account owner pk:', account?.owner.toBase58());
  console.log('MY pk:', myKeypair.publicKey.toBase58());

  //todo start EB instruction

  const ebKey = (await PublicKey.findProgramAddress(
    [myKeypair.publicKey.toBuffer()],
    programId
  ))[0];

  const createEbInstruction = new TransactionInstruction({
    keys: [
      { pubkey: myKeypair.publicKey, isSigner: true, isWritable: true },
      { pubkey: ebKey, isSigner: false, isWritable: true },
      {
        pubkey: SystemProgram.programId,
        isSigner: false,
        isWritable: false,
      },
    ],
    programId,
    data: Buffer.from(new Uint8Array([2])),
  });

  trans.instructions = [
    createEbInstruction
  ];

  await sendAndConfirmTransaction(
     connection,
     trans,
     [
       //greet_key_2,
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