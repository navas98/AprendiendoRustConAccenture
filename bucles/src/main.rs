fn main() {
    // Ejemplo 1: cuenta atrás con while.
    // El bucle se repite mientras la condición sea verdadera.
    let mut numero = 5; // "mut" porque vamos a cambiar su valor
    while numero > 0 {
        println!("{}...", numero);
        numero -= 1; // igual que: numero = numero - 1
    }
    println!("¡Despegue!");

    // Ejemplo 2: recorrer un array con while usando un índice.
    let frutas = ["manzana", "pera", "plátano"];
    let mut indice = 0;
    while indice < frutas.len() {
        println!("Fruta {}: {}", indice, frutas[indice]);
        indice += 1;
    }

    // Ejemplo 3: sumar números hasta pasar de 20.
    let mut suma = 0;
    let mut n = 1;
    while suma <= 20 {
        suma += n;
        n += 1;
    }
    println!("La suma pasó de 20 al llegar a {} (último número sumado: {})", suma, n - 1);

    // Tu código original: "while true" es un bucle infinito (nunca termina).
    // En Rust, para un bucle infinito se usa "loop" en lugar de "while true".
    // Lo dejo comentado para que el programa no se quede colgado:
    //
    // while true {
    //     println!("Este es un bucle infinito");
    //     while false {
    //         println!("Este bucle nunca se ejecutará");
    //     }
    // }
}
