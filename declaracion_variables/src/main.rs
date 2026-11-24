fn main() {
    //las variables en rust se crean del siguiente modo:
    let x=5; //declaramos una variable llamada x y le asignamos el valor 5
    //Creacion de una variable llamada x, de tipo entero (indentificado por el contexto)
    println!("El valor de x es: {}", x); //imprimimos el valor de x en la consola
    // Ls {} sirven para poner el valor de la variable en el texto que se imprime
    //Para ejecutar es cargo run 
    //x=1; 
    // cannot assign twice to immutable variable
    
    //Para cambiar el valor de una variable, debemos declararla como mutable, de la siguiente manera:
    let mut y=5; //declaramos una variable mutable llamada y y le asignamos el valor 5
    println!("El valor de y es: {}", y); //imprimimos el valor de y en la consola
    y=10; //asignamos un nuevo valor a la variable y
    println!("El valor de y es: {}", y); //imprimimos el nuevo valor de y en la consola
}
