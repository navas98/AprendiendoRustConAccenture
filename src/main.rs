/*
Ejercico tema 2: Prediccion del precio de la vivienda
Una inmoviliaria quiere crear un pequeño sistema de Machine Learning supervisado para predecir el precio de la vivienda
Disponemos de informaciond e diferentes viviendas y conocemos el precio real.
Cada vivienda tendra 3 features:
- Superficie en metros cuadrados
- Numero de habitaciones
- Antiguedad de la vivienda en años
Y tendra un target:
- Precio de la vivienda en euros
Por ejemplo:
Vivienda 1: Metros:80, habitaciones:3, antiguedad:10, precio:150000
Vivienda 2: Metros:100, habitaciones:4, antiguedad:5, precio:200000
vienda 3: Metros:60, habitaciones:2, antiguedad:20, precio:100000

Objetivo: Crear un programa en Rust que haga una simulacion del proceso de machine learning.
1. Crear una esturta que represente una vivienda
2. Crear dataset con 10 viviendas diferentes
3. Identificares cuales son las features (x) y las targets (y)
4. Dividir los datos en :
    - Datos de entrenamiento (80%)
    - Datos de prueba (20%)
5. Crear un modelo sencillo que estime el precio utilizando las caracteristicas de cada vivienda
6. Utilizar el modelo para realizar predicciones unicamente sobre las viiviendas del dataset de prueba
7. Para cada vivienda del test mostrar: precio real, precio estimado y error cometido
8. Crear funciones para calcular: MAE (Mean Absolute Error), MSE (Mean Squared Error), RMSE (Root Mean Squared Error) para evaluar el rendimiento del modelo
9. Mostrar las metricas obtenidas y determinar si el modelo esta cometiendo errores grandes o pequeños en sus predicciones
10. Convertir posteriormente el probble en uno de clasificacion, clasificando las viviendas en 3 categorias: barata, media y cara, segun el precio de la vivienda
11. Comparar las clases reales con las predichas
12.Calcular el accuracy del clasificador
*/

// Paso 1: Crear una estructura que represente una vivienda
struct Vivienda {
    metros: f64,
    habitaciones: u32,
    antiguedad: u32,
    precio: f64,
}

// ---------------------------------------------------------------------------
// Paso 5: El modelo -> Regresion lineal entrenada con descenso de gradiente
// ---------------------------------------------------------------------------
// La idea: el precio se estima como una suma ponderada de las features:
//
//     precio ≈ w1*metros + w2*habitaciones + w3*antiguedad + b
//
// - pesos (w1, w2, w3): cuanto influye cada feature en el precio.
// - sesgo (b): el precio "base" cuando todas las features valen la media.
//
// "Entrenar" = buscar los pesos y el sesgo que hacen que el error sea minimo.
//
// Detalle importante: las features tienen escalas muy distintas
// (metros ~100, habitaciones ~3). Si no las igualamos, el descenso de gradiente
// se vuelve inestable. Por eso las NORMALIZAMOS: a cada valor le restamos la
// media y lo dividimos por la desviacion tipica. Asi todas quedan en torno a 0.
// Guardamos esas medias y desviaciones en el modelo porque hay que aplicar
// exactamente la misma transformacion al predecir viviendas nuevas.
struct Modelo {
    medias: [f64; 3],
    desviaciones: [f64; 3],
    pesos: [f64; 3],
    sesgo: f64,
}

