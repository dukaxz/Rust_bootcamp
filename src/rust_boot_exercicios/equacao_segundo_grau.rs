use std::io;


fn delta(a: f64, b: f64, c: f64) -> f64 {
    b.powi(2) - 4.0 * a * c
}


fn execute_equacao_segundo_grau() {
    let mut a: String = String::new();
    let mut b: String = String::new();
    let mut c: String = String::new();
    
    println!("Digite o valor de A: ");
    io::stdin().read_line(&mut a).expect("Falha ao ler a linha");      
        
    println!("Digite o valor de B: ");
    io::stdin().read_line(&mut b).expect("Falha ao ler a linha");   

    println!("Digite o valor de C: ");
    io::stdin().read_line(&mut c).expect("Falha ao ler a linha");

    let a: f64 = a.trim().parse().expect("Digite um numero valido!");
    let b: f64 = b.trim().parse().expect("Digite um numero valido!");  
    let c: f64 = c.trim().parse().expect("Digite um numero valido!");  
    
    println!("\nEquação informada: {}x² + {}x + {} = 0.", a, b, c);
    println!("calculo de delta: Δ = {}² - 4 * {} * {}.", b, a, c);
    println!("Portanto o valor de delta (Δ) é: {}.", delta(a, b, c));
    
    let d: f64 = delta(a, b, c);
    if d < 0.0 {
        println!("Não existem raízes reais.");
    } else if d == 0.0 {
        let x = -b / (2.0 * a);
        println!("Uma raiz real: x = {}.", x);
    } else {
        let sqrt_d = d.sqrt();
        let x1 = (-b + sqrt_d) / (2.0 * a);
        let x2 = (-b - sqrt_d) / (2.0 * a);
        println!("Duas raízes reais: x1 = {}, x2 = {}.", x1, x2);
    }
}


fn menu() {
    loop {
    println!("\nEquação do segundo grau!!");
    println!("\n1.Execute o calculo\n2.Sair.");
    
    let mut opcao: String = String::new();
        io::stdin().read_line(&mut opcao).expect("Erro...");
    
    let opcao: i16 = opcao.trim().parse().expect("Digite um caracter valido");

    match opcao {
        1 => execute_equacao_segundo_grau(),
        2 => break,
        _ => println!("Opção invalida."),
    };

    }
}

fn main(){
    menu()
}