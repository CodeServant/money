use std::{collections::HashMap, fmt::Display, hash::Hash, sync::LazyLock};

use rust_decimal::Decimal;

/// Currency standard.
pub trait CurrencyStd: Hash + Eq {}

type MinorUnitsType = u32;

/// Currency standard that have known decimal minor units like Crypto or ISO Currencies.
pub trait CurrencyWIthMinorStd: CurrencyStd {
    fn minor_units(&self) -> MinorUnitsType;
}

type ISONumericType = u32;

pub trait HasISO4217Code<'a> {
    fn alphabetic_iso(&'a self) -> &'a ISO4217Alphabetic;
    fn numeric_iso(&self) -> ISONumericType;
}

/// Code string like USD, EUR, PLN etc.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ISO4217Alphabetic {
    code: String,
}

impl From<ISO4217Alphabetic> for String {
    fn from(value: ISO4217Alphabetic) -> Self {
        value.code
    }
}

impl Display for ISO4217Alphabetic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code)
    }
}

#[derive(Debug)]
pub struct WrongISOAlpha {
    /// Actual iso code that were provided.
    pub used: String,
    pub reason: WrongISOReason,
}

#[derive(Debug, PartialEq)]
pub enum WrongISOReason {
    Length(usize),
    NonAlphabetic,
}

impl TryFrom<String> for ISO4217Alphabetic {
    type Error = WrongISOAlpha;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let prepared = value.trim().to_uppercase();
        let len = prepared.chars().count();
        if len != 3 {
            return Err(WrongISOAlpha {
                used: value,
                reason: WrongISOReason::Length(len),
            });
        }

        for c in prepared.chars() {
            if !c.is_alphabetic() {
                return Err(WrongISOAlpha {
                    used: value,
                    reason: WrongISOReason::NonAlphabetic,
                });
            };
        }

        Ok(ISO4217Alphabetic { code: prepared })
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ISO4217Currency {
    iso_alpha: ISO4217Alphabetic,
    iso_numeric: ISONumericType,
    minor_units: MinorUnitsType,
}

impl<'a> HasISO4217Code<'a> for ISO4217Currency {
    fn alphabetic_iso(&'a self) -> &'a ISO4217Alphabetic {
        &self.iso_alpha
    }

    fn numeric_iso(&self) -> ISONumericType {
        self.iso_numeric
    }
}

impl CurrencyStd for ISO4217Currency {}

impl CurrencyWIthMinorStd for ISO4217Currency {
    fn minor_units(&self) -> MinorUnitsType {
        self.minor_units
    }
}

/// As of 2026 2 countries have non decimal rounding MGA and MRU.
#[cfg(feature = "rounding_exceptions")]
const rounding_exceptions: LazyLock<HashMap<ISONumericType, Decimal>> = LazyLock::new(|| {
    let mut map = HashMap::<ISONumericType, Decimal>::new();
    const MGA: u32 = 969;
    const MRU: u32 = 929;
    let min_change = Decimal::new(2, 1);
    map.insert(MGA, min_change);
    map.insert(MRU, min_change);
    map
});

#[cfg(test)]
mod currency_tests {
    use crate::currencies::{ISO4217Alphabetic, WrongISOReason};

    #[test]
    fn currency_code_validation() {
        fn test(iso: &str, reason: WrongISOReason) {
            ISO4217Alphabetic::try_from(iso.to_owned()).is_err_and(|f| f.reason == reason);
        }
        test("PL!", WrongISOReason::NonAlphabetic);
        test("PL1", WrongISOReason::NonAlphabetic);
        test("pln s", WrongISOReason::Length(5));
        ISO4217Alphabetic::try_from("  pln ".to_owned()).unwrap();
    }
}