// 'impl' es donde se escriben las funciones (metodos) que pertenecen a un struct.
impl Modelo {
    // Crea y entrena un modelo a partir de los datos de entrenamiento.
    // - tasa_aprendizaje: lo grande que es cada "pasito" al corregir los pesos.
    // - epocas: cuantas veces recorremos todos los datos corrigiendo los pesos.
    fn entrenar(x: &[[f64; 3]], y: &[f64], tasa_aprendizaje: f64, epocas: usize) -> Modelo {
        let n = x.len() as f64;

        // 1) Media de cada feature (columna).
        let mut medias = [0.0; 3];
        for fila in x {
            for j in 0..3 {
                medias[j] += fila[j];
            }
        }
        for j in 0..3 {
            medias[j] /= n;
        }

        // 2) Desviacion tipica de cada feature: cuanto se separan los valores de la media.
        let mut desviaciones = [0.0; 3];
        for fila in x {
            for j in 0..3 {
                desviaciones[j] += (fila[j] - medias[j]).powi(2);
            }
        }
        for j in 0..3 {
            desviaciones[j] = (desviaciones[j] / n).sqrt();
            // Si todas las viviendas tuvieran el mismo valor, la desviacion seria 0
            // y dividiriamos por 0. En ese caso usamos 1 para no romper nada.
            if desviaciones[j] == 0.0 {
                desviaciones[j] = 1.0;
            }
        }

        // 3) Empezamos con todos los pesos y el sesgo a 0 (el modelo no sabe nada).
        let mut modelo = Modelo {
            medias,
            desviaciones,
            pesos: [0.0; 3],
            sesgo: 0.0,
        };

        // 4) Descenso de gradiente: repetimos muchas veces
        //    "predecir -> medir el error -> corregir los pesos un poquito".
        for _ in 0..epocas {
            // Los gradientes indican hacia donde hay que mover cada peso para reducir el error.
            let mut gradiente_pesos = [0.0; 3];
            let mut gradiente_sesgo = 0.0;

            // zip recorre x e y a la vez, emparejando cada vivienda con su precio real.
            for (fila, &precio_real) in x.iter().zip(y) {
                let fila_normalizada = modelo.normalizar(fila);
                let error = modelo.predecir_normalizado(&fila_normalizada) - precio_real;

                // Cada peso se corrige en proporcion al error y al valor de su feature.
                for j in 0..3 {
                    gradiente_pesos[j] += error * fila_normalizada[j];
                }
                gradiente_sesgo += error;
            }

            // Movemos los pesos en sentido CONTRARIO al gradiente (por eso se resta),
            // usando el gradiente medio (dividido entre n).
            for j in 0..3 {
                modelo.pesos[j] -= tasa_aprendizaje * gradiente_pesos[j] / n;
            }
            modelo.sesgo -= tasa_aprendizaje * gradiente_sesgo / n;
        }

        modelo
    }

    // Aplica la normalizacion a una fila usando las medias y desviaciones del entrenamiento.
    // &self significa que el metodo solo lee el modelo, no lo modifica.
    fn normalizar(&self, fila: &[f64; 3]) -> [f64; 3] {
        let mut resultado = [0.0; 3];
        for j in 0..3 {
            resultado[j] = (fila[j] - self.medias[j]) / self.desviaciones[j];
        }
        resultado
    }

    // Formula del modelo sobre una fila ya normalizada: w1*x1 + w2*x2 + w3*x3 + b
    fn predecir_normalizado(&self, fila_normalizada: &[f64; 3]) -> f64 {
        let mut resultado = self.sesgo;
        for j in 0..3 {
            resultado += self.pesos[j] * fila_normalizada[j];
        }
        resultado
    }

    // Prediccion para una vivienda "normal" (sin normalizar): es la que usamos desde fuera.
    fn predecir(&self, fila: &[f64; 3]) -> f64 {
        let fila_normalizada = self.normalizar(fila);
        self.predecir_normalizado(&fila_normalizada)
    }
}

// ---------------------------------------------------------------------------
// Paso 8: Metricas de error
// ---------------------------------------------------------------------------
// Las tres reciben los precios reales y los predichos (en el mismo orden).

// MAE (Mean Absolute Error): media de los errores en valor absoluto.
// Se lee facil: "de media, el modelo se equivoca en X euros".
fn mae(reales: &[f64], predichos: &[f64]) -> f64 {
    let mut suma = 0.0;
    for (r, p) in reales.iter().zip(predichos) {
        suma += (r - p).abs();
    }
    suma / reales.len() as f64
}

