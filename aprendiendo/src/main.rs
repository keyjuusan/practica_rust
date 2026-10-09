use std::time::Instant;

use crate::ejercicios::modificar_sin_mutex::modificar_sin_mutex;

mod ejercicios;

fn main() {
  let tiempo_inicio = Instant::now();

  modificar_sin_mutex();

  println!("Duracion de ejecucion: {:?}",tiempo_inicio.elapsed())
}
