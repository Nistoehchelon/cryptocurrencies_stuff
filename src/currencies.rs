use crate::atomic_unit::{AtomicUnit, CryptocurrencyConfig};

pub type CryptoAtomicUnitRaw = u128;
#[derive(Clone, Debug)]
pub struct CryptoAtomicUnit(pub CryptoAtomicUnitRaw);

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