// MSE (Mean Squared Error): media de los errores al cuadrado.
// Al elevar al cuadrado, castiga mucho mas los errores grandes.
// Su unidad es "euros al cuadrado", por eso no se interpreta directamente.
fn mse(reales: &[f64], predichos: &[f64]) -> f64 {
    let mut suma = 0.0;
    for (r, p) in reales.iter().zip(predichos) {
        suma += (r - p).powi(2);
    }
    suma / reales.len() as f64
}

// RMSE (Root Mean Squared Error): raiz cuadrada del MSE.
// Vuelve a estar en euros, pero sigue castigando mas los errores grandes que el MAE.
fn rmse(reales: &[f64], predichos: &[f64]) -> f64 {
    mse(reales, predichos).sqrt()
}

// ---------------------------------------------------------------------------
// Paso 10: Clasificacion en categorias
// ---------------------------------------------------------------------------
// Un enum representa un valor que solo puede ser UNA de varias opciones.
// derive genera codigo automaticamente:
// - Debug: para poder imprimirlo con {:?}
// - PartialEq: para poder compararlo con ==
// - Clone, Copy: para poder copiarlo como si fuera un numero
#[derive(Debug, Clone, Copy, PartialEq)]
enum Categoria {
    Barata,
    Media,
    Cara,
}

// Umbrales de precio para cada categoria.
// 'const' define valores fijos que no cambian nunca.
const LIMITE_BARATA: f64 = 150_000.0; // por debajo -> Barata
const LIMITE_CARA: f64 = 200_000.0; // desde aqui -> Cara (entre medias -> Media)

// Convierte un precio en su categoria. 'if' en Rust devuelve un valor,
// por eso no hace falta escribir 'return'.
fn clasificar(precio: f64) -> Categoria {
    if precio < LIMITE_BARATA {
        Categoria::Barata
    } else if precio < LIMITE_CARA {
        Categoria::Media
    } else {
        Categoria::Cara
    }
}

// Paso 12: Accuracy = aciertos / total.
// Es el porcentaje de viviendas cuya categoria predicha coincide con la real.
fn accuracy(reales: &[Categoria], predichas: &[Categoria]) -> f64 {
    let mut aciertos = 0;
    for (r, p) in reales.iter().zip(predichas) {
        if r == p {
            aciertos += 1;
        }
    }
    aciertos as f64 / reales.len() as f64
}

