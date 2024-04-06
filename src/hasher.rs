use std::str::FromStr;

use cosmwasm_std::Uint256;

pub struct Hasher {
    p: Uint256,
    n_rounds: u8,
    c: Vec<Uint256>
}

impl Default for Hasher {
    fn default() -> Self {
        Hasher {
            p: Uint256::from_str("21888242871839275222246405745257275088548364400416034343698204186575808495617").expect("Failed conversion"),
            n_rounds: 10,
            c: vec![
                Uint256::zero(),
                Uint256::from_str("25823191961023811529686723375255045606187170120624741056268890390838310270028").expect("Failed conversion"),
                Uint256::from_str("71153255768872006974285801937521995907343848376936063113800887806988124358800").expect("Failed conversion"),
                Uint256::from_str("51253176922899201987938365653129780755804051536550826601168630951148399005246").expect("Failed conversion"),
                Uint256::from_str("66651710483985382365580181188706173532487386392003341306307921015066514594406").expect("Failed conversion"),
                Uint256::from_str("45887003413921204775397977044284378920236104620216194900669591190628189327887").expect("Failed conversion"),
                Uint256::from_str("14399999722617037892747232478295923748665564430258345135947757381904956977453").expect("Failed conversion"),
                Uint256::from_str("29376176727758177809204424209125257629638239807319618360680345079470240949145").expect("Failed conversion"),
                Uint256::from_str("13768859312518298840937540532277016512087005174650120937309279832230513110846").expect("Failed conversion"),
                Uint256::from_str("54749662990362840569021981534456448557155682756506853240029023635346061661615").expect("Failed conversion"),
                Uint256::from_str("25161436470718351277017231215227846535148280460947816286575563945185127975034").expect("Failed conversion"),
                Uint256::from_str("90370030464179443930112165274275271350651484239155016554738639197417116558730").expect("Failed conversion"),
                Uint256::from_str("92014788260850167582827910417652439562305280453223492851660096740204889381255").expect("Failed conversion"),
                Uint256::from_str("40376490640073034398204558905403523738912091909516510156577526370637723469243").expect("Failed conversion"),
                Uint256::from_str("903792244391531377123276432892896247924738784402045372115602887103675299839").expect("Failed conversion"),
                Uint256::from_str("112203415202699791888928570309186854585561656615192232544262649073999791317171").expect("Failed conversion"),
                Uint256::from_str("114801681136748880679062548782792743842998635558909635247841799223004802934045").expect("Failed conversion"),
                Uint256::from_str("111440818948676816539978930514468038603327388809824089593328295503672011604028").expect("Failed conversion"),
                Uint256::from_str("64965960071752809090438003157362764845283225351402746675238539375404528707397").expect("Failed conversion"),
                Uint256::from_str("98428510787134995495896453413714864789970336245473413374424598985988309743097").expect("Failed conversion")
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
            let mask = last_r.wrapping_add(*k).checked_rem(hasher.p).unwrap();
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

    pub fn mimc_sponge(left: &Uint256, right: &Uint256, k: &Uint256) -> Uint256 {
        let mut last_r = left.clone();
        let mut last_l = right.clone();
    
        for _ in 0..Hasher::default().n_rounds {
            let (new_last_r, new_last_l) = Hasher::mimc_feistel(&last_r, &last_l, &k);
    
            last_r = new_last_r.wrapping_add(Uint256::one()).checked_rem(Hasher::default().p).unwrap();
            last_l = new_last_l.clone();
        }
    
        last_r
    }
}