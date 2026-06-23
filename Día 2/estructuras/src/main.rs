fn main() {
    println!("---*Estructuras*---");
    Estructuras();
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

