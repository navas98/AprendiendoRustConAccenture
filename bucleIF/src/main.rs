fn main() {
    let numero = 5;
    if numero < 5 {
        println!("El número es menor que 5");
    } else if numero == 5 {
        println!("El número es igual a 5");
    } else {
        println!("El número es mayor que 5");
    }
    let condicion = true;
    let numero = if condicion { 5 } else { 6 };
    println!("El valor de numero es: {}", numero);
}
