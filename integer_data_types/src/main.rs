use std::io;


fn main() {
    println!("Lets test some Integer Data types");

    let mut number : u8;
    loop {
        println!("Digite um número");

        let mut new_number = String::new();

        io::stdin()
            .read_line(&mut new_number)
            .expect("Failed to read new number value");

        number = new_number.trim().parse().expect("Failed to convert to a integer type");

        if number == 0 {
            println!("Exited!");
            break;
        }

    }
}
