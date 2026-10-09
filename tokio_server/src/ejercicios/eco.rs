use tokio::net::TcpListener;

pub async fn app(){
  let listener = TcpListener::bind("127.0.0.1:1111").await.unwrap();
  println!("Servidor escuchando: http://127.0.0.1:1111");

  loop {
      // 2. Aceptar una nueva conexión entrante
      let (mut socket, direccion) = listener.accept().await.unwrap();
      println!("Nueva conexión aceptada desde: {}", direccion);

      tokio::spawn(async move {
          // 1. Dividir el socket en lector y escritor
          let (mut lector, mut escritor) = socket.split();

          // 2. Copiar directamente todo lo que entra hacia la salida
          if let Err(e) = tokio::io::copy(&mut lector, &mut escritor).await {
              eprintln!("Error al copiar datos del cliente {}: {}", direccion, e);
          }

          println!("Cliente {} desconectado.", direccion);
      });
  }
}