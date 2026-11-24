fn main() {
    let mut contador = 0;
    loop {
        println!("Este es un bucle infinito");
        contador += 1;
        if contador == 5 {
            break;
        }
    }
}
