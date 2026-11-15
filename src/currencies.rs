use crate::atomic_unit::{AtomicUnit, CryptocurrencyConfig};

pub type CryptoAtomicUnitRaw = u128;
#[derive(Clone, Debug)]
pub struct CryptoAtomicUnit(pub CryptoAtomicUnitRaw);

impl CryptoAtomicUnit {
    pub fn as_raw(&self) -> CryptoAtomicUnitRaw {
        self.0
    }
    pub fn from_u128(n: u128) -> Self {
        Self(n)
    }
    pub fn from_u64(n: u64) ->  Self {
        Self(n.into())
    }
}
#[derive(Clone, Debug)]
pub struct XMR;
impl CryptocurrencyConfig for XMR {
    const ATOMIC_PER_UNIT: CryptoAtomicUnit = CryptoAtomicUnit(1_000_000_000_000);
    const SYMBOL: &'static str = "XMR";
}

pub type XMRAtomicUnit = AtomicUnit<XMR>;

#[derive(Clone, Debug)]
pub struct LTC;
impl CryptocurrencyConfig for LTC {
    const ATOMIC_PER_UNIT: CryptoAtomicUnit = CryptoAtomicUnit(100_000_000);
    const SYMBOL: &'static str = "LTC";
}
pub type LTCAtomicUnit = AtomicUnit<LTC>;

#[derive(Clone, Debug)]
pub struct XNO;
impl CryptocurrencyConfig for XNO {
    const ATOMIC_PER_UNIT: CryptoAtomicUnit = CryptoAtomicUnit(10u128.pow(30));
    const SYMBOL: &'static str = "XNO";
}

pub type XNOAtomicUnit = AtomicUnit<XNO>;
