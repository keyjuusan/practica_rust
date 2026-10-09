use std::sync::Arc;
use std::thread;

pub fn modificar_sin_mutex() {
    // Usamos SOLO Arc, sin Mutex
    let numero = Arc::new(5);
    let numero_clon = Arc::clone(&numero);

    thread::spawn(move || {
        // ERROR DE COMPILACIÓN: Esto va a fallar inmediatamente antes de correr
        *numero_clon = 10;
    });
}
