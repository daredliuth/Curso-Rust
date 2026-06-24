fn main() {
    println!("---*Estructuras*---");
    Estructuras();

    println!("\n---*Enums*---");
    Enums();

    println!("\n---*Constantes*---");
    Constantes();
}

//Estructuras
#[derive(Clone)]//Permite clonar a la estructura.
struct profesor{
    nombre: String,
    edad: u8,
    jubilado: bool,
}

//Estructura Tupla, cuando los nombres de los campos definidos no son importantes.
struct TuplaEjemplo(i32,i32);

//Otras estructuras.
struct Newtons(f64);
struct LibrasFuerza(f64);

fn Estructuras(){

    let mut profesor1 = profesor{
        nombre: String::from("Papaez"), //Los dos puntos es un trade. Una función definida que se implementa con alguna variavle.
        edad: 35,
        jubilado: false,
    };
    let mut profesorClon = profesor1.clone();
    DescribirProfesor(&profesor1);

    profesor1.edad = 45;
    profesor1.jubilado = true;
    DescribirProfesor(&profesor1);

    let tupla = TuplaEjemplo(10,20);
    println!("{}",tupla.0);
}

fn DescribirProfesor(prof: &profesor){
    println!("El nombre del profesor es {}, tiene {} años.", prof.nombre, prof.edad);
}

//Estructura enum.
#[derive(Debug)]
enum Direccion {
    Derecha,
    Izquierda
}
#[derive(Debug)]
enum MovimientoJugador{
    Pase,
    Correr(Direccion),
    MovimientoRapido {x:u32, y:u32}
}

enum ObjetoDireccion{
    Enfrente,
    Atras
}

type Objeto = ObjetoDireccion;
use std::cell::RefCell;
use std::sync::{Arc,RwLock};

//type Inventario = RwLock<Vec<Arc<RefCell<Item>>>>;

fn Enums(){
    let direc = Direccion::Izquierda;
    let movimientoJugador: MovimientoJugador = MovimientoJugador::Correr(direc);
    println!("El movimiento del jugador es el siguiente: {movimientoJugador:?}");
}

//Constantes
const EDAD_PROMEDIO: i32 = 50;
const FUNCION_CONSTANTE: u8 = FuncionConstanteValorEntero();

const fn FuncionConstanteValorEntero()->u8{
    if EDAD_PROMEDIO > 10 {1} else {0}
}

fn Constantes(){
    let mut valorFuncionConstante = FUNCION_CONSTANTE;
    println!("Este valor viene de una función constante: {valorFuncionConstante:?}");
    println!("Este es el valor constante: {EDAD_PROMEDIO:?}");
}