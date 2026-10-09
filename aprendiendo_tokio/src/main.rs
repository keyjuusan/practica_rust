use tokio::time::Instant;

use crate::ejercicios::{pizarra_pizzas};

mod ejercicios;

#[tokio::main]
async fn main() {
    let inicio = Instant::now();

    pizarra_pizzas::app().await;

    println!("{:?}",inicio.elapsed())
}