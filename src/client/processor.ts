import {
    Keypair,
    Connection,
    PublicKey,
    Transaction,
    TransactionInstruction,
} from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import {
    EB_PDA_SEED_GENERATORS,
    ExchangeBoothProgram,
    Instruction,
} from "./program";

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
        ownerKeypair: Keypair
    ): Promise<TransactionInstruction> {
        const mint1PK = this.mint1Key;
        const mint2PK = this.mint2Key;
        const programId = this.programId;
        const connection = this.connection;

        let program = new ExchangeBoothProgram(mint1PK, mint2PK, programId);

        const token1Account = await getOrCreateAssociatedTokenAccount(
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

        const vault1Key = (
            await PublicKey.findProgramAddress(
                EB_PDA_SEED_GENERATORS.VAULT1(ownerKeypair.publicKey, mint1PK),
                programId
            )
        )[0];

        const vault2Key = (
            await PublicKey.findProgramAddress(
                EB_PDA_SEED_GENERATORS.VAULT1(ownerKeypair.publicKey, mint2PK),
                programId
            )
        )[0];

        switch (ix) {
            case Instruction.Initialize: {
                const oracleKey = (
                    await PublicKey.findProgramAddress(
                        EB_PDA_SEED_GENERATORS.ORACLE(
                            mint1PK,
                            mint2PK,
                            ownerKeypair.publicKey
                        ),
                        programId
                    )
                )[0];

                const ebKey = (
                    await PublicKey.findProgramAddress(
                        EB_PDA_SEED_GENERATORS.EXCHANGE_BOOTH(oracleKey),
                        programId
                    )
                )[0];

                console.log(
                    "PREPARE INSTRUCTION INIT EXCHANGE BOOTH. ADDRESSES:"
                );
                console.log("vault:", vault1Key.toBase58());
                console.log("vault2:", vault2Key.toBase58());
                console.log("tokenAccount:", token1Account.address.toBase58());
                console.log("token2Account:", token2Account.address.toBase58());
                console.log("oracle:", oracleKey.toBase58());
                console.log("ebKey:", ebKey.toBase58());

                return program.initialize({
                    ownerKey: ownerKeypair.publicKey,
                    ebKey,
                    vault1Key,
                    vault2Key,
                    oracleKey,
                    tokenRate: 0.5,
                });
            }
            case Instruction.Deposit: {
                return program.deposit({
                    ownerKey: ownerKeypair.publicKey,
                    vault1Key,
                    vault2Key,
                    donor1Key: token1Account.address,
                    donor2Key: token2Account.address,
                    amount1: 5,
                    amount2: 5,
                });
            }
            case Instruction.Close: {
                const oracleKey = (
                    await PublicKey.findProgramAddress(
                        EB_PDA_SEED_GENERATORS.ORACLE(
                            mint1PK,
                            mint2PK,
                            ownerKeypair.publicKey
                        ),
                        programId
                    )
                )[0];

                const ebKey = (
                    await PublicKey.findProgramAddress(
                        EB_PDA_SEED_GENERATORS.EXCHANGE_BOOTH(oracleKey),
                        programId
                    )
                )[0];

                return program.close({
                    ownerKey: ownerKeypair.publicKey,
                    oracleKey,
                    ebKey,
                    receiver1Key: token1Account.address,
                    receiver2Key: token2Account.address,
                    vault1Key,
                    vault2Key,
                });
            }
            case Instruction.Exchange: {
                const oracleKey = (
                    await PublicKey.findProgramAddress(
                        EB_PDA_SEED_GENERATORS.ORACLE(
                            mint1PK,
                            mint2PK,
                            ownerKeypair.publicKey
                        ),
                        programId
                    )
                )[0];

                return program.exchange({
                    ownerKey: ownerKeypair.publicKey,
                    oracleKey,
                    receiverVaultKey: vault2Key,
                    donorVaultKey: vault1Key,
                    receiverKey: token1Account.address,
                    donorKey: token2Account.address,
                });
            }
            case Instruction.Withdraw: {
                return program.withdrow({
                    ownerKey: ownerKeypair.publicKey,
                    vault1Key,
                    vault2Key,
                    receiver1Key: token1Account.address,
                    receiver2Key: token2Account.address,
                });
            }
        }
    }
}
