fn TakesTuple(tuple: (char, i32, bool)){
    let achar = tuple.0;
    let bentero = tuple.1;
    let cbooleano = tuple.2;
    //Destructuración.
    let (achar,bentero,cbooleano) = tuple;
    let (_,bentero,cbooleano) = tuple;
    let(..,cbooleano) = tuple;
}

struct Movimiento{
    delta: (i32, i32),
    repeat: u32
}

enum Resultado{
    Ok(i32),
    Err(String)
}

fn DivisibleEntreDos(n:i32)->Resultado{
    if n%2 == 0{
        Resultado::Ok(n / 2)
    } else{
        Resultado::Err(String::from("No se puede dividir entre dos."))
    }
}

fn main() {
    let valorAComparar = 'x';
    match valorAComparar{
        'e' => println!("Exit"),
        'a'|'s'|'w' => println!("Coincide con a, s, o w."),
        //key if key.is_lowercase() => println!("{key} es una letra minúscula."),
        pedro if pedro.is_lowercase() => println!("{pedro} es una letra minúscula."), //¿Dafaq? Podemos utilizar otro nombre para la variable del match (Enlazamiento).
        _ => println!("Otro caso.")
    }

    let mov = Movimiento{delta:(1,2),repeat:4};
    match mov{
        Movimiento {delta:(0,0),..} => println!("Solo se comparó el delta."),
        Movimiento {delta:(x,0),repeat} => println!("Solo se comparó el delta."),
        Movimiento {delta:(0,y),repeat:2} => println!("Solo se comparó el delta."),
        Movimiento {delta:(1,y),repeat} => println!("Coincide el primer valor de delta."),
        _ => println!("Otro caso.")
    }

    let inputn = 32;
    match DivisibleEntreDos(inputn){
        //Resultado{..,String::from("No se puede dividir entre dos.")} => println!("Error."),
        //Resultado{inputn/2,..} => println!("Numero divisible entre dos."),
        Resultado::Ok(x) => println!("Numero {inputn} divisible entre dos = {x}."),
        Resultado::Err(x) => println!("Error."),
        _=>println!("Otro caso.")
    }

    dormirPor(1.0);

    println!("Resultado: {:?}", HexOrDieTrying(Some(String::from("eoo"))));
}

//if let ejecuta un código dependiendo de la comparación de un valor con un patrón.
use std::time::Duration;
fn dormirPor(secs: f32){
    let resultado = Duration::try_from_secs_f32(secs);
    if let Ok(duration) = resultado{
        std::thread::sleep(duration);
        println!("Se ha dormido por {duration:?}");
    }
}

/*
enum Option{
    Some(T),
    None
}

enum Result{
    Ok(T),
    Err(T)
}
*/

fn HexOrDieTrying(maybeString: Option<String>) -> Resultado{
    let s = if let Some(s) = maybeString{
        s
    } else{
        return Err(String::from("Got none"));
    };

    let firstByteChar = if let Some(first) = s.chars().next(){
        first
    } else{
        return Err(String::from("Got empty sring"));
    };

    let digit = if let Some(digit) = firstByteChar.to_digit(16){
        digit
    } else{
        return Err(String::from("Not a hex digit."));
    };
    Ok(digit)
}