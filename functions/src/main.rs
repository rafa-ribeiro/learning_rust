fn main() {
    println!("Hello, world!");

    another_function(10, 'h');
}

fn another_function(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
