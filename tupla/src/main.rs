fn main() {
    let tupla:(i32, &str, bool)=(1, "Hola", true); //declaramos una tupla llamada tupla y le asignamos los valores 1, "Hola" y true
    let(x, y, z)=tupla; //desestructuramos la tupla y asignamos los valores a las variables x, y y z-desestructuracion de la tupla
    println!("El valor de x es: {}", x); //imprimimos el valor de x
    println!("El valor de y es: {}", y); //imprimimos el valor de y
    println!("El valor de z es: {}", z); //imprimimos el valor de z
    let primero=tupla.0; //accedemos al primer valor de la tupla
}
