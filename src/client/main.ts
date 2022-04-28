/* eslint-disable @typescript-eslint/no-unsafe-assignment */
/* eslint-disable @typescript-eslint/no-unsafe-member-access */

import {
    Connection,
    sendAndConfirmTransaction,
} from '@solana/web3.js';
import { createMint,
} from '@solana/spl-token';

import path from 'path';

import * as yargs from 'yargs'
 import { hideBin } from "yargs/helpers";

import { closeAssociatedToken, mintAssociatedToken } from './service';
import { createKeypairFromFile, getConfig} from './helpers';
import { InstructionType } from './exchange_booth';
import { Processor } from './processor';


const PROGRAM_PATH = path.resolve(__dirname, '../../dist/program');
const KEYS_PATH = path.resolve(__dirname, '../../dist/keys');
const MINT1_SO_PATH = path.join(KEYS_PATH, 'mint1.so');
const MINT2_SO_PATH = path.join(KEYS_PATH, 'mint2.so');

const PROGRAM_SO_PATH = path.join(PROGRAM_PATH, 'exchange_booth.so');
const PROGRAM_KEYPAIR_PATH = path.join(PROGRAM_PATH, 'exchange_booth-keypair.json');

async function main() {
  console.log("Running solana RPC program...");
  const config = await getConfig();
  const connection = new Connection(config.json_rpc_url, 'confirmed');
  const version = await connection.getVersion();
  console.log('Connection to cluster established:', config.json_rpc_url, version);

  let myKeypair = await createKeypairFromFile(config.keypair_path);
  let mint1Keypair = await createKeypairFromFile(MINT1_SO_PATH);
  let mint2Keypair = await createKeypairFromFile(MINT2_SO_PATH);

  const args = yargs.default(hideBin(process.argv))
      .option('ix', {
        description: 'Run instruction',
        type: 'number',
        requiresArg: true,
        demandOption: false,
      })
      .option('tokens:clear', {
        description: 'Remove mint and token accounts',
        type: 'boolean',
        requiresArg: false,
        demandOption: false,
      })
      .option('tokens:create', {
        description: 'Create mint and token accounts',
        type: 'boolean',
        requiresArg: false,
        demandOption: false,
      })
      .option('airdrop', {
        description: 'Request airdrop',
        type: 'boolean',
        requiresArg: false,
        demandOption: false,
      })
      .parseSync();

  switch (true) {
    case args.ix !== undefined:

      let programKeypair = await createKeypairFromFile(PROGRAM_KEYPAIR_PATH);
      let programId = programKeypair.publicKey;

      const processor = new Processor(mint1Keypair.publicKey, mint2Keypair.publicKey, programId, connection);
      const transaction = await processor.process(args.ix as InstructionType, myKeypair);

      await sendAndConfirmTransaction(
        connection,
        transaction,
        [
           myKeypair
        ],
     );
     
      break;
    case args['airdrop']: 
      const sig = await connection.requestAirdrop(
        myKeypair.publicKey,
        1000000000
      );
      let airdropResponse = await connection.confirmTransaction(sig);
      console.log('airdropResponse', airdropResponse);
      break;
    case args['tokens:create']: 
      await mintAssociatedToken({ connection, payerKeypair: myKeypair, mintPK: mint1Keypair.publicKey, amount: 10 * Math.pow(10, 9) });
      await mintAssociatedToken({ connection, payerKeypair: myKeypair, mintPK: mint2Keypair.publicKey, amount: 10 * Math.pow(10, 9) });
      break;
    case args['mints:create']:
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
      break;
    case args['tokens:clear']:
      closeAssociatedToken({ connection, mintPK: mint1Keypair.publicKey, owner: myKeypair })
      closeAssociatedToken({ connection, mintPK: mint2Keypair.publicKey, owner: myKeypair })
      break;
    default:
      throw Error("No args provided");
  }
}

main().then(
  () => process.exit(),
  err => {
    console.error(err);
    process.exit(-1);
  },
);