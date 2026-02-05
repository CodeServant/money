use std::{collections::HashMap, hash::Hash, sync::LazyLock};

use rust_decimal::Decimal;

/// Currency standard.
pub trait CurrencyStd: Hash + Eq {}

type MinorUnitsType = u32;

/// Currency standard that have known decimal minor units like Crypto or ISO Currencies.
pub trait CurrencyWIthMinorStd: CurrencyStd {
    fn minor_units() -> MinorUnitsType;
}

type ISONumericType = u32;

pub trait HasISO4217Code<'a> {
    fn alphabetic_iso() -> &'a ISO4217Alphabetic;
    fn numeric_iso() -> ISONumericType;
}

/// Code string like USD, EUR, PLN etc.
pub struct ISO4217Alphabetic {
    code: String,
}

pub struct ISO4217Currency {
    iso_alpha: ISO4217Alphabetic,
    iso_numeric: ISONumericType,
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
