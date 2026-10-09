use std::io;

pub fn nombre_cli(){
  let mut nombre = String::new();
  println!("ingresa tu nombre:");
  io::stdin().read_line(&mut nombre).expect("texto vacio");
  nombre.push_str("es un genio!");
  println!("{nombre}");
}