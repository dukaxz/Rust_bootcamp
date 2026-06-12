fn main() {
    // Variáveis do tipo i32 ficam na Stack e implementam a trait Copy
    let x: i32 = 4; // x é dona do valor 4
    let y: i32 = x; // o valor é copiado para y, não ocorre transferência de ownership

    println!("O valor de x é: {} - Referencia {:p}", x, &x);
    println!("O valor de y é: {} - Referencia {:p}", y, &y);
}

//=========================================================================================

// Abordando por referência

fn main() {
    // Variáveis Copy armazenadas na Stack
    let x: i32 = 4; // owner (dona do valor)
    let y: &i32 = &x; // y recebe uma referência para x, sem copiar o valor

    println!("O valor de x é: {} - Referencia {:p}", x, &x);
    println!("O valor de y é: {} - Referencia {:p}", y, y);
}

/*
Neste caso não ocorre cópia do valor.

x continua sendo a única dona do valor 4.

y apenas guarda o endereço onde o valor de x está armazenado.

Memória:

x -> 4
      ^
      |
      y

Enquanto y existir, ela depende da existência de x.
Rust garante que uma referência nunca aponte para um valor já destruído.
*/

//==========================================================================

// Desreferenciação

fn main() {
    let x: i32 = 4;
    let y: &i32 = &x; // Referencia 1

    let t: &i32 = y; // Referencia 2

    let w: i32 = *y; // Desreferenciação da memoria do y
    println!("Endereço de: x {}, y {}, t {}, w {}", x, y, t, w);
    println!("Endereço de: x {:p}, y {:p}, t {:p}, w {:p}", &x, &y, &t, &w);

}