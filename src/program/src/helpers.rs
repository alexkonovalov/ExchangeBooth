use crate::commands::Direction;

pub fn convert(
    rate_a_to_b: u64,
    value: u64,
    fee: u64,
    direction: Direction,
    decimals_rate: u8,
    decimals_a: u8,
    decimals_b: u8,
    decimals_fee: u8,
) -> u64 {
    let fee_koeff = u64::pow(10, decimals_fee as u32) - fee;

    if direction == Direction::ToB {
        let decimals =
            decimals_b as i16 + decimals_rate as i16 - decimals_a as i16 - decimals_fee as i16;

        if decimals >= 0 {
            return u64::pow(10, decimals as u32) * value * fee_koeff / rate_a_to_b;
        } else {
            return value * fee_koeff / (u64::pow(10, -decimals as u32) * rate_a_to_b);
        }
    } else {
        let decimals =
            decimals_a as i16 - decimals_b as i16 - decimals_rate as i16 - decimals_fee as i16;

        if decimals >= 0 {
            return value * u64::pow(10, decimals as u32) * rate_a_to_b * fee_koeff;
        } else {
            return value * rate_a_to_b * fee_koeff / u64::pow(10, -decimals as u32);
        }
    }
}
