fn main(){
    println!("---*Mutabilidad*---");
    Mutabilidad();
    println!("\n---*Inferencia de tipos*---");
    InferenciaDeTipos();
    println!("\n---*Ejercicio 1*---");
    let mut nFibonacci = 46;
    Fibonacci(nFibonacci);
    println!("Utilizando recursividad: {}", FibonacciRecursivo(nFibonacci));
}

//Mutabilidad
fn Mutabilidad(){
    let x=10;
    println!("El valor de x es: {x}");
    println!("Esta es la dirección de x antes de mutar: {:p}",&x);
    //No es posible cambiar el valor de una variable, ya que es inmutable por defecto.
    //x = 15;
    //Solución 1: Volver a declarar la variable.
    let x = 15;
    println!("Aquí cambió x: {x}");
    println!("Esta es la dirección de x después de mutar: {:p}",&x);
    //Solución 2: hacer que la variable sea mutable.
    //let mut x=10;
}

//Inferencia de tipos
fn FuncionEntero32(x: u32){
    println!("Función entero 32.");
}

fn FuncionEnteroI8(x: i8){
    println!("Función entero i8.");
}

fn InferenciaDeTipos(){
    //let x = -10.8674; //Esto manda un error.
    let x = 10;
    let y = 40;
    FuncionEnteroI8(x);
}

/*Ejercicio 1: Fibonacci
La serie de Fibonacci empieza con 0 y 1. El número siguientes es la suma de los dos anteriores.
0, 1, 1, 2, 3, 5, 8, 13...

Crear una función que calcule el elemento n de la serie de Fibonacci.
*/

fn Fibonacci(n: u32){
    if n<2{
        println!("El número debe ser mayor a 2.");
    }
    else{
        let mut suma1 = 0;
        let mut suma2 = 1;
        let mut suma = 0;

        for i in 2..=n{
            suma = suma1+suma2;
            suma1 = suma2;
            suma2 = suma;
        }
        println!("El valor {n} de Fibonacci es: {suma}");
    }
}

fn FibonacciRecursivo(n: u32) -> u32{
    if n < 2{
        return n;
    }
    else{
        return FibonacciRecursivo(n-1) + FibonacciRecursivo(n-2);
    }
}