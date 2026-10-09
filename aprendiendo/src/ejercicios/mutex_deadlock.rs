use std::sync::Mutex;

pub fn mutex_deadlock() {
    let cofre = Mutex::new("Oro");

    // Primera solicitud: Éxito. Guardamos el candado en la variable '_guardia1'
    let _guardia1 = cofre.lock().unwrap();
    println!("Tengo el acceso 1");

    // SOLUCION
    drop(_guardia1);

    // Segunda solicitud: ¡CONGELADO!
    // El programa se quedará aquí eternamente esperando que '_guardia1' se libere.
    let _guardia2 = cofre.lock().unwrap();
    println!("Tengo el acceso 2 (Nunca llegará aquí)");
}
