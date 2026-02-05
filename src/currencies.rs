use std::hash::Hash;

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
