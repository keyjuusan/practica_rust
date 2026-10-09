use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

pub async fn app() {
    let host = "127.0.0.1:1112";
    let tcp_listener = TcpListener::bind(host).await.unwrap();
    println!("Escuchando en: {host}");

    let (emisor, _receptor) = tokio::sync::broadcast::channel::<(String, String)>(10);

    loop {
        let Ok((tcp_stream, socket_addr)) = tcp_listener.accept().await else {
            continue;
        };

        let nuevo_emisor = emisor.clone();
        let mut nuevo_receptor = emisor.subscribe();

        tokio::spawn(async move {
            let (lector, mut escritor) = tcp_stream.into_split();
            let mut lineas_lector = BufReader::new(lector);
            let mut linea = String::new();
            let id_cliente = socket_addr.to_string();

            let _ = nuevo_emisor.send((
                "Sistema".to_string(),
                format!("{id_cliente} se ha unido al chat\n"),
            ));

            loop {
                tokio::select! {
                resultado = lineas_lector.read_line(&mut linea) => {
                    match resultado {
                      Ok(0) => break,
                    Ok(_) => {
                        let _ = nuevo_emisor.send((id_cliente.clone(), linea.clone()));
                        linea.clear();
                    }
                    Err(_) => break,}
                }
                resultado = nuevo_receptor.recv() => {
                    // Ok(0) => break,
                    match resultado {
                      Ok((remitente, msg)) => {
                        if remitente != id_cliente {
                            if escritor
                                .write_all(format!("[{remitente}]: {msg}").as_bytes())
                                .await
                                .is_err()
                            {
                                break;
                            }
                        }
                    }
                    Err(_) => break,}
                }
                }
            }
            let _ = nuevo_emisor.send((
                "Sistema".to_string(),
                format!("{id_cliente} ha abandonado al chat\n"),
            ));
        });
    }
}
