import {
    Keypair,
    Connection,
    PublicKey,
    TransactionInstruction,
} from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import { EB_PDA_SEED_GENERATORS, ExchangeBoothProgram } from "./program";
import { DECIMALS, Instruction, TOKEN_A_TO_B_RATE } from "./const";

export class Processor {
    private readonly mint1Key: PublicKey;
    private readonly mint2Key: PublicKey;
    private readonly programId: PublicKey;
    private readonly connection: Connection;

    constructor(
        mint1Key: PublicKey,
        mint2Key: PublicKey,
        programId: PublicKey,
        connection: Connection
    ) {
        this.mint1Key = mint1Key;
        this.mint2Key = mint2Key;
        this.programId = programId;
        this.connection = connection;
    }

    async process(
        ix: Instruction,
        signerKeypair: Keypair,
        ebAuthority: PublicKey
    ): Promise<TransactionInstruction> {
        const mint1PK = this.mint1Key;
        const mint2PK = this.mint2Key;
        const programId = this.programId;
        const connection = this.connection;

        let program = new ExchangeBoothProgram(mint1PK, mint2PK, programId);

        const token1Account = await getOrCreateAssociatedTokenAccount(
            connection,
            signerKeypair,
            mint1PK,
            signerKeypair.publicKey
        );

        const token2Account = await getOrCreateAssociatedTokenAccount(
            connection,
            signerKeypair,
            mint2PK,
            signerKeypair.publicKey
        );

        const oracleKey = (
            await PublicKey.findProgramAddress(
                EB_PDA_SEED_GENERATORS.ORACLE(mint1PK, mint2PK, ebAuthority),
                programId
            )
        )[0];

        const ebKey = (
            await PublicKey.findProgramAddress(
                EB_PDA_SEED_GENERATORS.EXCHANGE_BOOTH(oracleKey),
                programId
            )
        )[0];

        const vault1Key = (
            await PublicKey.findProgramAddress(
                EB_PDA_SEED_GENERATORS.VAULT1(ebKey, mint1PK),
                programId
            )
        )[0];

        const vault2Key = (
            await PublicKey.findProgramAddress(
                EB_PDA_SEED_GENERATORS.VAULT1(ebKey, mint2PK),
                programId
            )
        )[0];

        switch (ix) {
            case Instruction.Initialize: {
                console.log("INITIALIZE");
                console.log("adminKey", ebAuthority.toBase58());
                console.log("ebKey", ebKey.toBase58());
                console.log("vault1Key", vault1Key.toBase58());
                console.log("vault2Key", vault2Key.toBase58());
                console.log("token1Account", token1Account.address.toBase58());
                console.log("token2Account", token2Account.address.toBase58());

                return program.initialize({
                    adminKey: ebAuthority,
                    ebKey,
                    vault1Key,
                    vault2Key,
                    oracleKey,
                    tokenRate: TOKEN_A_TO_B_RATE,
                });
            }
            case Instruction.Deposit: {
                return program.deposit({
                    adminKey: ebAuthority,
                    vault1Key,
                    vault2Key,
                    donor1Key: token1Account.address,
                    donor2Key: token2Account.address,
                    amount_a: BigInt(10 * Math.pow(10, DECIMALS)),
                    amount_b: BigInt(10 * Math.pow(10, DECIMALS)),
                });
            }
            case Instruction.Close: {
                return program.close({
                    adminKey: ebAuthority,
                    oracleKey,
                    ebKey,
                    receiver1Key: token1Account.address,
                    receiver2Key: token2Account.address,
                    vault1Key,
                    vault2Key,
                });
            }
            case Instruction.Exchange: {
                return program.exchange({
                    userKey: signerKeypair.publicKey,
                    adminKey: ebAuthority,
                    oracleKey,
                    receiverVaultKey: vault2Key,
                    donorVaultKey: vault1Key,
                    receiverKey: token1Account.address,
                    donorKey: token2Account.address,
                    ebKey,
                    amount: BigInt(1 * Math.pow(10, DECIMALS)),
                });
            }
            case Instruction.Withdraw: {
                return program.withdrow({
                    adminKey: signerKeypair.publicKey,
                    vault1Key,
                    vault2Key,
                    receiver1Key: token1Account.address,
                    receiver2Key: token2Account.address,
                });
            }
        }
    }
}
