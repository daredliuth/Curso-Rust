#[derive(Clone)]
struct Carro{
    modelo: String,
    atributos: Vec<i32>,
}

//Creamos métodos para la estructura carro.
impl Carro{
    //Constructor
    fn new(nombre: &str) -> Self{
        Self {modelo: String::from(nombre), atributos: Vec::new()}
    }

    //Función para adicionar un atributo.
    fn adicionar_atributo(&mut self, atributo: i32){
        self.atributos.push(atributo);
    }

    fn leer_atributos(&self){
        println!("Número de atributos: {}\nModelo del carro: {}\nAtributos:", self.atributos.len(), self.modelo);
        for elemento in self.atributos.iter(){
            println!("{}", elemento);
        }
    }
}

fn main() {
    let mut carrazo = Carro::new("Chevy Pop");
    carrazo.adicionar_atributo(10);
    carrazo.adicionar_atributo(9);
    carrazo.leer_atributos();
}
