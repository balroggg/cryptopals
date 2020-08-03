extern crate base64;
extern crate hex;

/// Convert hex to base64
///
/// https://cryptopals.com/sets/1/challenges/1
pub fn hex_to_base64(hex: String) -> String {
    let bin: Vec<u8> = hex::decode(hex).unwrap();
    base64::encode(bin)
}

/// Fixed XOR
///
/// https://cryptopals.com/sets/1/challenges/2
pub fn xor(hex1: String, hex2: String) -> String {
    assert_eq!(hex1.len(), hex2.len());

    let bin1 = hex::decode(hex1).unwrap();
    let bin2 = hex::decode(hex2).unwrap();
    let mut xor = Vec::new();
    for i in 0..(bin1.len()) {
        xor.push(&bin1[i] ^ &bin2[i])
    }
    return hex::encode(xor);
}
