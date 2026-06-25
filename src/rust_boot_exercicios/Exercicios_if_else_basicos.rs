use std::io;

fn main() {
    
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        let trimmed = input.trim();
        let parts: Vec<&str> = trimmed.split_whitespace().collect();

    if parts.len() == 2 {
      println!("Welcome, {}! Your account type is {}.", parts[0], parts[1])
    } else { 
      println!("Invalid input.")}
      
    }
}

//=============================================================================================

use std::io;

fn main() {

    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Erro na leitura");

    let valores: Vec<u32> = entrada
        .split_whitespace()
        .filter_map(|s| s.trim().parse().ok())
        .collect();
  
    if valores[0] >= valores[1] {
        println!("Compra aprovada")
    } else {
        println!("Saldo insuficiente")
    }
  
}

//

fn e_par(numero: u32) -> bool {
    if numero % 2 == 0{ println!("O numero é par!"); true
    } else { println!("O numero é impar!"); false
    }
}

fn main() {
    let resultado = e_par(9);
    println!("{}", resultado)
}


//============================================================================================

fn primeira_letra(s: &String) -> char {
    s.chars().next().unwrap()
}

fn main() {
    let texto = String::from ("Rust");
    let primeira = primeira_letra(&texto);
    println!("{}", primeira)
}

//==============================================================================================

use std::io;
fn main() {
    loop {
        println!("Digite um numero: ");

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).expect("Erro...");

        let opcao: i32 = opcao.trim().parse().expect("Digite um caracter valido");
        
        match opcao {
            x if x > 0 => println!("Numero positivo"),
            x if x < 0 => println!("Numero negativo"),
            0 => println!("O numero é zero"),
        }
    }
}

//==============================================================================================

fn maior_elemento(v: &Vec<i32>) -> i32 {
    *v.iter().max().unwrap()

}

fn main() {
    let numero = vec![3, 17, 5, 9, 2];
    println!("O maior numero é {}", maior_elemento(&numero));

}



