/*Existen diferentes tipos de traits: o herencia de traits.
    Asociados: tipos son por default y que son definidos cuando implementa el trait.
*/


trait Mascota{
    //Funciones que se pueden implementar con este trait.
    fn Hablar(&self)->String;
    fn Saludar(&self);
}

trait Animal{
    fn ContarPatas(&self)->u32;
}

trait Multiply{

}

trait Mascota:Animal{
    type Salida;
    fn Caminar(&self, otroAnimal:&Self)->Self::Salida;
    fn Hablar(&self)->String;
    fn Saludar(&self);
}

struct Perro{
    nombre: String,
    edad: i8,
}

impl Animal for Perro{
    fn ContarPatas(&self)->u32{
        4
    }
}

impl Mascota for Perro{
    fn Hablar(&self)->String{
        //String::From();
        format!("Change da worl mai fainal mesash.\n-{}",self.nombre)
    }

    fn Saludar(&self){
        println!("Saludos cordiales.\n{}",self.Hablar());
    }
}

fn main() {
    let perroObj = Perro{nombre:String::from("ScoobyDoo"),edad:7};
    dbg!(perroObj.Hablar());
    perroObj.Saludar();
    println!("El animal tiene {} patas",perroObj.ContarPatas());
}