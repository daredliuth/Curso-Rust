fn main() {
    println!("---*Referencias*---");
    Referencias();

    println!("\n---*Slices*---");
    Slices();

    println!("\n---*Ejercicio 1*---");
    let mut vector = [4.0, 5.0, 6.0];
    println!("Magnitud del vector {vector:?}: {}",CalcularMagnitud(&vector));
    
    NormalizarVector(&mut vector);
    println!("Vector normalizado: {vector:?}");
}


//Referencias
fn Referencias(){
    //let a = 'a';
    //let mut a = 'a';
    //let b = 'b';
    //let mut r = a; //Se pasa completamente la variable, incluyendo el espacio de memoria.
    //let mut r = &a; //Se referencia el valor.
    //let r = &mut a;

    //dbg!(r);
    //println!("Espacio de memoria {:p}",&r);
    //println!("Espacio de memoria {:p}",&a);
    //r = &b;
    //r = &mut b;
    //*r = 't';
    //dbg!(r);
    //println!("Espacio de memoria {:p}",&r);
    //let x_referencia = {
    //    let equis = 1;
    //    &equis //Marca error porqué la referencia no puede vivir fuera del scope.
    //}
    //dbg(x_referencia); 
}

//Slices
fn Slices(){
    let a:[i32;6] = [1,2,3,4,5,6];
    //Tomamos una sola porción del arreglo.
    let rebanada: &[i32] = &a[1..3]; //Tomamos los valores del 1 al 3.
    //let rebanada: &[i32] = &a[1.]; //Tomamos los valores desde el 1 hasta el final.
    println!("Rebanada: {rebanada:?}");
}

/*Ejercicio 1: Geometría.
Se practica referencias. Implementar dos funciones, la primera para calcular
magnitud de un vector, y la segunda para normalizar el mismo vector.
*/
fn CalcularMagnitud(refVector: &[f64;3])-> f64{
    let mut suma = 0.0;
    for elemento in refVector{
        suma += elemento*elemento;
    }
    suma.sqrt()
}

fn NormalizarVector(refVector: &mut[f64;3]){
    let magnitud = CalcularMagnitud(refVector);

    for i in 0..refVector.len(){
        refVector[i] = refVector[i]/magnitud;
    }
}

fn NormalizarVectorProfe(refVector: &mut[f64;3]){
    let magnitud = CalcularMagnitud(refVector);

    for elemento in refVector{
        *elemento /= magnitud;
    }
}