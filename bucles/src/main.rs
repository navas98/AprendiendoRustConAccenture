fn main() {
    // =========================================================
    // MISMO PROBLEMA, TRES BUCLES: cuenta atrás de 5 a 1
    // =========================================================

    // ---- 1. loop ----
    // Se repite para siempre hasta que encuentra un "break".
    println!("--- loop ---");
    let mut contador = 5;
    loop {
        println!("{}...", contador);
        contador -= 1;
        if contador == 0 {
            break; // sale del bucle
        }
    }
    println!("¡Despegue!");

    // ---- 2. while ----
    // Se repite mientras la condición sea verdadera.
    println!("--- while ---");
    let mut contador = 5;
    while contador > 0 {
        println!("{}...", contador);
        contador -= 1;
    }
    println!("¡Despegue!");

    // ---- 3. for ----
    // Recorre un rango; no hace falta variable "mut" ni restar a mano.
    println!("--- for ---");
    for contador in (1..=5).rev() {
        println!("{}...", contador);
    }
    println!("¡Despegue!");

    // =========================================================
    // MISMO PROBLEMA, TRES BUCLES: recorrer un array
    // =========================================================
    let frutas = ["manzana", "pera", "plátano"];

    println!("--- loop ---");
    let mut i = 0;
    loop {
        if i == frutas.len() {
            break;
        }
        println!("Fruta {}: {}", i, frutas[i]);
        i += 1;
    }

    println!("--- while ---");
    let mut i = 0;
    while i < frutas.len() {
        println!("Fruta {}: {}", i, frutas[i]);
        i += 1;
    }

    println!("--- for ---");
    for fruta in frutas {
        println!("Fruta: {}", fruta);
    }

    // =========================================================
    // EXTRA: lo que hace especial a cada uno
    // =========================================================

    // loop puede DEVOLVER un valor con "break valor".
    let mut n = 1;
    let primera_potencia_mayor_de_100 = loop {
        n *= 2;
        if n > 100 {
            break n;
        }
    };
    println!("Primera potencia de 2 mayor que 100: {}", primera_potencia_mayor_de_100);

    // while es ideal cuando no sabes cuántas vueltas vas a dar.
    let mut suma = 0;
    let mut numero = 1;
    while suma <= 20 {
        suma += numero;
        numero += 1;
    }
    println!("La suma pasó de 20 al llegar a {}", suma);

    // for con "continue": salta los números impares.
    print!("Números pares del 1 al 10:");
    for numero in 1..=10 {
        if numero % 2 != 0 {
            continue; // pasa a la siguiente vuelta
        }
        print!(" {}", numero);
    }
    println!();
}
