mod enums;
mod models;

// use enums::*;
// ou
use enums::Tipo;
use enums::Sexo;

fn main() {
    let daniel = Pessoa::new(
        "Daniel",
        "222.333.333.32",
        Tipo::Fisica
    );

    daniel.show();

    println!("{}", "-".to_string().repeat(20));

    let c_e_c = Pessoa::new(
        "C&C",
        "222.222.00000-33",
        Tipo::Juridica,
    );

    c_e_c.show();

    println!("{}", "-".to_string().repeat(20));

    let sexo_f = Sexo::Feminino;
    let sexo_m = Sexo::Masculino;
    let sexo_o = Sexo::Outros;
    
}

fn sexo_string(sexo: Sexo) -> &'static str {

    match sexo {
        Sexo::Masculino => "Masculino",
        Sexo::Feminino => "Feminino",
        Sexo::Outros => "Outros",

    }
}