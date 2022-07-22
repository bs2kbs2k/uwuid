use std::{fmt::Display, str::FromStr};

use rand::random;
use time::OffsetDateTime;

#[cfg(test)]
mod tests {
    use crate::UwuId;

    #[test]
    fn generate_uwuid() {
        let uwuid = UwuId::new();
        let uwuid_parsed: UwuId = format!("{}", uwuid).parse().unwrap();
        assert_eq!(uwuid, uwuid_parsed);
    }
}

static UWUID_HEX_DIGITS: [char; 16] = [
    'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', '🥰', '😳', '🥺', '🤗', '😍', ',',
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct UwuId(u128);

impl UwuId {
    pub fn new() -> Self {
        Self(
            (((OffsetDateTime::now_utc().unix_timestamp_nanos() as u128) / 1_000_000_u128) << 80)
                + (random::<u128>() & 0xffffffffffffffffffff),
        )
    }
    pub fn time(&self) -> Result<OffsetDateTime, time::error::ComponentRange> {
        OffsetDateTime::from_unix_timestamp_nanos((self.0 >> 80) as i128 * 1_000_000)
    }
}

impl Display for UwuId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..32 {
            let mask = 0xF << (i * 4);
            let digit = (self.0 & mask) >> (i * 4);
            write!(f, "{}", UWUID_HEX_DIGITS[digit as usize])?;
        }
        Ok(())
    }
}

impl FromStr for UwuId {
    type Err = std::fmt::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut id = 0_u128;
        for digit in s.chars().rev() {
            id <<= 4;
            id += UWUID_HEX_DIGITS
                .iter()
                .position(|e| *e == digit)
                .ok_or(std::fmt::Error)? as u128;
        }
        Ok(Self(id))
    }
}
