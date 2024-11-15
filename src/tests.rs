#[cfg(test)]
use crate::integer::Integer;

#[test]
fn bin_to_dec() {
    let number = Integer::new::<usize>("0000000000001101", 2, false);
    assert_eq!(number.to_decimal(), 13);
}

#[test]
fn dec_to_bin() {
    let number = Integer::new::<usize>("13", 10, false);
    assert_eq!(number.to_binary(), "0000000000001101");
}

#[test]
fn bin_to_hex() {
    let number = Integer::new::<usize>("1111000110001101", 2, false);
    assert_eq!(number.to_hex(), "F18D");
}

#[test]
fn hex_to_bin() {
    let number = Integer::new::<usize>("F18D", 16, false);
    assert_eq!(number.to_binary(), "1111000110001101");
}

#[test]
fn dec_to_bin_signed() {
    let number = Integer::new::<isize>("-15", 10, true);
    assert_eq!(number.to_binary(), "1111111111110001");
}

#[test]
fn bin_to_dec_signed() {
    let number = Integer::new::<isize>("1111111111110001", 2, true);
    assert_eq!(number.to_decimal(), -15);
}
