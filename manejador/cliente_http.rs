use reqwest::Client;
use std::time::Duration;

pub fn crear_cliente_http() -> Client {
    return reqwest::ClientBuilder::new()
        .cookie_store(true)
        .timeout(Duration::new(12, 0))
        .build()
        .expect("ERROR FATAL: no se ha podido crear el cliente http");
}
