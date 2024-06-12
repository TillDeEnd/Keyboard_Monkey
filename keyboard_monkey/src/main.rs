fn input() -> String {
    use std::io::stdin;
    let mut user_input: String = String::new();
    stdin()
        .read_line(&mut user_input)
        .expect("An error occured while getting user input.");
    return user_input;
}

fn generate_random_char() -> char { // This function seems like a very goofy way to do this. But whatever, if it works it works.
    use rand::Rng;
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let random_int: i32 = rng.gen_range(1..26);

    match random_int {
        1 => return 'q',
        2 => return 'w',
        3 => return 'e',
        4 => return 'r',
        5 => return 't',
        6 => return 'y',
        7 => return 'u',
        8 => return 'i',
        9 => return 'o',
        10 => return 'p',
        11 => return 'a',
        12 => return 's',
        13 => return 'd',
        14 => return 'f',
        15 => return 'g',
        16 => return 'h',
        17 => return 'j',
        18 => return 'k',
        19 => return 'l',
        20 => return 'z',
        21 => return 'x',
        22 => return 'c',
        23 => return 'v',
        24 => return 'b',
        25 => return 'n',
        26 => return 'm',
        _ => return '!',
    }
}

fn main() {
    println!("");
    println!("Hello, have you heard about the infinite monkey theorem?");
    println!("How given enough monkeys, one of them could eventually write something cool.");
    println!("This program is meant to turn your computer into a very fast monkey.");
    println!("");
    println!("Press Ctrl + C to end the program when you are done.");
    println!("");
    println!("Press Enter to continue...");
    input();

    loop {
        let monkey_spam: char = generate_random_char();
        print!("{monkey_spam}");
    }

}