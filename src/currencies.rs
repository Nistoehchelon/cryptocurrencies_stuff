use ruint::aliases::U256;

use crate::atomic_unit::{AtomicUnit, CurrencyConfig};

pub type CurrencyAtomicUnitRaw = U256;
#[derive(Clone, Debug, derive_more::Display, serde::Serialize, serde::Deserialize)]
pub struct CurrencyAtomicUnit(pub CurrencyAtomicUnitRaw);

impl CurrencyAtomicUnit {
    pub fn as_raw(&self) -> CurrencyAtomicUnitRaw {
        self.0
    }
    pub fn from_u128(n: u128) -> Self {
        Self(U256::from(n))
    }
    pub fn from_u64(n: u64) -> Self {
        Self(U256::from(n))
    }
}
#[derive(Clone, Debug)]
pub struct XMR;
impl CurrencyConfig for XMR {
    const ATOMIC_UNITS_PER_ONE_WHOLE: CurrencyAtomicUnit =         CurrencyAtomicUnit(ruint_macro::uint!(1_000_000_000_000_U256));

    const SYMBOL: &'static str = "XMR";
}

pub type XMRAtomicUnit = AtomicUnit<XMR>;

#[derive(Clone, Debug)]
pub struct LTC;
impl CurrencyConfig for LTC {
    const ATOMIC_UNITS_PER_ONE_WHOLE: CurrencyAtomicUnit =  CurrencyAtomicUnit(ruint_macro::uint!(100_000_000_U256));
    const SYMBOL: &'static str = "LTC";
}
pub type LTCAtomicUnit = AtomicUnit<LTC>;

#[derive(Clone, Debug)]
pub struct XNO;
impl CurrencyConfig for XNO {
    const ATOMIC_UNITS_PER_ONE_WHOLE: CurrencyAtomicUnit =  CurrencyAtomicUnit(ruint_macro::uint!(1_000_000_000_000_000_000_000_000_000_000_U256));
    const SYMBOL: &'static str = "XNO";
}

pub type XNOAtomicUnit = AtomicUnit<XNO>;