fn main() {
    // Paso 2: Crear un dataset con 10 viviendas diferentes
    let viviendas = vec![
        Vivienda { metros: 80.0, habitaciones: 3, antiguedad: 10, precio: 150000.0 },
        Vivienda { metros: 100.0, habitaciones: 4, antiguedad: 5, precio: 200000.0 },
        Vivienda { metros: 60.0, habitaciones: 2, antiguedad: 20, precio: 100000.0 },
        Vivienda { metros: 90.0, habitaciones: 3, antiguedad: 15, precio: 180000.0 },
        Vivienda { metros: 120.0, habitaciones: 5, antiguedad: 2, precio: 250000.0 },
        Vivienda { metros: 70.0, habitaciones: 2, antiguedad: 30, precio: 120000.0 },
        Vivienda { metros: 110.0, habitaciones: 4, antiguedad: 8, precio: 220000.0 },
        Vivienda { metros: 85.0, habitaciones: 3, antiguedad: 12, precio: 160000.0 },
        Vivienda { metros: 95.0, habitaciones: 4, antiguedad: 7, precio: 190000.0 },
        Vivienda { metros: 75.0, habitaciones: 2, antiguedad: 25, precio: 130000.0 },
    ];

    // Paso 3: Separar features (X) y target (y)
    // X: datos de entrada del modelo (metros, habitaciones, antiguedad).
    //    Cada fila es un array de 3 f64, por eso X es un Vec<[f64; 3]>.
    // y: lo que queremos predecir (el precio), un Vec<f64>.
    let mut x: Vec<[f64; 3]> = Vec::new();
    let mut y: Vec<f64> = Vec::new();

    // Recorremos las viviendas con & para solo leerlas (prestamo),
    // asi el vector 'viviendas' se puede seguir usando despues.
    for v in &viviendas {
        // habitaciones y antiguedad son u32: los convertimos a f64 con 'as'
        // para que todos los features tengan el mismo tipo.
        x.push([v.metros, v.habitaciones as f64, v.antiguedad as f64]);
        y.push(v.precio);
    }

    // Paso 4: Dividir los datos en entrenamiento (80%) y prueba (20%)
    // Calculamos cuantas viviendas van a entrenamiento en vez de escribir 8 a mano,
    // asi si cambiamos el tamano del dataset el codigo sigue funcionando.
    // x.len() es usize: lo pasamos a f64 para multiplicar por 0.8
    // y el resultado lo volvemos a usize porque los indices son usize.
    let n_train = (x.len() as f64 * 0.8) as usize;

    // Usamos slices (&vector[rango]): son "vistas" de un trozo del vector, sin copiar datos.
    // [..n_train] va desde el principio hasta n_train (sin incluirlo): indices 0..=7
    // [n_train..] va desde n_train hasta el final: indices 8 y 9
    let x_train: &[[f64; 3]] = &x[..n_train];
    let x_test: &[[f64; 3]] = &x[n_train..];
    let y_train: &[f64] = &y[..n_train];
    let y_test: &[f64] = &y[n_train..];

    println!("=== Datos ===");
    println!("Entrenamiento: {} viviendas", x_train.len());
    println!("Prueba:        {} viviendas", x_test.len());

    // Paso 5: Entrenar el modelo SOLO con los datos de entrenamiento.
    // El test se guarda aparte para evaluar con viviendas que el modelo no ha visto.
    let modelo = Modelo::entrenar(x_train, y_train, 0.1, 5000);

    println!("\n=== Modelo entrenado ===");
    println!("Pesos (metros, habitaciones, antiguedad): {:.0?}", modelo.pesos);
    println!("Sesgo: {:.0}", modelo.sesgo);

    // Paso 6: Predecir unicamente sobre las viviendas de prueba.
    let mut predicciones: Vec<f64> = Vec::new();
    for fila in x_test {
        predicciones.push(modelo.predecir(fila));
    }

    // Paso 7: Mostrar precio real, estimado y error de cada vivienda de prueba.
    // {:>10.0} = alinear a la derecha en 10 caracteres y sin decimales.
    println!("\n=== Predicciones sobre el test ===");
    println!("{:<10} {:>10} {:>10} {:>10}", "Vivienda", "Real", "Estimado", "Error");
    for i in 0..x_test.len() {
        let error = predicciones[i] - y_test[i];
        println!(
            "{:<10} {:>10.0} {:>10.0} {:>10.0}",
            i + 1,
            y_test[i],
            predicciones[i],
            error
        );
    }

    // Paso 8 y 9: Calcular y mostrar las metricas.
    let valor_mae = mae(y_test, &predicciones);
    let valor_mse = mse(y_test, &predicciones);
    let valor_rmse = rmse(y_test, &predicciones);

    println!("\n=== Metricas de regresion ===");
    println!("MAE:  {:.2} €", valor_mae);
    println!("MSE:  {:.2} €²", valor_mse);
    println!("RMSE: {:.2} €", valor_rmse);

    // Para saber si el error es "grande" o "pequeno" lo comparamos con el precio
    // medio de las viviendas de prueba: no es lo mismo fallar 10.000 € en una casa
    // de 100.000 € que en una de 1.000.000 €.
    let precio_medio_test = y_test.iter().sum::<f64>() / y_test.len() as f64;
    let error_relativo = valor_rmse / precio_medio_test * 100.0;

    println!("Error relativo (RMSE / precio medio): {:.2} %", error_relativo);
    if error_relativo < 5.0 {
        println!("Conclusion: el modelo comete errores PEQUENOS.");
    } else if error_relativo < 15.0 {
        println!("Conclusion: el modelo comete errores MODERADOS.");
    } else {
        println!("Conclusion: el modelo comete errores GRANDES.");
    }

    // Paso 10: Convertir el problema en clasificacion.
    // Clasificamos tanto el precio real como el predicho de cada vivienda de prueba.
    let mut clases_reales: Vec<Categoria> = Vec::new();
    let mut clases_predichas: Vec<Categoria> = Vec::new();
    for i in 0..y_test.len() {
        clases_reales.push(clasificar(y_test[i]));
        clases_predichas.push(clasificar(predicciones[i]));
    }

    // Paso 11: Comparar las clases reales con las predichas.
    println!("\n=== Clasificacion (Barata < 150.000 € <= Media < 200.000 € <= Cara) ===");
    println!("{:<10} {:>8} {:>10} {:>10}", "Vivienda", "Real", "Predicha", "Acierto");
    for i in 0..clases_reales.len() {
        let acierto = if clases_reales[i] == clases_predichas[i] { "Si" } else { "No" };
        // {:?} imprime el nombre de la variante del enum; format! lo convierte en texto
        // para poder alinearlo igual que los numeros.
        println!(
            "{:<10} {:>8} {:>10} {:>10}",
            i + 1,
            format!("{:?}", clases_reales[i]),
            format!("{:?}", clases_predichas[i]),
            acierto
        );
    }

    // Paso 12: Accuracy del clasificador.
    let valor_accuracy = accuracy(&clases_reales, &clases_predichas);
    println!("\nAccuracy: {:.0} %", valor_accuracy * 100.0);
}

