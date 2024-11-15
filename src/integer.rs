use num_traits::cast::NumCast;
use num_traits::Num;

const HEX_CHARS: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];
const SIZE: usize = 8;

pub struct Integer {
    bits: [u8; SIZE],
    is_signed: bool,
}

impl Integer {
    pub fn new<T: Num + NumCast + Copy>(s: &str, r: u32, is_signed: bool) -> Self {
        let mut bits = [0; SIZE];
        let n = T::from_str_radix(s, r).unwrap_or(T::zero());
        let l: i64 = num_traits::cast(n).unwrap();

        if n == T::zero() && s != "0" {
            let sn = if is_signed { "signed " } else { "unsigned" };
            panic!("Invalid input for {} base-{} number", sn, r);
        }

        let sz_err_uns = !is_signed && l >= 2_i64.pow(SIZE as u32);
        let sz_err_sig = is_signed && r != 2 && l.abs() >= 2_i64.pow(SIZE as u32 - 1);
        let sz_err_bin = r == 2 && s.len() > SIZE;

        if sz_err_uns || sz_err_sig || sz_err_bin {
            panic!("Number too large for {} bits", SIZE);
        }

        (0..SIZE).for_each(|i| bits[(SIZE - 1) - i] = ((l >> i) & 1) as u8);

        Self { bits, is_signed }
    }

    pub fn to_binary(&self) -> String {
        self.bits.iter().map(|b| b.to_string()).collect()
    }

    pub fn to_decimal(&self) -> isize {
        if !self.is_signed || self.bits[0] == 0 {
            return Self::uns_decimal(self.bits) as isize;
        }

        let pos_bits = self.twos_complement();
        let dec = Self::uns_decimal(pos_bits) as isize;

        -(dec)
    }

    pub fn to_hex(&self) -> String {
        if !self.is_signed || self.bits[0] == 0 {
            return Self::uns_hex(self.bits);
        }

        let pos_bits = self.twos_complement();
        let hex = Self::uns_hex(pos_bits);

        "-".to_string() + &hex
    }

    fn twos_complement(&self) -> [u8; SIZE] {
        let mut result = [0; SIZE];
        let mut carry = 1;

        self.bits
            .iter()
            .enumerate()
            .for_each(|(i, &x)| result[i] = x ^ 1);

        result.iter_mut().rev().for_each(|bit| {
            let sum = *bit + carry;
            *bit = sum % 2;
            carry = sum / 2;
        });

        result
    }

    fn uns_decimal(bits: [u8; SIZE]) -> usize {
        let mut n: usize = 0;
        bits.iter().for_each(|&x| n = n * 2 + x as usize);

        n
    }

    fn uns_hex(bits: [u8; SIZE]) -> String {
        bits.chunks_exact(4)
            .map(|chunk| {
                let hex = chunk.iter().fold(0, |acc, &x| acc * 2 + x);
                HEX_CHARS[hex as usize].to_string()
            })
            .collect()
    }
}
