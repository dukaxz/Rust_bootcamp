use std::io;

fn soma_entre_valores(x: i16, y: i16) -> i16 {
    x + y
    
}

fn subtracao_entre_valores(x: i16, y: i16) -> i16 {
    x - y
}

fn solicita_parametro_para_calculo(soma: bool) {
    {
        let mut x: String = String::new();
        let mut y: String = String::new();
                
        println!("Digite o primeiro valor: ");
        io::stdin().read_line(&mut x).expect("Falha ao ler a linha");      
        
        println!("Digite o segundo valor: ");
        io::stdin().read_line(&mut y).expect("Falha ao ler a linha");                
        
        let x: i16 = x.trim().parse().expect("Digite um numero valido!");
        let y: i16 = y.trim().parse().expect("Digite um numero valido!");
        let resultado = if soma { soma_entre_valores(x, y) } else { subtracao_entre_valores(x, y) };

        println!("O resultado entre os valores é de: {}", resultado);
    }
}

fn solicita_tabuada() {
    println!("Digite o valor da tabuada desejada:");

    let mut valor_tabuada: String = String::new();
    io::stdin().read_line(&mut valor_tabuada).expect("Falha ao ler a linha");

    let valor_tabuada: i32 = valor_tabuada.trim().parse().expect("Digite um caracter valido");

    for multiplicador in 1..=10 {
        println!("{} x {} = {}",valor_tabuada,multiplicador,multiplicador * valor_tabuada);
    }
}
 

fn menu() {
    loop {
        println!("\nEscolha uma opção:");

        println!(
            r#"
1. Soma entre valores
2. Subtração entre valores
3. Criar a tabuada de um numero
0. Encerrar
"#
        );

        let mut opcao: String = String::new();
        io::stdin().read_line(&mut opcao).expect("Erro...");

        let opcao: i16 = opcao.trim().parse().expect("Digite um caracter valido");

        match opcao {
            1 => solicita_parametro_para_calculo(true),
            2 => solicita_parametro_para_calculo(false),
            3 => solicita_tabuada(),
            0 => break,
            _ => println!("Opção invalida."),
        };
    }
}

fn main() {
    menu()
}