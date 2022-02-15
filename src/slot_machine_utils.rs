pub mod random_utils {
    use rand::Rng;

    pub fn get_random_ints() -> (u8, u8, u8) {
        let mut rng = rand::thread_rng();

        (
            rng.gen_range(0..3), 
            rng.gen_range(0..3), 
            rng.gen_range(0..3)
        )
    }
}

pub mod slot_utils {
    use std::fmt;

    pub enum SpinStatus {
        Win {
            slot: u8
        },
        Lose {
            slot1: u8,
            slot2: u8,
            slot3: u8
        }
    }

    impl fmt::Display for SpinStatus {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match (self) {
                SpinStatus::Win { slot } => write!(f, "{} {} {} \n You Win!", slot, slot, slot),
                SpinStatus::Lose { slot1, slot2, slot3} => write!(f, "{} {} {} \n You Lose!", slot1, slot2, slot2)
            }
        }
    }
}