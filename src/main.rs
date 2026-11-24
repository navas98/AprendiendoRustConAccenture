/*
1. Crear un estrucura alumno que almacene:
    . Nombre
    . Edad
    . Nota
    . Estado de alumno
    . Puntos acumulados
2. Crear un enum llamado Estado que pueda terner 3 valores:
    . Aprobado
    . suspenso
    . SinEvaluar
3. Crear una funcion que permita consturir un alumno nuevo
4. Crear una funcion que permita modificar la nota de un alumno
5. Cuando se introduce una nota
    . Si la nota es mayor o igual a 6, el estado del alumno pasa a Aprobado
    . Si la nota es menor a 6, el estado del alumno pasa a Suspenso
6. Crear una funcion para aladir puntos acumulados a un alumno
7. Crear una funcion que reciba una referencia al alumno y muestre su nombre sin tomar su propiedad
8. Utiliza match para mostrar un mensaje dependiendo del estado del alumno
    . Aprobado :" Felicidades, has aprobado
    . Suspenso: "Lo siento, has suspendido
    . SinEvaluar: "Aún no se ha evaluado
9. Crear un trait que permita mostrar la información del alumno
10. El main:
    . Crear un alumno llamado Javier, de 25 años
    . Asignarle una nota de 7 y 10 puntos acumulados
    . Mostrar la información del alumno
11. Finalmente, crear test que comprueben
    . Que un alunmmno nuevo empieza con 0 puntos 
    . Que una nota de 8 produce el estado aprobado
    . Que una nota de 3 produce el estado suspenso
*/
enum Estado{
    Aprobado,
    Suspenso,
    SinEvaluar
}
struct Alumno{
    nombre: String,
    edad:u32,
    nota: f32,
    estado: Estado,
    puntos_acumulados: u32
 }
 impl Alumno{
    fn nuevo(nombre:String, edad:u32)->Alumno{
        Alumno{
            nombre,
            edad,
            nota:0.0,
            estado:Estado::SinEvaluar,
            puntos_acumulados:0
        }
    }

    fn poner_nota(&mut self, nota: f32) {
        self.nota = nota;
        if nota >= 6.0 {
            self.estado = Estado::Aprobado;
        } else {
            self.estado = Estado::Suspenso;
        }
    }

    // 6. Añadir puntos: += suma a lo que ya había (no lo reemplaza)
    fn anadir_puntos(&mut self, puntos: u32) {
        self.puntos_acumulados += puntos;
    }
 }

// 7. Recibe &Alumno (una referencia): puede leer el alumno, pero no se queda con él.
//    Después de llamarla, el main puede seguir usando el alumno.
fn mostrar_nombre(alumno: &Alumno) {
    println!("Nombre: {}", alumno.nombre);
}

// 8. match obliga a cubrir TODOS los valores del enum.
//    &'static str = un texto fijo escrito en el código.
fn mensaje_estado(estado: &Estado) -> &'static str {
    match estado {
        Estado::Aprobado => "Felicidades, has aprobado",
        Estado::Suspenso => "Lo siento, has suspendido",
        Estado::SinEvaluar => "Aún no se ha evaluado",
    }
}

// 9. Un trait es como un "contrato": dice qué funciones debe tener un tipo.
trait MostrarInfo {
    fn mostrar_info(&self);
}

// Aquí Alumno cumple el contrato, escribiendo cómo se muestra su información.
impl MostrarInfo for Alumno {
    fn mostrar_info(&self) {
        mostrar_nombre(self);
        println!("Edad: {}", self.edad);
        println!("Nota: {}", self.nota);
        println!("Puntos acumulados: {}", self.puntos_acumulados);
        println!("{}", mensaje_estado(&self.estado));
    }
}

// 10. El main
fn main() {
    let mut alumno = Alumno::nuevo(String::from("Javier"), 25);
    alumno.poner_nota(7.0);
    alumno.anadir_puntos(10);
    alumno.mostrar_info();
}

// 11. Tests: solo se compilan al ejecutar `cargo test`.
#[cfg(test)]
mod tests {
    use super::*; // trae todo lo de arriba (Alumno, Estado...) a los tests

    #[test]
    fn alumno_nuevo_empieza_con_cero_puntos() {
        let alumno = Alumno::nuevo(String::from("Test"), 20);
        assert_eq!(alumno.puntos_acumulados, 0);
    }

    #[test]
    fn nota_8_es_aprobado() {
        let mut alumno = Alumno::nuevo(String::from("Test"), 20);
        alumno.poner_nota(8.0);
        assert!(matches!(alumno.estado, Estado::Aprobado));
    }

    #[test]
    fn nota_3_es_suspenso() {
        let mut alumno = Alumno::nuevo(String::from("Test"), 20);
        alumno.poner_nota(3.0);
        assert!(matches!(alumno.estado, Estado::Suspenso));
    }
}