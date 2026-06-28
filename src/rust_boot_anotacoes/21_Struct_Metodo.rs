#[derive(Debug)]

struct Cliente {
    id: u32,
    nome: String,
    cpf: String,
    salario: f32
}

impl Cliente {
    fn cpf_valido(&self) -> bool {
        if self.cpf.is_empty() {
            return false;
        }
        true
    } // Verifica se o CPF não está vazio, retorna true ou false

    fn add_sobrenome(&mut self) {
        self.nome += " Victor";
    } // Concatena " Victor" ao nome do cliente

    fn aumento_salario(&mut self, valor: f32) {
        self.salario += valor;
    } // Soma o valor recebido ao salário atual do cliente
}

fn main() {
    let mut cliente = Cliente {
        id: 1,
        nome: String::from("Jao"),
        cpf: String::from("111.222.333-45"),
        salario: 3000.0,
    };

    cliente.add_sobrenome();

    cliente.aumento_salario(200.0);

    let validar: &str = if cliente.cpf_valido() {
        "Valido"
    } else {
        "Invalido"
    };

    println!(
        "Cliente {} id {}, cpf {} e salario {}\nresultado: {}",
        cliente.nome,
        cliente.id,
        cliente.cpf,
        cliente.salario,
        validar
    );
}