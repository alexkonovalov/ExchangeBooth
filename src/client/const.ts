export enum Instruction {
    Initialize = 0,
    Deposit = 1,
    Close = 2,
    Exchange = 3,
    Withdraw = 4,
}

export enum Direction {
    ToA,
    ToB,
}

//COMMAND VALUES
//TODO should be replaced by parameters

//initialization
export const RATE_DECIMALS = 2;
export const FEE_DECIMALS = 2;
export const MINT_A_DECIMALS = 9;
export const MINT_B_DECIMALS = 9;

export const BOOTH_FEE = BigInt(0.1 * Math.pow(10, FEE_DECIMALS));
export const EXCHANGE_RATE_A_TO_B = BigInt(0.5 * Math.pow(10, RATE_DECIMALS));

//deposit
export const DEPOSIT_A_VALUE = BigInt(10 * Math.pow(10, MINT_A_DECIMALS));
export const DEPOSIT_B_VALUE = BigInt(10 * Math.pow(10, MINT_B_DECIMALS));

//exchange
export const EXCHANGE_DIRECTION = Direction.ToA as Direction;
export const EXCHANGED_AMOUNT = BigInt(
    2 *
        Math.pow(
            10,
            EXCHANGE_DIRECTION === Direction.ToB
                ? MINT_B_DECIMALS
                : MINT_A_DECIMALS
        )
);
