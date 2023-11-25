mod argumentos;
mod manejador;
mod utilidades;

#[tokio::main]
async fn main() {
    let argumentos_cli = argumentos::parsear_argumentos();
    let manejador = manejador::Manejador::new(argumentos_cli).await;
    manejador.resumir_estado();
    manejador.disparador_clicks().await;
}
