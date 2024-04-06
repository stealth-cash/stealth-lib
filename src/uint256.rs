// use std::str::FromStr;
// use borsh::{BorshDeserialize, BorshSerialize};
// use primitive_types::U256;
// use hex;
// use crate::utils::{self, err, SolanaError};

// #[derive(Debug, Clone, Copy, Eq, Hash, PartialOrd)]
// pub struct Uint256 {
//     pub v: U256
// }

// impl BorshSerialize for Uint256 {
//     fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
//         let mut buf = [0u8; 32];
//         self.v.to_big_endian(&mut buf);
//         writer.write_all(&buf)?;
//         Ok(())
//     }
// }

// impl BorshDeserialize for Uint256 {
//     fn deserialize(buf: &mut &[u8]) -> Result<Self, std::io::Error> {
//         let v = U256::from_little_endian(buf);
//         Ok(Self { v })
//     }
    
//     fn deserialize_reader<R: std::io::prelude::Read>(reader: &mut R) -> std::io::Result<Self> {
//         let mut buf = [0u8; 32];
//         reader.read_exact(&mut buf)?;
//         let v = U256::from_little_endian(&buf);
//         Ok(Self { v })
//     }
// }

// impl FromStr for Uint256 {
//     type Err = SolanaError;
//     fn from_str(s: &str) -> Result<Self, SolanaError> {
//         match U256::from_str_radix(s, 16) {
//             Ok(n) => return Ok(Self { v: n }),
//             Err(_) => return Err(err("Failed to parse").into())
//         }
//     }
// }

// impl ToString for Uint256 {
//     fn to_string(&self) -> String {
//         let mut bytes = [0; 32];
//         self.v.to_big_endian(&mut bytes);
//         hex::encode(bytes)
//     }
// }

// impl PartialEq for Uint256 {
//     fn eq(&self, other: &Self) -> bool {
//         self.v == other.v
//     }
    
//     fn ne(&self, other: &Self) -> bool {
//         !self.eq(other)
//     }
// }

// impl Uint256 {
//     pub fn new(n: u128) -> Self {
//         Self { v: U256::from(n) }
//     }

//     pub fn from(s: &'static str) -> Result<Self, SolanaError> {
//         match U256::from_str_radix(s, 16) {
//             Ok(n) => return Ok(Self { v: n }),
//             Err(_) => return Err(err("Failed to parse").into())
//         }
//     }

//     pub fn from_string(s: &String) -> Self {
//         Self { v: U256::from(s.as_bytes()) }
//     }

//     pub fn from_dec_str(s: &str) -> Result<Self, SolanaError> {
//         match U256::from_dec_str(s) {
//             Ok(n) => return Ok(Self { v: n }),
//             Err(_) => return Err(err("Failed to parse").into())
//         }
//     }

//     pub fn from_bytes(&self, bytes: &[u8]) -> Self {
//         assert!(bytes.len() <= 32, "big-endian");
//         return Self { v: U256::from_big_endian(bytes) }
//     }

//     pub fn to_bytes(&self, r: &mut [u8]) {
//         self.v.to_big_endian(r);
//     }

//     pub fn zero() -> Self {
//         Self::from_str("0x0").unwrap()
//     }

//     pub fn one() -> Self {
//         Self::from_str("0x1").unwrap()
//     }

//     pub fn add_mod(&self, b: &Uint256, p: &Uint256) -> Uint256 {
//         /* (a + b) mod p = [(a mod p) + (b mod p)] mod p */
        
//         // a mod p
//         let x1 = self.v.checked_rem(p.v).expect("modulo");
        
//         // b mod p
//         let x2 = b.v.checked_rem(p.v).expect("modulo");        
        
//         let (mut x3, overflow) = x1.overflowing_add(x2);

//         if overflow {
//             x3 = x3
//                 .checked_add(
//                     U256::MAX.checked_sub(p.v).expect("sub")
//                         .checked_add(U256::from_big_endian(&[1])).expect("conversion")   
//                 ).expect("conversion")
//         }

//         x3 = x3.checked_rem(p.v).expect("modulo");

//         return Uint256 { v: x3 };
//     }

//     pub fn sub_mod(&self, b: &Uint256, p: &Uint256) -> Uint256 {
//         /* 
//             (a - b) mod p 
//             => [(a mod p) - (b mod p)] mod p 
//             => [a mod p + (p - b) mod p] mod p 
//         */
//         let x1 = self.v.checked_rem(p.v).expect("modulo");
//         let x2 = b.v.checked_rem(p.v).expect("modulo");

//         return Uint256 { v: x1 }.add_mod(&Uint256 { v: p.v - x2 }, p);
//     }

//     pub fn mul_mod(&self, b: &Uint256, p: &Uint256) -> Uint256 {
//         /*
//             add-and-double / square-and-multiply
//             9 * 2 = 2 + 2 ... + 2;
            
//             9 = b'1001
            
//             iterate through b'1001, base = 0
//                 if n = 1:
//                     base *= x
//                 else:
//                     base *= x
//                     base += x 

//             base = 0
//             base = 2
//             base = 4
//             base = 8
//             base = 18
            
//         */
//         let x1 = Self { v: self.v.checked_rem(p.v).expect("modulo") } ;
//         let x2 = Self { v: b.v.checked_rem(p.v).expect("modulo") };
        
//         let mut base = Self::zero();
        
//         let seq: Self;
//         let adder: Self;

//         if x1.v < x2.v {
//             seq = x1.clone();
//             adder = x2.clone();
//         } else {
//             seq = x2.clone();
//             adder = x1.clone();
//         }

