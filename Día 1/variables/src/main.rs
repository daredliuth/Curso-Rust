fn main(){
    println!("---*Mutabilidad*---");
    Mutabilidad();

    println!("\n---*Inferencia de tipos*---");
    InferenciaDeTipos();
    
    println!("\n---*Ejercicio 1*---");
    let mut nFibonacci = 46;
    Fibonacci(nFibonacci);
    //println!("Utilizando recursividad: {}", FibonacciRecursivo(nFibonacci));

    println!("\n---*Alcance de las variables*---");
    AlcanceVariable();

    println!("\n---*EStructuras de control*---");
    EstructurasDeControl();

    println!("\n---*Ejercicio 2*---");
    let mut nCollatz = 11;
    println!("La longitud de collatz con n={nCollatz} es: {}", CollatzLongitud(nCollatz));
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

//Alcance de las variables.
fn AlcanceVariable(){
    let afuera = 5;
    let x = {
        let adentro = 1;
        9+adentro
    };
    println!("Valor de x: {x}");
    //println!("Valor de adentro {adentro}");//Esto marca un error por el alcance la variable.
}

//Estructuras de control
fn EstructurasDeControl(){
    //if
    let x = 10;
    if x== 1{
        println!("Equis es igual a 1.");
    } else if x == 2{
        println!("Equis es igual a 2.");
    } else{
        println!("Equis es igual a otro valor.");
    }

    let tam = if x<15 {"mini"} else {"grande"};
    println!("El valor {x} es {tam}");

    //match
    let x = 10;
    match x{
        1 => println!("Uno."),
        2 => println!("Dos."),
        _ => {println!("Otro")}
    }

    //while
    let mut x = 8;
    while x >= 3 {
        println!("Se está ejecutando el ciclo güail.");
        x = x-1;
    }
    println!("Valor final de x: {x}");

    //for
    for i in 1..5{
        println!("{i}");
    }
    for elemento in [2,3,4,5]{
        println!("{elemento}");
    }

    //loop
    let mut contador = 0;
    loop{
        contador += 1;
        if contador == 5{
            continue; //Deja de ejecutar el resto del loop.
        }
        if contador < 10{
            println!("Se ejecuta el loop hasta el 10. {contador}");
        }
        if contador == 10{
            break;
        }
    }
}

/*Ejercicio 2: Secuencia COLLATZ.
Para un número n que sea mayor a 0.
Cuando n es 1, la secuencia termina en ni.
Cuando n es par, entonces ni+1 = ni / 2.
Cuando n es impar, entonces ni+1 = 3 * ni +1;

Ejemplo:
n = 3
n es impar, por ende 3 * 3 + 1 = 10
10 es par, por ende 10 / 2 = 5
5 es impar, 16.
16-8
8-4
4-2
2-1
1, terminamos.
*/

fn CollatzLongitud(mut n:u32) -> u32{
    let mut contador = 0;
    println!("n inicial: {n}");
    while n > 1{
        contador += 1;
        if(n % 2 == 0){
            n = n / 2;
        } else{
            n = 3 * n + 1;
        }
        println!("El nuevo número es: {n}, la longitud es: {contador}");
    }
    //return contador;
    //también se puede realizar de esta manera:
    contador
}