//Tipos de datos enteros
// con signo : i..., i8, i16, i32, i64, i128
// sin signo : u..., u8, u16, u32, u64, u128
//Tipos de datos flotantes, deciimales
// f32, f64

fn main() {
    let mut x:u32=5;
    println!("El valor de x es: {}", x); //imprimimos el valor de x
    //x=-3699999;
    //cannot apply unary operator `-`
    // (comentado para que el programa compile: un u32 no puede ser negativo)
    x = 3699999; //un valor positivo si es valido para u32
    println!("El valor de x es: {}", x); //imprimimos el nuevo valor de x
    //pon un ejemplo de datos flotantes con f32 y f64, y que muestre la diferencia de precision entre ambos

    // f32 ocupa 32 bits y guarda unos 7 digitos exactos.
    // f64 ocupa 64 bits y guarda unos 15-16 digitos exactos.
    // Si no indicas el tipo, Rust usa f64 por defecto para los decimales.
    // Guardamos el mismo numero en los dos tipos y vemos que guarda cada uno.
    let pi32: f32 = 3.14159265358979323846;
    let pi64: f64 = 3.14159265358979323846;
    println!("pi en f32: {}", pi32); //solo guarda unos 7 digitos
    println!("pi en f64: {}", pi64); //guarda unos 15-16 digitos

    let precio32: f32 = 123456789.123;
    let precio64: f64 = 123456789.123;
    println!("123456789.123 en f32: {}", precio32); //no le caben los decimales y redondea
    println!("123456789.123 en f64: {}", precio64); //lo guarda bien
    
}
