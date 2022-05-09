import {
    Keypair,
    Connection,
    PublicKey,
    TransactionInstruction,
} from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount } from "@solana/spl-token";
import { EB_PDA_SEED_GENERATORS, ExchangeBoothProgram } from "./program";
import {
    MINT_A_DECIMALS,
    Instruction,
    MINT_B_DECIMALS,
    EXCHANGED_AMOUNT,
    Direction,
    EXCHANGE_DIRECTION,
} from "./const";

export class Processor {
    private readonly mintAKey: PublicKey;
    private readonly mintBKey: PublicKey;
    private readonly programId: PublicKey;
    private readonly connection: Connection;

    constructor(
        mintAKey: PublicKey,
        mintBKey: PublicKey,
        programId: PublicKey,
        connection: Connection
    ) {
        this.mintAKey = mintAKey;
        this.mintBKey = mintBKey;
        this.programId = programId;
        this.connection = connection;
    }

    async process(
        ix: Instruction,
        signerKeypair: Keypair,
        ebAuthority: PublicKey
    ): Promise<TransactionInstruction> {
        const mintAKey = this.mintAKey;
        const mintBKey = this.mintBKey;
        const programId = this.programId;
        const connection = this.connection;

        let program = new ExchangeBoothProgram(mintAKey, mintBKey, programId);

        const tokenAAccount = await getOrCreateAssociatedTokenAccount(
            connection,
            signerKeypair,
            mintAKey,
            signerKeypair.publicKey
        );

        const tokenBAccount = await getOrCreateAssociatedTokenAccount(
            connection,
            signerKeypair,
            mintBKey,
            signerKeypair.publicKey
        );

        const oracleKey = (
            await PublicKey.findProgramAddress(
                EB_PDA_SEED_GENERATORS.ORACLE(mintAKey, mintBKey, ebAuthority),
                programId
            )
        )[0];

        const ebKey = (
            await PublicKey.findProgramAddress(
                EB_PDA_SEED_GENERATORS.EXCHANGE_BOOTH(oracleKey),
                programId
            )
        )[0];

        const vaultAKey = (
            await PublicKey.findProgramAddress(
                EB_PDA_SEED_GENERATORS.VAULT(ebKey, mintAKey),
                programId
            )
        )[0];

        const vaultBKey = (
            await PublicKey.findProgramAddress(
                EB_PDA_SEED_GENERATORS.VAULT(ebKey, mintBKey),
                programId
            )
        )[0];

        switch (ix) {
            case Instruction.Initialize: {
                console.log("INITIALIZE");
                console.log("adminKey", ebAuthority.toBase58());
                console.log("ebKey", ebKey.toBase58());
                console.log("vaultAKey", vaultAKey.toBase58());
                console.log("vaultBKey", vaultBKey.toBase58());
                console.log("tokenAAccount", tokenAAccount.address.toBase58());
                console.log("tokenBAccount", tokenBAccount.address.toBase58());

                return program.initialize({
                    adminKey: ebAuthority,
                    ebKey,
                    vaultAKey: vaultAKey,
                    vaultBKey: vaultBKey,
                    oracleKey,
                });
            }
            case Instruction.Deposit: {
                return program.deposit({
                    adminKey: ebAuthority,
                    vaultAKey: vaultAKey,
                    vaultBKey: vaultBKey,
                    donorAKey: tokenAAccount.address,
                    donorBKey: tokenBAccount.address,
                    amountA: BigInt(10 * Math.pow(10, MINT_A_DECIMALS)),
                    amountB: BigInt(10 * Math.pow(10, MINT_B_DECIMALS)),
                });
            }
            case Instruction.Close: {
                return program.close({
                    adminKey: ebAuthority,
                    oracleKey,
                    ebKey,
                    receiverAKey: tokenAAccount.address,
                    receiverBKey: tokenBAccount.address,
                    vaultAKey: vaultAKey,
                    vaultBKey: vaultBKey,
                });
            }
            case Instruction.Exchange: {
                return program.exchange({
                    userKey: signerKeypair.publicKey,
                    adminKey: ebAuthority,
                    oracleKey,
                    ...(EXCHANGE_DIRECTION === Direction.ToB
                        ? {
                              receiverVaultKey: vaultAKey,
                              donorVaultKey: vaultBKey,
                              receiverKey: tokenBAccount.address,
                              donorKey: tokenAAccount.address,
                          }
                        : {
                              receiverVaultKey: vaultBKey,
                              donorVaultKey: vaultAKey,
                              receiverKey: tokenAAccount.address,
                              donorKey: tokenBAccount.address,
                          }),
                    ebKey,
                    amount: EXCHANGED_AMOUNT,
                });
            }
            case Instruction.Withdraw: {
                return program.withdrow({
                    adminKey: signerKeypair.publicKey,
                    vaultAKey: vaultAKey,
                    vaultBKey: vaultBKey,
                    receiverAKey: tokenAAccount.address,
                    receiverBKey: tokenBAccount.address,
                });
            }
        }
    }
}
