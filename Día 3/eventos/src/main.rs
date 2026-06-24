/*Ejercicio: Elevador*/

enum Evento{
    BotonPresionado(Boton),
    AbiertoPuertas,
    CerradoPuertas,
    Llegar
}

enum Direccion{
    Arriba,
    Abajo
}
enum Piso{
    Uno,
    Dos,
    Tres,
    Cuatro,
    Cinco
}

enum Boton{
    IrLobby(Direccion,Piso),
    IrPiso(Piso)
}

fn YaLLego(piso: i32)->Evento{
    Evento::Llegar
}

fn AbrirPuertas()->Evento{
    Evento::AbiertoPuertas

}

fn CerrarPuertas()->Evento{
    Evento::CerradoPuertas
}

fn BotonPresionadoLobby(piso: i32, direc:Direccion)->Evento{
    Evento::BotonPresionado(Boton::IrLobby(direc,piso))
}

fn BotonPresionadoElevador(piso: i32, direc:Direccion)->Evento{
    Evento::BotonPresionado(Boton::IrPiso(direc,piso))
}


fn main() {
    println!("Se ha presionado el botón en el piso {:?}", BotonPresionadoPiso(0,Direccion::Arriba));
    println!("Las puertas se han abierto {:?}", AbrirPuertas());
}