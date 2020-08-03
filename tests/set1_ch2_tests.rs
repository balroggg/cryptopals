#[test]
fn test_xor() {
    let src1 = "1c0111001f010100061a024b53535009181c";
    let src2 = "686974207468652062756c6c277320657965";
    let exp = "746865206b696420646f6e277420706c6179";
    assert_eq!(cryptopals::xor(src1.to_string(), src2.to_string()),
               exp.to_string());
}
