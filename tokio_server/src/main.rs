use crate::ejercicios::chat_grupal;

mod ejercicios;

#[tokio::main]
async fn main() {
  chat_grupal::app().await;
}
