use std::{ time::Duration};

use tokio::{ sync::mpsc::{self, Receiver, Sender}, time::sleep};

pub async fn app(){
  let (sender, mut receiver):(Sender<String>,Receiver<String>) = mpsc::channel(3);
  let sender2 = sender.clone();
  println!("Abriendo pizzeria...");

  tokio::spawn(async move {
    for i in 1..=3{
      cocinar_pizza(i,&sender).await;
    }
  });

  tokio::spawn(async move {
    for i in 4..=6{
      cocinar_pizza(i,&sender2).await;
    }
  });


  while let Some(pizza) = receiver.recv().await {
    println!("la pizza fue recibida y empaquetada");
  }

  println!("Cerrando pizzeria...");
}

async fn cocinar_pizza(numero_pizza:u8,sender:&Sender<String>){
  sleep(Duration::from_secs(1)).await;
  let pizza = format!("Pizza de peperoni #{numero_pizza}");
  sender.send(pizza.clone()).await.unwrap();
  println!("Cocina: {pizza} lista!");
}