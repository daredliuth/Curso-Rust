enum OperacionesAritmeticas{
    Suma,
    Resta,
    Mult,
    Div
}

enum Expresion{
    Operacion{op: OperacionesAritmeticas, derecho:Box<Expresion>, izquierdo:Box<Expresion>},
    Valor(i64)
}

fn eval(e:Expresion)->i64{
    /*
    let mut resultado = 0;
    match crate::Expresion::op{
        Suma => resultado = crate::Expresion::izquierdo + crate::Expresion::derecho,
        Resta => resultado = crate::Expresion::izquierdo - crate::Expresion::derecho,
        Mult => resultado = crate::Expresion::izquierdo * crate::Expresion::derecho,
        Div => resultado = crate::Expresion::izquierdo / crate::Expresion::derecho,
        _ => resultado = 0
    }*/

    match e{
        Expresion::Operacion{op,derecho,izquierdo}=>{
            let derecho = eval(*derecho);
            let izquierdo = eval(*izquierdo);

            match op{
                OperacionesAritmeticas::Suma => izquierdo + derecho,
                OperacionesAritmeticas::Resta => izquierdo - derecho,
                OperacionesAritmeticas::Mult => izquierdo * derecho,
                OperacionesAritmeticas::Div => izquierdo / derecho,
            }
        }
        Expresion::Valor(v) => v,
    }
}

fn main() {
    //Cambiar box por otro tipo de punteros.
    let prueba = eval(Expresion::Operacion{op: OperacionesAritmeticas::Suma, derecho:Box::new(Expresion::Valor(20)), izquierdo:Box::new(Expresion::Valor(40))});
    println!("El resultado es {:?}", prueba);
}
