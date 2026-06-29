use std::rc::Rc;

enum OperacionesAritmeticas {
    Suma,
    Resta,
    Mult,
    Div,
}

enum Expresion {
    Operacion {
        op: OperacionesAritmeticas,
        derecho: Rc<Expresion>,
        izquierdo: Rc<Expresion>,
    },
    Valor(i64),
}

fn eval(e: Rc<Expresion>) -> i64 {
    match &*e {
        Expresion::Operacion { op, derecho, izquierdo } => {
            let derecho = eval(derecho.clone());
            let izquierdo = eval(izquierdo.clone());

            match op {
                OperacionesAritmeticas::Suma => izquierdo + derecho,
                OperacionesAritmeticas::Resta => izquierdo - derecho,
                OperacionesAritmeticas::Mult => izquierdo * derecho,
                OperacionesAritmeticas::Div => izquierdo / derecho,
            }
        }
        Expresion::Valor(v) => *v,
    }
}

fn main() {
    let expr = Rc::new(Expresion::Operacion {
        op: OperacionesAritmeticas::Suma,
        derecho: Rc::new(Expresion::Valor(20)),
        izquierdo: Rc::new(Expresion::Valor(40)),
    });

    let resultado = eval(expr);
    println!("El resultado es {:?}", resultado);
}