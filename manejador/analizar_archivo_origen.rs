use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::PathBuf,
};
use url::Url;

fn validar_url_hostname_moodle(hostname: &mut String, url: &mut String) -> Result<(), String> {
    if !url.contains("/course/") {
        return Err("la url introducida debe contener /course/ y hacer referencia a la página principal de un curso".to_string());
    };

    let url_parseada;

    match Url::parse(&url) {
        Err(error) => {
            let mensaje_error = format!("ha fallado el parseado de la URL introducida {}", error);
            return Err(mensaje_error);
        }
        Ok(ok) => url_parseada = ok,
    }

    let hostname_url;
    match url_parseada.host() {
        None => {
            let mensaje_error =
                format!("no se ha podido obtener el hostname de la URL introducida");
            return Err(mensaje_error);
        }
        Some(host) => hostname_url = host.to_string(),
    }

    if hostname == "" {
        *hostname = hostname_url.clone();
    }

    if *hostname != hostname_url {
        let mensaje_error = format!("el hostname es diferente al esperado. Solo puede haber un hostname por cada aplicación ejecutada.");
        return Err(mensaje_error);
    }

    let parametros_url = url_parseada.query_pairs();
    let mut curso_id = "".to_string();
    for (c, v) in parametros_url {
        if c == "id" {
            curso_id = v.to_string()
        }
    }
    if curso_id == "" {
        return Err("no se ha encontrado el identificador del curso".to_string());
    }

    *url = curso_id;
    return Ok(());
}

pub fn analizar_archivo_origen(ruta: PathBuf) -> (String, Vec<String>) {
    let archivo = File::open(ruta).expect("ERROR FATAL: no se ha encontrado el archivo con los cursos de Moodle. Se puede pasar con el argumento -r o --ruta. En caso contrario, buscará cursos.txt en la misma ruta del ejecutable");
    let buffer = BufReader::new(archivo);

    let mut hostname = "".to_string();
    let mut cursos_id = Vec::new();
    for (indice, linea) in buffer.lines().enumerate() {
        let mut url;
        match linea {
            Err(error) => {
                let mensaje_error =
                    format!("ERROR: LÍNEA {}: imposible de leer {}", indice + 1, error);
                eprintln!("{}", mensaje_error);
                continue;
            }
            Ok(ok) => url = ok,
        }

        match validar_url_hostname_moodle(&mut hostname, &mut url) {
            Err(error) => {
                let mensaje_error = format!("ERROR: LÍNEA {} {}", indice + 1, error);
                eprintln!("{}", mensaje_error);
                continue;
            }
            Ok(_) => {
                // url ya se ha transformado en el id del curso
                if cursos_id.contains(&url) {
                    let mensaje_error = format!(
                        "ERROR: LÍNEA {} hace referencia a un curso ya incorporado",
                        indice + 1,
                    );
                    eprintln!("{}", mensaje_error);
                    continue;
                }
                cursos_id.push(url);
            }
        }
    }
    if cursos_id.len() == 0 {
        panic!("ERROR FATAL: no se han encontrado URLs que analizar")
    }
    return (hostname, cursos_id);
}
