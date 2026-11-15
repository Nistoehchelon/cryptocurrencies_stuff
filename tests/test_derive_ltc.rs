// cargo test -- --nocapture to show println!()

use cryptocurrencies_stuff::ltc_get_address::get_address_from_mnemonic_and_index;

#[test]
pub fn test_ltc() {
    let mnemonic =
        "stairs grace visit maple author pattern pumpkin erosion chimney learn quick cabin";

    let max = 10;
    let mut index = 0;
    for li in (0..=3).skip(1) {
        println!("loop {li}");
        loop {
            let Ok(addr) = get_address_from_mnemonic_and_index(mnemonic, index) else {
                assert_eq!(1, 2);
                break;
            };
            println!("{index} {addr}");
            index += 1;
            if index > max {
                break;
            }
        }
        index = 0;
    }
}