//         let mut seq_bytes = [0; 32];
//         seq.to_bytes(&mut seq_bytes);
//         let mut seq_binaries: Vec<u8> = vec![];
        
//         utils::bytes_to_binary(&seq_bytes, &mut seq_binaries);

//         let mut on = false;
//         for d in seq_binaries.into_iter() {
//             if on {
//                 base = base.add_mod(&base, p);
//             }
//             if d > 0 {
//                 on = true;
//                 base = base.add_mod(&adder, p);
//             }
//         }

//         return base;
//     }

//     pub fn exp_mod(&self, e: &Uint256, p: &Uint256) -> Uint256 {
//         let seq = e.clone();
//         let multiplier = Self { v: self.v.checked_rem(p.v).expect("modulo") };

//         let mut base = Self::one();

//         let mut seq_bytes = [0; 32];
//         seq.to_bytes(&mut seq_bytes);

//         let mut seq_binaries: Vec<u8> = vec![];
//         utils::bytes_to_binary(&seq_bytes, &mut seq_binaries);

//         let mut on = false;
//         for d in seq_binaries.into_iter() {
//             if on {
//                 base = base.mul_mod(&base, p);
//             }
//             if d > 0 {
//                 on = true;
//                 base = base.mul_mod(&multiplier, p);
//             }
//         }

//         return base;
//     }

//     pub fn div_mod(&self, b: &Uint256, p: &Uint256) -> Uint256 {
//         return self.mul_mod(&b.exp_mod(&Self{ v: p.v - 2 }, p), p);
//     }

//     pub fn is_zero(&self) -> bool {
//         self.v.is_zero()
//     }


// }
// #[cfg(test)]
// mod tests {
//     use std::str::FromStr;
//     use crate::uint256::Uint256;

//     #[test]
//     fn uin256_addition_case_1() {
//         let a = Uint256::from_str("0xBD").unwrap();
//         let b = Uint256::from_str("0x2B").unwrap();
//         let p = Uint256::from_str("0xB").unwrap();

//         let r = a.add_mod(&b, &p);

//         assert_eq!(r.to_string(), "0000000000000000000000000000000000000000000000000000000000000001");
//     }

//     #[test]
//     fn uin256_addition_case_2() {
//         let a = Uint256::from_str("0xa167f055ff75c").unwrap();
//         let b = Uint256::from_str("0xacc457752e4ed").unwrap();
//         let p = Uint256::from_str("0xf9cd").unwrap();

//         let r = a.add_mod(&b, &p);

//         assert_eq!(r.to_string(), "0000000000000000000000000000000000000000000000000000000000006bb0");
//     }

//     #[test]
//     fn uin256_addition_case_3() {
//         let a = Uint256::from_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2E").unwrap();
//         let b = Uint256::from_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2E").unwrap();
//         let p = Uint256::from_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F").unwrap();

//         let r = a.add_mod(&b, &p);

//         assert_eq!(r.to_string(), "fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2d");
//     }

//     #[test]
//     fn uin256_subtraction_case_1() {
//         let a = Uint256::from_str("0x1ce606").unwrap();     // a = 189389.unwrap();
//         let b = Uint256::from_str("0xacc12484").unwrap();       // b = 289833894.unwrap();
//         let p = Uint256::from_str("0xf3fa3").unwrap();      // p = 99933.unwrap();

//         let r = a.sub_mod(&b, &p);

//         assert_eq!(r.to_string(), "000000000000000000000000000000000000000000000000000000000009645b");
//     }

//     #[test]
//     fn uin256_subtraction_case_2() {
//         let a = Uint256::from_str("0xacc12484").unwrap();       // a = 289833894.unwrap();
//         let b = Uint256::from_str("0x1ce606").unwrap();     // b = 189389.unwrap();
//         let p = Uint256::from_str("0xf3fa3").unwrap();      // p = 99933.unwrap();

//         let r = a.sub_mod(&b, &p);

//         assert_eq!(r.to_string(), "000000000000000000000000000000000000000000000000000000000005db48");
//     }

//     #[test]
//     fn uin256_multiplication_case() {
//         let a = Uint256::from_str("0xa167f055ff75c").unwrap();       // a = 283948457393954.unwrap();
//         let b = Uint256::from_str("0xacc457752e4ed").unwrap();     // b = 303934849383754.unwrap();
//         let p = Uint256::from_str("0xf9cd").unwrap();      // p = 6394.unwrap();

//         let r = a.mul_mod(&b, &p);

//         assert_eq!(r.to_string(), "000000000000000000000000000000000000000000000000000000000000e116");
//     }

//     #[test]
//     fn uin256_exponentiation_case() {
//         let a = Uint256::from_str("0x1ce606").unwrap();       // a = 189389.unwrap();
//         let b = Uint256::from_str("0xacc12484").unwrap();     // b = 289833894.unwrap();
//         let p = Uint256::from_str("0xf3fa3").unwrap();      // p = 99933.unwrap();

//         let r = a.exp_mod(&b, &p);

//         assert_eq!(r.to_string(), "000000000000000000000000000000000000000000000000000000000002a0fd");
//     }

//     #[test]
//     fn uin256_division_case() {
//         let a = Uint256::from_str("0x1ce606").unwrap();       // a = 189389.unwrap();
//         let b = Uint256::from_str("0xacc12484").unwrap();     // b = 289833894.unwrap();
//         let p = Uint256::from_str("0xf3fa3").unwrap();      // p = 99933.unwrap();

//         let r = a.div_mod(&b, &p);

//         assert_eq!(r.to_string(), "0000000000000000000000000000000000000000000000000000000000061f57");
//     }
// }