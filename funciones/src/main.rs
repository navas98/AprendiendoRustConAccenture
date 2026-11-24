fn main() {
    mi_funcion(2, 6);
    let x = {
        let z = mi_funcion(2, 6) + 1;
        z + 1
    };
    println!("El valor de x es: {}", x);
    println!("El valor de trece es: {}", trece());
    println!("El valor de catorce es: {}", catorce())
}

fn mi_funcion(i: i32, j: i32) -> i32 {
    i + j
}
fn trece() -> i32 {
    13
    //Esta es la más usada, ya que no es necesario poner el return y el punto y coma, ya que Rust entiende que es el valor de retorno de la función.
}
fn catorce() -> i32 {
    return 14;
}