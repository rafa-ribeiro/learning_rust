use std::io;

fn main() {
    
    println!("Digite o Enésimo termo da sequência de Fibonacci que deseja visualizar: ");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Erro ao tentar ler o enésimo elemento informado.");


    let n: u128 = n.trim().parse().expect("Erro ao converter o enésimo termo para um valor inteiro");

    println!("Sequência de Fibonacci até o termo {}", n);

    let mut a: u128 = 1;
    let mut b: u128 = 1;
    println!("{} -> {}", 0, a);
    println!("{} -> {}", 1, b);
    
    for i in 2..n {
        let c = a + b;
        println!("{} -> {}", i, c);
        a = b;
        b = c;
    }
}
