pub trait CryptocurrencyConfig {
    const ATOMIC_PER_UNIT: u64;
    const SYMBOL: &'static str;
    fn precision() -> usize {
        Self::ATOMIC_PER_UNIT.ilog10() as usize
    }
}

#[derive(Clone, Debug)]
pub struct AtomicUnit<C: CryptocurrencyConfig>(u64, std::marker::PhantomData<C>);

impl<C: CryptocurrencyConfig> AtomicUnit<C> {
    pub fn from_atomic(a: u64) -> Self {
        Self(a, std::marker::PhantomData)
    }

    pub fn as_atomic(&self) -> u64 {
        self.0
    }

    pub fn to_string_trimmed(&self) -> String {
        let whole = self.0 / C::ATOMIC_PER_UNIT;
        let fractional = self.0 % C::ATOMIC_PER_UNIT;

        if fractional == 0 {
            return whole.to_string();
        }

        let frac_str = format!("{:0width$}", fractional, width = C::precision())
            .trim_end_matches('0')
            .to_string();

        format!("{}.{}", whole, frac_str)
    }
}

impl<C: CryptocurrencyConfig> PartialEq for AtomicUnit<C> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<C: CryptocurrencyConfig> Eq for AtomicUnit<C> {}

impl<C: CryptocurrencyConfig> PartialOrd for AtomicUnit<C> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<C: CryptocurrencyConfig> Ord for AtomicUnit<C> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
#[derive(Debug, PartialEq)]
pub enum ArithmeticError {
    Overflow,
}

impl<C: CryptocurrencyConfig> AtomicUnit<C> {
    pub fn checked_sub_result(self, rhs: Self) -> anyhow::Result<Self, ArithmeticError> {
        self.0
            .checked_sub(rhs.0)
            .map(AtomicUnit::from_atomic)
            .ok_or(ArithmeticError::Overflow)
    }
    pub fn checked_mul_result(self, rhs: Self) -> anyhow::Result<Self, ArithmeticError> {
        self.0
            .checked_mul(rhs.0)
            .map(AtomicUnit::from_atomic)
            .ok_or(ArithmeticError::Overflow)
    }
}
