use create::enums::Tipo; // enums == nome da pasta
// use create::enums::*;  =  " * "== todos os tipos da pasta

pub struct Pessoa {
    pub nome: String,
    pub documento: String,
    pub tipo: Tipo,
}

impl Pessoa {
    pub fn new(nome: &str, documento: &str, tipo: Tipo) -> Pessoa {
        Pessoa {
            nome: nome.to_string(),
            documento: documento.to_string(),
            tipo: tipo
        }
    }

    pub fn show(&self) {
        println!("Nome: {}", self.nome);
        println!("Documentos: {}", self.documento);
        println!("Tipo: {}", self.tipo_string);
    }

    fn tipo_string(&self) -> &str {
        match self.tipo {
            Tipo::Fisica => "Fisica",
            Tipo::Juridica => "Juridica", 
        }
    }

}