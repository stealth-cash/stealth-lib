use std::str::FromStr;

pub struct Hasher {
    p: u128,
    n_rounds: u8,
    c: Vec<u128>
}

impl Default for Hasher {
    fn default() -> Self {
        Hasher {
            p: u128::from_str("340282366920938463463374607431768211455").expect("Failed conversion"),
            n_rounds: 10,
            c: vec![
                0,
                u128::from_str("25823191961023811529686723375255045").expect("Failed conversion"),
                u128::from_str("48376936063113800887806988124358800").expect("Failed conversion"),
                u128::from_str("75580405153655082660116863095114839").expect("Failed conversion"),
                u128::from_str("66651710483985382365580181188706173").expect("Failed conversion"),
                u128::from_str("45887003413921204775397977044284378").expect("Failed conversion"),
                u128::from_str("14399999722617037892747232478295923").expect("Failed conversion"),
                u128::from_str("29376176727758177809204424209125257").expect("Failed conversion"),
                u128::from_str("13768859312518298840937540532277016").expect("Failed conversion"),
                u128::from_str("54749662990362840569021981534456448").expect("Failed conversion"),
                u128::from_str("25161436470718351277017231215227846").expect("Failed conversion"),
                u128::from_str("90370030464179443930112165274275271").expect("Failed conversion"),
                u128::from_str("92014788260850167582827910417652439").expect("Failed conversion"),
                u128::from_str("40376490640073034398204558905403523").expect("Failed conversion"),
                u128::from_str("90379224439153137712327643289289624").expect("Failed conversion"),
                u128::from_str("11220341520269979188892857030918685").expect("Failed conversion"),
                u128::from_str("11480168113674888067906254878279274").expect("Failed conversion"),
                u128::from_str("11144081894867681653997893051446803").expect("Failed conversion"),
                u128::from_str("64965960071752809090438003157362764").expect("Failed conversion"),
                u128::from_str("98428510787134995495896453413714864").expect("Failed conversion")
            ]    
        }
    }
}

impl Hasher {
    fn mimc_feistel(il: u128, ir: u128, k: u128) -> (u128, u128) {
        let hasher = Hasher::default();
        let mut last_l = il.clone();
        let mut last_r = ir.clone();

        for i in 0..hasher.n_rounds {
            let mask = last_r.wrapping_add(k).checked_rem(hasher.p).unwrap();
            let mask = mask.wrapping_add(hasher.c[i as usize]).checked_rem(hasher.p).unwrap();
            let mask2 = mask.wrapping_mul(mask).checked_rem(hasher.p).unwrap();
            let mask4 = mask2.wrapping_mul(mask2).checked_rem(hasher.p).unwrap();
            let mask = mask4.wrapping_mul(mask).checked_rem(hasher.p).unwrap();
    
            let temp = last_r;
            last_r = last_l.wrapping_add(mask).checked_rem(hasher.p).unwrap();
            last_l = temp;
        }
    
        (last_l, last_r)
    }

    pub fn mimc_sponge(left: u128, right: u128, k: u128) -> u128 {
        let mut last_r = left.clone();
        let mut last_l = right.clone();
    
        for _ in 0..Hasher::default().n_rounds {
            let (new_last_r, new_last_l) = Hasher::mimc_feistel(last_r, last_l, k);
    
            last_r = new_last_r.wrapping_add(1).checked_rem(Hasher::default().p).unwrap();
            last_l = new_last_l.clone();
        }
    
        last_r
    }
}