use rust_decimal::Decimal;

use crate::currencies::CurrencyStd;

pub trait MonetaryAmount<C, AM>
where
    C: CurrencyStd,
    AM: Into<Decimal>,
{
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct Positive {
    dec: Decimal,
}

impl From<Positive> for Decimal {
    fn from(value: Positive) -> Self {
        value.dec
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct NotPositive {
    dec: Decimal,
}

impl TryFrom<Decimal> for Positive {
    type Error = NotPositive;

    fn try_from(dec: Decimal) -> Result<Self, Self::Error> {
        if dec > Decimal::ZERO {
            Ok(Positive { dec })
        } else {
            Err(NotPositive { dec })
        }
    }
}

pub trait PositiveAmt<C>: MonetaryAmount<C, Positive>
where
    C: CurrencyStd,
{
}

#[cfg(test)]
mod money_tests {
    use rust_decimal::Decimal;

    use crate::money::Positive;

    #[test]
    fn positive_num() {
        let pos = Decimal::new(100, 1);
        let res = Positive::try_from(pos);
        assert!(res.is_ok());
        let neg = Decimal::new(-100, 1);
        assert!(Positive::try_from(neg).is_err_and(|f| f.dec == neg));
        assert!(Positive::try_from(Decimal::ZERO).is_err_and(|f| f.dec == Decimal::ZERO));
    }
}
