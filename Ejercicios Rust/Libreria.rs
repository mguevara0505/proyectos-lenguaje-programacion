struct Libro {
    titulo: String,
    autor: String,
    fecha: u32,
}

impl Libro {
    fn resumen(titulo: &str, autor: &str, fecha: u32) -> Self {
        Self{
            titulo: titulo.to_string(), 
            autor: autor.to_string(), 
            fecha
        }
    }
}

fn main() {
    let l = Libro::resumen("Cien años de soledad", "Gabriel García Marquéz", 1967);
    println!("Nombre del libro: {}", l.titulo);
    println!("Nombre del autor: {}", l.autor);
    println!("Fecha de publicación: {}", l.fecha);
}