use rand::Rng;

pub struct KeyHelper;

impl KeyHelper {
    pub fn validate_b(a: i64, b: i64) -> bool {
        b > a && ((b == a + (26-a)/2) || (b == a - (26-a)/2))
    }

    pub fn generate_b(a: i64) -> i64 {
        for num in 450_000_000..900_000_000 {
            if Self::validate_b(a, num) {
                return num as i64;
            }
        }
        0
    }

    pub fn generate_key() -> (i64, i64) {
        let mut rng = rand::thread_rng();
        let mut a = 0;
        let mut b = Self::generate_b(a);
        let mut aa = 0;
        let mut bb = 0;
        
        loop {
            if !Self::validate_a(a) {
                a = rng.gen_range(150_000_000..450_000_000);
            }
                aa = a + rng.gen_range(150_000_000..350_000_000) * 26;
                bb = b + rng.gen_range(450_000_000..800_000_000) * 26;
                if Self::validate_b(a, b) && Self::validate_a(a) {
                    break;
                }
            }
        (a, b)
    }

    pub fn find_c(a: i64) -> i64 {
        (1..26).find(|&c| (a * c) % 26 == 1).expect("Aucune valeur de C trouvee pour valider A")
    }

    pub fn validate_a(a: i64,) -> bool {
        (1..26).any(|c| (c * a) % 26 == 1)
    }

    pub fn validate_cc(cc: i64, aa: i64) -> bool {
        (cc * aa) % 26 == 1
    }

    pub fn generate_public_key(key: (i64, i64), c: i64) -> (i64, i64) {
        let mut rng = rand::thread_rng();
        
        let cc = loop {
            let candidate = c + rng.gen_range(450_005_000..900_000_000);
            if Self::validate_cc(candidate, key.0) {
                break candidate;
            }
        };

        (cc, key.1)
    }
}