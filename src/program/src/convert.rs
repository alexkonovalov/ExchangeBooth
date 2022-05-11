use crate::{commands::Direction, error::ExchangeBoothError};

pub fn convert(
    rate_a_to_b: u64,
    value: u64,
    fee: u64,
    direction: Direction,
    decimals_rate: u8,
    decimals_a: u8,
    decimals_b: u8,
    decimals_fee: u8,
) -> Result<u64, ExchangeBoothError> {
    // enlarge the var sizes to eliminate
    // potential overflow with extreme values & -+ conversions
    let rate_a_to_b = u128::from(rate_a_to_b);
    let value = u128::from(value);

    let fee_koeff = i128::pow(10, decimals_fee as u32) - fee as i128;
    if fee_koeff < 0 {
        return Err(ExchangeBoothError::FeeOverMaxError.into());
    }
    let fee_koeff = u128::try_from(fee_koeff)
        .map_err(|_| ExchangeBoothError::ConversionError)
        .unwrap();

    let decimals_a = decimals_a as i16;
    let decimals_b = decimals_b as i16;
    let decimals_fee = decimals_fee as i16;
    let decimals_rate = decimals_rate as i16;

    let product: u128;
    if direction == Direction::ToB {
        let decimals = decimals_b + decimals_rate - decimals_a - decimals_fee;

        if decimals >= 0 {
            product = u128::pow(10, decimals as u32) * value * fee_koeff / rate_a_to_b;
        } else {
            product = value * fee_koeff / (u128::pow(10, -decimals as u32) * rate_a_to_b);
        }
    } else {
        let decimals = decimals_a - decimals_b - decimals_rate - decimals_fee;

        if decimals >= 0 {
            product = value * u128::pow(10, decimals as u32) * rate_a_to_b * fee_koeff;
        } else {
            product = value * rate_a_to_b * fee_koeff / u128::pow(10, -decimals as u32);
        }
    }

    return Ok(u64::try_from(product)
        .map_err(|_| ExchangeBoothError::ConversionError)
        .unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn adjust(value: f32, decimals: u8) -> u64 {
        (value * f32::powf(10.0, decimals as f32)).floor() as u64
    }

    #[test]
    fn exchange_a_to_b() {
        let decimals_rate: u8 = 1;
        let decimals_a: u8 = 1;
        let decimals_b: u8 = 1;
        let decimals_fee: u8 = 1;

        let rate_a_to_b = adjust(0.5, decimals_rate);
        let fee = adjust(0.1, decimals_fee);
        let deposited_a: u64 = adjust(0.1, decimals_a);
        let direction: Direction = Direction::ToB;

        let expected_b = adjust(0.18, decimals_b);

        let result = convert(
            rate_a_to_b,
            deposited_a,
            fee,
            direction,
            decimals_rate,
            decimals_a,
            decimals_b,
            decimals_fee,
        )
        .unwrap();

        assert_eq!(result, expected_b);
    }

    #[test]
    fn exchange_b_to_a() {
        let decimals_rate: u8 = 1;
        let decimals_a: u8 = 1;
        let decimals_b: u8 = 1;
        let decimals_fee: u8 = 1;

        let rate_a_to_b = adjust(0.5, decimals_rate);
        let fee = adjust(0.1, decimals_fee);
        let deposited_a: u64 = adjust(0.1, decimals_a);
        let direction: Direction = Direction::ToA;

        let expected_b = adjust(0.04, decimals_b);

        let result = convert(
            rate_a_to_b,
            deposited_a,
            fee,
            direction,
            decimals_rate,
            decimals_a,
            decimals_b,
            decimals_fee,
        )
        .unwrap();

        assert_eq!(result, expected_b);
    }

    #[test]
    fn exchange_extreme_large_num() {
        let decimals_rate: u8 = 0;
        let decimals_a: u8 = 0;
        let decimals_b: u8 = 0;
        let decimals_fee: u8 = 1;

        let rate_a_to_b = 1;
        let fee = adjust(0.1, decimals_fee);
        let direction: Direction = Direction::ToB;

        let result = convert(
            rate_a_to_b,
            18400000000000000000,
            fee,
            direction,
            decimals_rate,
            decimals_a,
            decimals_b,
            decimals_fee,
        )
        .unwrap();

        assert_eq!(result, 16560000000000000000);
    }

    #[test]
    fn exchange_fee_overflow() {
        let decimals_rate: u8 = 1;
        let decimals_a: u8 = 1;
        let decimals_b: u8 = 1;
        let decimals_fee: u8 = 1;

        let rate_a_to_b = adjust(0.5, decimals_rate);
        let fee = adjust(1.1, decimals_fee);
        let deposited_a: u64 = adjust(0.1, decimals_a);
        let direction: Direction = Direction::ToA;

        let expected_error = Err(ExchangeBoothError::FeeOverMaxError.into());

        let result = convert(
            rate_a_to_b,
            deposited_a,
            fee,
            direction,
            decimals_rate,
            decimals_a,
            decimals_b,
            decimals_fee,
        );

        assert_eq!(result, expected_error);
    }
}
