fn cuadrado(numero: i32) -> i32 {
    let cuadrado = numero.pow(2);
    println!("El cuadrado del número es: {}", cuadrado);
    numero
}

fn main() {
    let numero = 5;
    cuadrado(numero);
}
