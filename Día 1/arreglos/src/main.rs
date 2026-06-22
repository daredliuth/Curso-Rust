fn main() {
    //Arreglos
    let mut arreglo1: [i8;3] = [5,2,8];
    println!("{}",arreglo1[1]);
    println!("{arreglo1:?}");
    arreglo1[1] = 7;
    println!("{}",arreglo1[1]);
    println!("{arreglo1:?}");
        for elementos in arreglo1{
        println!("{elementos}");
    }
    for i in 0..arreglo1.len(){
        println!("{}",arreglo1[i]);
    }

    //Tuplas
    let tupla1:(i8, bool) = (2,false);
    println!("\n{0:?}",tupla1.0);
    println!("{0:?}",tupla1.0);
}