// ---------------------------------------------------------------------------
// Tests: se ejecutan con `cargo test`
// ---------------------------------------------------------------------------
// #[cfg(test)] hace que este modulo solo se compile al ejecutar los tests.
#[cfg(test)]
mod tests {
    // 'super::*' importa todo lo del archivo de arriba (funciones, structs, enums).
    use super::*;

    #[test]
    fn metricas_con_errores_conocidos() {
        // Errores: 10 y -20 -> MAE = (10+20)/2 = 15, MSE = (100+400)/2 = 250
        let reales = [100.0, 200.0];
        let predichos = [110.0, 180.0];
        assert_eq!(mae(&reales, &predichos), 15.0);
        assert_eq!(mse(&reales, &predichos), 250.0);
        assert_eq!(rmse(&reales, &predichos), 250.0_f64.sqrt());
    }

    #[test]
    fn metricas_sin_error_son_cero() {
        let reales = [150000.0, 200000.0];
        assert_eq!(mae(&reales, &reales), 0.0);
        assert_eq!(rmse(&reales, &reales), 0.0);
    }

    #[test]
    fn clasificar_respeta_los_umbrales() {
        assert_eq!(clasificar(149_999.0), Categoria::Barata);
        assert_eq!(clasificar(150_000.0), Categoria::Media);
        assert_eq!(clasificar(199_999.0), Categoria::Media);
        assert_eq!(clasificar(200_000.0), Categoria::Cara);
    }

    #[test]
    fn accuracy_cuenta_los_aciertos() {
        let reales = [Categoria::Barata, Categoria::Media, Categoria::Cara, Categoria::Cara];
        let predichas = [Categoria::Barata, Categoria::Cara, Categoria::Cara, Categoria::Media];
        assert_eq!(accuracy(&reales, &predichas), 0.5);
    }

    #[test]
    fn el_modelo_aprende_una_relacion_lineal_exacta() {
        // Si el precio es exactamente 2000*metros, el modelo deberia acertar casi perfecto.
        let x = [[50.0, 1.0, 10.0], [80.0, 2.0, 5.0], [100.0, 3.0, 20.0], [120.0, 4.0, 1.0]];
        let y = [100_000.0, 160_000.0, 200_000.0, 240_000.0];
        let modelo = Modelo::entrenar(&x, &y, 0.1, 20000);
        let prediccion = modelo.predecir(&[90.0, 2.0, 8.0]);
        assert!((prediccion - 180_000.0).abs() < 1000.0, "prediccion = {}", prediccion);
    }
}

