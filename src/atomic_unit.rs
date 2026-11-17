use ruint::aliases::U256;

use crate::currencies::CurrencyAtomicUnit;

pub trait CurrencyConfig {
    const ATOMIC_UNITS_PER_ONE_WHOLE: CurrencyAtomicUnit;
    const SYMBOL: &'static str;
    fn precision() -> usize {
        Self::ATOMIC_UNITS_PER_ONE_WHOLE.0.to_string().len() - 1
    }
}

#[derive(Clone, Debug)]
pub struct AtomicUnit<C: CurrencyConfig>(CurrencyAtomicUnit, std::marker::PhantomData<C>);

impl<C: CurrencyConfig> AtomicUnit<C> {
    pub fn from_atomic(a: CurrencyAtomicUnit) -> Self {
        Self(a, std::marker::PhantomData)
    }

    pub fn as_atomic(&self) -> CurrencyAtomicUnit {
        self.0.clone()
    }

    pub fn to_string_trimmed(&self) -> String {
        let whole = self.0.0 / C::ATOMIC_UNITS_PER_ONE_WHOLE.0;
        let fractional = self.0.0 % C::ATOMIC_UNITS_PER_ONE_WHOLE.0;

        if fractional == U256::ZERO {
            return whole.to_string();
        }

        let frac_str = format!("{:0width$}", fractional, width = C::precision())
            .trim_end_matches('0')
            .to_string();

        format!("{}.{}", whole, frac_str)
    }
}

impl<C: CurrencyConfig> PartialEq for AtomicUnit<C> {
    fn eq(&self, other: &Self) -> bool {
        self.0.0 == other.0.0
    }
}

impl<C: CurrencyConfig> Eq for AtomicUnit<C> {}

impl<C: CurrencyConfig> PartialOrd for AtomicUnit<C> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<C: CurrencyConfig> Ord for AtomicUnit<C> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.0.cmp(&other.0.0)
    }
}
#[derive(Debug, PartialEq)]
pub enum ArithmeticError {
    Overflow,
}

impl<C: CurrencyConfig> AtomicUnit<C> {
    pub fn checked_sub_result(self, rhs: Self) -> anyhow::Result<Self, ArithmeticError> {
        self.0
            .0
            .checked_sub(rhs.0.0)
            .map(|arg0: U256| AtomicUnit::from_atomic(CurrencyAtomicUnit(arg0)))
            .ok_or(ArithmeticError::Overflow)
    }
    pub fn checked_mul_result(self, rhs: Self) -> anyhow::Result<Self, ArithmeticError> {
        self.0
            .0
            .checked_mul(rhs.0.0)
            .map(|arg0: U256| AtomicUnit::from_atomic(CurrencyAtomicUnit(arg0)))
            .ok_or(ArithmeticError::Overflow)
    }
}
