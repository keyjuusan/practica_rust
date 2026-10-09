use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::TcpListener};

pub async fn app() {
    let host = "127.0.0.1:1112";
    let tcp_listener = TcpListener::bind(host).await.unwrap();
    println!("Escuchando en: {host}");

    loop {
        match tcp_listener.accept().await {
            Ok((tcp_stream, socket_addres)) => {

                println!("Cliente conectado: {socket_addres}");

                tokio::spawn(async move {
                let (lector, mut escritor) = tcp_stream.into_split();
                    let mut lineas_lector = BufReader::new(lector);
                    let mut linea_temporal = String::new();
                    // let mut buf = [0; 1024];

                    loop {
                      linea_temporal.clear();
                        match lineas_lector.read_line(&mut linea_temporal).await {
                            Ok(0) => return,
                            Ok(_) => {
                              let inicio_ejecucion = tokio::time::Instant::now();
                              let orden = linea_temporal.trim();

                              match orden {
                                "ping" => {
                                  escritor.write_all(b"pong\n").await.unwrap();
                                },
                                _ => {}

                              }
                                // if let Ok(texto) = std::str::from_utf8(&buf[0..n]) {
                                //     let texto_limpio = texto.trim();
                                //     println!("{socket_addres}: {texto_limpio}");
                                //     if let Err(e) = escritor.write_all(format!("{texto_limpio}, como estas {socket_addres}\n").as_bytes()).await {
                                //       eprintln!("{e}");
                                //     };
                                // } else {
                                //     println!("Recibidos {n} bytes de datos no válidos para UTF-8");
                                // }
                                println!("{:?}",inicio_ejecucion.elapsed())
                            }
                            Err(e) => {
                                println!("Error al leer {socket_addres}: {e:?}");
                                return;
                            }
                        }
                    }
                });

                // tokio::spawn(async move {
                //   if let Err(e) = escritor.write_all(b"Bienvenido!\n").await{
                //     eprintln!("error de escritor: {e}")
                //   }
                // });

            }
            Err(e) => println!("No se pudo recibir el cliente: {:?}", e),
        };
    }
}
