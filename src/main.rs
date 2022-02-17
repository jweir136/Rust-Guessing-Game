#![allow(non_snake_case)]

mod slot_machine_utils;

fn greet() {
    println!("Welcome to Slot Machine!");
}

fn play(mut coins: usize) {
    loop {
        let mut line = String::new();
        println!("Enter 1 to play, 0 to exit");
        std::io::stdin().read_line(&mut line).unwrap();

        if line.trim().eq("1") {
            println!("You have {} coins.", coins);

            coins -= 1;

            let spin = slot_machine_utils::spin();
            println!("{}", spin);

            match spin {
                Win => coins += 3,
                Lose => coins += 0
            };
        } else if line.trim().eq("0") {
            println!("Good-bye!");
            break;
        } else {
            println!("Invalid Input.")
        }
    }
}

fn main() {
    greet();
    let mut coins: usize = 10;
    play(coins);
}