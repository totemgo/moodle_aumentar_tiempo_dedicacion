use super::CursoMoodle;
use rand::{Rng, SeedableRng};
use reqwest::Client;

use crate::utilidades;

pub async fn simular_clicks(
    c: CursoMoodle,
    cliente: &Client,
    intervalo_inferior: u16,
    intervalo_superior: u16,
) {
    loop {
        let mut rango = rand::rngs::StdRng::from_entropy();
        let indice_azar = rango.gen_range(0..c.enlaces.len());
        let enlace_destino = &c.enlaces[indice_azar];

        match cliente.get(enlace_destino).send().await {
            Err(error) => {
                let mensaje_error = format!(
                    "ATENCIÓN: ha fallado el intento de click en {}: {}  {}",
                    c.nombre, enlace_destino, error
                );
                eprintln!("{}", mensaje_error);
            }
            Ok(ok) => {
                if ok.status() == 200 {
                    println!(
                        "{} ##### {} ##### {}",
                        utilidades::ahora(),
                        c.nombre,
                        enlace_destino
                    );
                }
            }
        }

        let espera_azar: u64 =
            (rango.gen_range(intervalo_inferior..intervalo_superior) * 60).into();
        tokio::time::sleep(std::time::Duration::from_secs(espera_azar)).await;
    }
}
