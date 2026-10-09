use std::sync::{Arc, Mutex};
use std::thread;

pub fn arc_mutex() {
    // Creamos un contador protegido por un Mutex y compartido mediante un Arc
    let contador = Arc::new(Mutex::new(0));
    let mut hilos = vec![];

    for _ in 0..10 {
        // Clonamos el Arc (el puntero), NO el dato interno
        let contador_clon = Arc::clone(&contador);

        let hilo = thread::spawn(move || {
            // Adquirimos el candado para poder modificar el número
            let mut dato = contador_clon.lock().unwrap();
            *dato += 1;
            // El candado se libera automáticamente aquí al terminar el hilo
        });

        hilos.push(hilo);
    }

    // Esperamos a que todos los hilos terminen
    for hilo in hilos {
        hilo.join().unwrap();
    }

    // Imprimimos el resultado final
    println!("Resultado final: {}", *contador.lock().unwrap());
}
