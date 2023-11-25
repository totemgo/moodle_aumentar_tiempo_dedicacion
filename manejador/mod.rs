use crate::argumentos;
use futures::future;
use reqwest::Client;

mod analizar_archivo_origen;
mod cliente_http;
mod gestionar_clicks;
mod identificar;
mod obtener_recursos;

pub struct Manejador {
    pub cliente: Client,
    pub intervalo_clicks_inferior: u16,
    pub intervalor_clicks_superior: u16,
    pub cursos: Vec<CursoMoodle>,
}
#[derive(Clone)]
pub struct CursoMoodle {
    pub nombre: String,
    pub enlaces: Vec<String>,
}

impl Manejador {
    pub async fn new(argumentos_cli: argumentos::Argumentos) -> Manejador {
        let cliente = cliente_http::crear_cliente_http();
        let (hostname, cursos) =
            analizar_archivo_origen::analizar_archivo_origen(argumentos_cli.ruta);
        identificar::identificar_usuario_moodle(
            &hostname,
            argumentos_cli.usuario,
            argumentos_cli.password,
            &cliente,
        )
        .await;

        let mut futuros_busqueda_enlaces = Vec::new();
        for c in cursos {
            futuros_busqueda_enlaces.push(obtener_recursos::buscar_enlaces(&hostname, c, &cliente));
        }
        let informacion_cursos = future::join_all(futuros_busqueda_enlaces).await;
        let mut manejador = Manejador {
            cliente,
            intervalo_clicks_inferior: argumentos_cli.inferior,
            intervalor_clicks_superior: argumentos_cli.superior,
            cursos: Vec::new(),
        };
        for (nombre, enlaces) in informacion_cursos {
            let curso_moodle = CursoMoodle { nombre, enlaces };
            manejador.cursos.push(curso_moodle);
        }

        return manejador;
    }
    pub fn resumir_estado(&self) {
        println!("INFO: usuario identificado correctamente. El cliente http dispone de las cookies necesarias para realizar peticiones verificadas");
        for c in &self.cursos {
            let mensaje_resumen = format!(
                "INFO: para el curso {} se alternarán visitas al azar en las siguientes páginas {}",
                c.nombre,
                c.enlaces.join(", ")
            );
            println!("{}", mensaje_resumen);
        }
        println!("\n\nINFO: comienzan las visitas automáticas con el formato Hora ### Curso ### Página\n\n")
    }
    pub async fn disparador_clicks(&self) {
        let mut coleccion_hilos = Vec::new();
        for c in &self.cursos {
            let cliente_clone = self.cliente.clone();
            let curso_clone = c.clone();
            let intervalo_inferior = self.intervalo_clicks_inferior.clone();
            let intervalor_superior = self.intervalor_clicks_superior.clone();

            let manejador_hilo = tokio::spawn(async move {
                gestionar_clicks::simular_clicks(
                    curso_clone,
                    &cliente_clone,
                    intervalo_inferior,
                    intervalor_superior,
                )
                .await;
            });
            coleccion_hilos.push(manejador_hilo);
        }

        for hilo in coleccion_hilos {
            hilo.await.expect(
                "ERROR FATAL: producido en la espera de la finalización de una tarea en un hilo",
            );
        }
    }
}