/*
===============================================================================
COSAS A TENER EN CUENTA
===============================================================================

--- Sobre Rust ---

1. f64 mejor que f32 para decimales.
   f32 solo guarda ~7 digitos exactos. Al elevar errores al cuadrado (MSE)
   salen numeros enormes y f32 pierde precision. f64 guarda ~15 digitos.

2. Los decimales f64 se escriben con punto: 80.0, no 80.
   Si escribes 80, Rust lo interpreta como entero y da error de tipos.

3. Rust no mezcla tipos en operaciones: hay que convertir con 'as'.
   Ejemplo: v.habitaciones as f64   (u32 -> f64)
            x.len() as f64          (usize -> f64)

4. 'mut' es obligatorio si una variable va a cambiar (por ejemplo, al hacer push).

5. '&' = prestar sin perder la propiedad (borrowing).
   'for v in &viviendas' lee el vector sin "comerselo", asi se puede seguir usando.

6. Slices (&vector[rango]) son trozos de un vector SIN copiar los datos.
   &x[..8] -> indices 0 a 7      &x[8..] -> del 8 hasta el final
   El final del rango NO se incluye.

7. Los indices y tamanos en Rust son de tipo usize.

8. 'struct' agrupa datos; 'impl' le anade metodos; '&self' = el metodo solo lee.

9. 'enum' = un valor que solo puede ser UNA de varias opciones (Barata/Media/Cara).
   #[derive(Debug, PartialEq, ...)] permite imprimirlo con {:?} y compararlo con ==.

10. zip recorre dos listas a la vez emparejando elementos (vivienda <-> precio).

11. Formato en println!:
    {:?}    -> imprimir arrays, vectores y enums
    {:.2}   -> 2 decimales
    {:>10}  -> alinear a la derecha en 10 caracteres

12. Guarda el archivo antes de ejecutar (Auto Save esta activado en VS Code).

13. Comandos utiles:
    cargo run    -> compilar y ejecutar
    cargo test   -> ejecutar los tests
    cargo fmt    -> ordenar el formato del codigo automaticamente

--- Sobre Machine Learning ---

14. Features (X) = datos de entrada. Target (y) = lo que queremos predecir.

15. Entrenar SOLO con los datos de entrenamiento. El test sirve para evaluar
    con viviendas que el modelo no ha visto nunca (como un examen con preguntas nuevas).

16. En proyectos reales hay que MEZCLAR los datos al azar antes de dividir en
    train/test. Aqui no se hace: el test son simplemente las 2 ultimas viviendas.

17. Normalizar las features (restar media, dividir por desviacion tipica) cuando
    tienen escalas distintas (metros ~100 vs habitaciones ~3). Sin esto el
    descenso de gradiente es inestable.
    IMPORTANTE: la media y la desviacion se calculan con el TRAIN y se reutilizan
    tal cual para el test.

18. Descenso de gradiente: repetir "predecir -> medir error -> corregir pesos un poco".
    - tasa de aprendizaje (0.1): tamano de cada paso. Muy grande = se descontrola;
      muy pequena = tarda mucho en aprender.
    - epocas (5000): cuantas veces se repite el proceso.

19. Metricas:
    - MAE:  error medio en euros. La mas facil de entender.
    - MSE:  castiga mucho los errores grandes. Esta en euros al cuadrado.
    - RMSE: raiz del MSE, vuelve a estar en euros.
    Para saber si un error es grande, compararlo con el precio medio (error relativo).

20. Accuracy = aciertos / total. Con solo 2 viviendas de test, un 100% NO demuestra
    que el modelo sea bueno: un solo fallo lo bajaria al 50%. Hacen falta mas datos.

21. Un peso negativo (habitaciones) no significa que mas habitaciones bajen el precio.
    Pasa porque habitaciones y metros estan muy relacionadas y el modelo atribuye
    casi todo el efecto a los metros.
===============================================================================
*/
