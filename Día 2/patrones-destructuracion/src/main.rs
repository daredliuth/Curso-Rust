fn main() {
    println!("---*Destructuración*---");
    let tupla = (1,2,3);
    println!("Este es el return de la función RevisarOrden: {}",RevisarOrden(tupla));

    println!("---*Arreglos anidados*---");
    let matriz1 = [[1,2,3],[4,5,6],[7,8,9]];
    //println!("Matriz original:\n{matriz1:?}\n");
    //println!("Matriz transpuesta:\n{:?}",Transpuesta(matriz1));
    println!("Matriz original:");
    ImprimirMatriz(matriz1);
    println!("Matriz transpuesta:");
    ImprimirMatriz(Transpuesta(matriz1));
}

//Destructuración
fn RevisarOrden(tu: (i32,i32,i32))-> bool{
    //Destructuración.
    let (var1, var2, var3) = tu;
    var1 < var2 && var2 < var3 //Retorno de la función.
}

//Arreglos anidados
fn ImprimirMatriz(matriz: [[i32;3];3]){
    for m in 0..matriz.len(){
        println!("{:?}",matriz[m]);
    }
    println!("");
}

fn Transpuesta(ma1: [[i32;3];3])->[[i32;3];3]{
    //Escribir el algoritmo para transponer una matriz.
    let mut matrizAux: [[i32;3];3] = [[0,0,0],[0,0,0],[0,0,0]];
    for m in 0..ma1.len(){
        for n in 0..ma1[0].len(){
            matrizAux[m][n] = ma1[n][m];
        }
    }
    matrizAux
}