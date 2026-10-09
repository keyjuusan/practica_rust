use std::time::Duration;

pub async fn app() {

    println!("Esperando a cocinar...");
    // forma 2
    // let _alarma_huevo = tokio::spawn(fn_alarma_huevo());
    // let _alarma_bizcocho = tokio::spawn(fn_alarma_bizcocho());

    // let _ = _alarma_bizcocho.await ;
    // let _ = _alarma_huevo.await;

    // forma 1
    tokio::join!(fn_alarma_bizcocho(),fn_alarma_huevo());

    // cual es la diferencia entre hacerlo directo en la misma line del spawn y hacerlo con el await despues?
    // porque se vuelve sincrono

}

async fn fn_alarma_huevo(){
  let duration = Duration::from_secs(2);
  tokio::time::sleep(duration).await;
  println!("El huevo esta cocido!")
}

async fn fn_alarma_bizcocho(){
  let duration = Duration::from_secs(4);
  tokio::time::sleep(duration).await;
  println!("El bizcocho esta cocido!")
}