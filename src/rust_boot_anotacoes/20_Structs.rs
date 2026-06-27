// Usando estrutura 
struct Usuario {
    id: u32,
    nome: String,
    saldo: f32,
}

fn main() {
    let usuario: Usuario = Usuario {
        id: 1,
        nome: String::from("Jao"),
        saldo: 1300.0,
    };

    println!("id: {}", usuario.id);
    println!("Nome: {}", usuario.nome);
    println!("Saldo: {}", usuario.saldo);

}

// //

// // Usando Array de Array

// fn main() {
//     let usuarios: [[&str; 3]; 2] = [
//         ["1", "Jao", "1300.0"],
//         ["2", "Maria", "2500.0"],
//     ];

//     println!("ID: {}", usuarios[0][0]);
//     println!("Nome: {}", usuarios[0][1]);
//     println!("Saldo: {}", usuarios[0][2]);
// }

// // 

// // Usando Tupla

// fn main() {
//     let usuario: (u32, &str, f32) = (1, "Jao", 1300.0);

//     println!("ID: {}", usuario.0);
//     println!("Nome: {}", usuario.1);
//     println!("Saldo: {}", usuario.2);
// }


// //

// // Usando Vec 

// fn main() {
//     let usuario: Vec<String> = vec![
//         String::from("1"),
//         String::from("Jao"),
//         String::from("1300.0"),
//     ];

//     println!("ID: {}", usuario[0]);
//     println!("Nome: {}", usuario[1]);
//     println!("Saldo: {}", usuario[2]);
// }

// // 

// // Usando Vec de tuplas

// fn main() {
//     let usuarios: Vec<(u32, &str, f32)> = vec![
//         (1, "Jao", 1300.0),
//         (2, "Maria", 2500.0),
//     ];

//     println!("ID: {}", usuarios[0].0);
//     println!("Nome: {}", usuarios[0].1);
//     println!("Saldo: {}", usuarios[0].2);
// }


//============================================================================================
