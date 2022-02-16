use rand::Rng;
use std::fmt;

pub enum SpinStatus {
    Win(u8),
    Lose(u8, u8, u8)
}

pub fn spin() -> SpinStatus {
    let mut rng = rand::thread_rng();

    let slot1: u8 = rng.gen_range(0..3);
    let slot2: u8 = rng.gen_range(0..3);
    let slot3: u8 = rng.gen_range(0..3);

    if slot1 == 0 && slot2 == 0 && slot3 == 0 {
        SpinStatus::Win(0)
    } else if slot1 == 1 && slot2 == 1 && slot3 == 0 {
        SpinStatus::Win(1)
    } else if slot1 == 2 && slot2 == 2 && slot3 == 2 {
        SpinStatus::Win(2)
    } else {
        SpinStatus::Lose(slot1, slot2, slot3)
    }
}