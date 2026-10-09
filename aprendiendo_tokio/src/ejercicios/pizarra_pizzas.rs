use std::{sync::Arc, time::Duration};

use tokio::sync::{
    Mutex,
    mpsc::{self, Receiver, Sender},
};

pub async fn app() {
    let (repartidor, mut mostrador): (Sender<String>, Receiver<String>) = mpsc::channel(3);
    let repartidor2 = repartidor.clone();

    let pizarra_compartida = Arc::new(Mutex::new(0));

    let pizarra_cocinero1 = pizarra_compartida.clone();
    let pizarra_cocinero2 = pizarra_compartida.clone();

    println!("Abriendo pizzeria...");
    // cocinero #1
    tokio::spawn(async move {
        for i in 1..=3 {
            cocinar_pizza(i, &repartidor, 1, &pizarra_cocinero1).await;
        }
    });

    // cocinero #2
    tokio::spawn(async move {
        for i in 4..=6 {
            cocinar_pizza(i, &repartidor2, 2, &pizarra_cocinero2).await;
        }
    });

    // El mostrador ahora tiene un límite de tiempo por pizza
    while let Some(pizza) = mostrador.recv().await {
        println!("La {pizza} esta lista para llevar");
    }

    println!("Cerrando pizzeria...");
    println!(
        "Pizzas totales vendidas {}",
        pizarra_compartida.lock().await
    )
}

async fn cocinar_pizza(
    id_pizza: u8,
    repartidor: &Sender<String>,
    id_cocinero: u8,
    pizarra: &Arc<Mutex<u8>>,
) {
    let pizza = format!("Pizza #{id_pizza}");
    println!("[Cocinero #{id_cocinero}] Cocinando {pizza}...");
    let tiempo_coccion = if id_pizza == 2 {
        Duration::from_secs(3)
    } else {
        Duration::from_secs(1)
    };

    let resultado =
        tokio::time::timeout(Duration::from_secs(1), tokio::time::sleep(tiempo_coccion)).await;
    if resultado.is_err() {
        println!("Uy! Al cocinero #{id_cocinero} se le quemo la {pizza} :(");
    } else {
        repartidor.send(pizza.clone()).await.unwrap();
        println!("{pizza} lista!");
        let mut total_pizzas = pizarra.lock().await;
        *total_pizzas += 1
    }
}
