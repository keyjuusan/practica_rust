use std::sync::{Arc, Mutex};
use std::thread;

pub fn candado_envenenado() {
    let cuenta = Arc::new(Mutex::new(100));
    let cuenta_clon = Arc::clone(&cuenta);

    let hilo = thread::spawn(move || {
        let mut saldo = cuenta_clon.lock().unwrap();
        *saldo -= 50;
        panic!("¡Oh no, el hilo explotó con el candado abierto!");
    });

    // Esperamos a que el hilo falle
    let _ = hilo.join();

    // INTENTO DE LECTURA: Aquí ocurrirá el error por envenenamiento
    // El .unwrap() fallará porque el Mutex está envenenado (PoisonError)
    // let saldo_final = cuenta.lock().unwrap();

    // SOLUCION
    let saldo_final = match cuenta.lock() {
      Ok(saldo_mutex_guard)=>*saldo_mutex_guard,
      Err(envenenado_mutex_guard)=>{
        let saldo_mutex_guard = envenenado_mutex_guard.into_inner();
        *saldo_mutex_guard
      }
    };

    println!("Saldo: {}", saldo_final);
}
