fn crear_mensaje() -> String {
    let frase = String::from("Dos pasos atrás");
    frase
}

fn mostrar_mensaje() {
    let llamado = crear_mensaje();
    println!("{}", llamado);
}

fn main() {
    mostrar_mensaje();
}