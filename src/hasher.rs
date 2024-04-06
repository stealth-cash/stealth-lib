use crate::uint256::Uint256;

pub struct Hasher {
    p: Uint256,
    n_rounds: u8,
    c: Vec<Uint256>
}

impl Default for Hasher {
    fn default() -> Self {
        Hasher {
            p: Uint256::from("218882428718392752222").expect("Failed to parse"),
            n_rounds: 10,
            c: vec![
                Uint256::from("0").expect("Failed to parse"),
                Uint256::from("2582319196106723375255").expect("Failed to parse"),
                Uint256::from("7115325576885801937521").expect("Failed to parse"),
                Uint256::from("5125317692288365653129").expect("Failed to parse"),
                Uint256::from("6665171048390181188706").expect("Failed to parse"),
                Uint256::from("4588700341397977044284").expect("Failed to parse"),
                Uint256::from("1439999972267232478295").expect("Failed to parse"),
                Uint256::from("2937617672774424209125").expect("Failed to parse"),
                Uint256::from("1376885931257540532277").expect("Failed to parse"),
                Uint256::from("5474966299031981534456").expect("Failed to parse"),
                Uint256::from("2516143647077231215227").expect("Failed to parse"),
                Uint256::from("9037003046412165274275").expect("Failed to parse"),
                Uint256::from("9201478826087910417652").expect("Failed to parse"),
                Uint256::from("4037649064004558905403").expect("Failed to parse"),
                Uint256::from("9037922443917643289289").expect("Failed to parse"),
                Uint256::from("1122034152022857030918").expect("Failed to parse"),
                Uint256::from("1148016811366254878279").expect("Failed to parse"),
                Uint256::from("1114408189487893051446").expect("Failed to parse"),
                Uint256::from("6496596007178003157362").expect("Failed to parse"),
                Uint256::from("9842851078716453413714").expect("Failed to parse")
            ]    
        }
    }
}

impl Hasher {
    fn mimc_feistel(il: &Uint256, ir: &Uint256, k: &Uint256) -> (Uint256, Uint256) {
        let hasher = Hasher::default();
        let mut last_l = il.clone();
        let mut last_r = ir.clone();

        for i in 0..hasher.n_rounds {
            let mask = last_r.add_mod(&k, &hasher.p);
            let mask = mask.add_mod(&hasher.c[i as usize], &hasher.p);
            let mask2 = mask.mul_mod(&mask, &hasher.p);
            let mask4 = mask2.mul_mod(&mask2, &hasher.p);
            let mask = mask4.mul_mod(&mask, &hasher.p);
    
            let temp = last_r;
            last_r = last_l.add_mod(&mask, &hasher.p);
            last_l = temp;
        }
    
        (last_l, last_r)
    }

    pub fn mimc_sponge(left: &Uint256, right: &Uint256, k: &Uint256) -> Uint256 {
        let mut last_r = left.clone();
        let mut last_l = right.clone();
    
        for _ in 0..Hasher::default().n_rounds {
            let (new_last_r, new_last_l) = Hasher::mimc_feistel(&last_r, &last_l, &k);
    
            last_r = new_last_r.add_mod(&Uint256::new(1), &Hasher::default().p);
            last_l = new_last_l.clone();
        }
    
        last_r
    }
}