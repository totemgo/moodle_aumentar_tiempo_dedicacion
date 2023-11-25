use core::panic;
use reqwest::Client;
use scraper::{Html, Selector};
use std::{collections::HashMap, process::exit};
pub async fn identificar_usuario_moodle(
    hostname: &str,
    usuario: String,
    pass: String,
    cliente: &Client,
) {
    let login_url = format!("https://{hostname}/login/index.php");
    let mut login_token = "".to_string();

    {
        let get_login = cliente
            .get(&login_url)
            .send()
            .await
            .expect("ERROR FATAL: no se ha podido realizar la petición a la página del login");

        if get_login.status() != 200 {
            panic!("ERROR FATAL: la petición para obtener la página del login ha devuelto un status code incorrecto: {}", get_login.status())
        }

        let get_login_html = get_login.text().await.expect("ERROR FATAL: no se ha podido obtener el contenido de la respuesta a la petición de la página del login");

        let get_login_scraper = Html::parse_document(&get_login_html);
        let selector_login_token = Selector::parse("input[name=\"logintoken\"]")
            .expect("ERROR FATAL: no se ha podido crear el selector input[name=\"logintoken\"]");

        for c in get_login_scraper.select(&selector_login_token) {
            match c.value().attr("value") {
                None => {
                    panic!("ERROR FATAL: no se ha encontrado el login_token en la página de login")
                }
                Some(ok) => {
                    login_token += &ok;
                    break;
                }
            }
        }
    }

    let mut login_parametros: HashMap<&str, &str> = HashMap::new();
    login_parametros.insert("anchor", "&");
    login_parametros.insert("logintoken", &login_token);
    login_parametros.insert("username", &usuario);
    login_parametros.insert("password", &pass);

    let post_login = cliente
        .post(&login_url)
        .form(&login_parametros)
        .send()
        .await
        .expect("ERROR FATAL: no se ha podido realizar el post de login");

    if post_login.status() != 200 {
        panic!(
            "ERROR FATAL: el post para el login ha obtenido un status code incorrecto {}",
            post_login.status()
        )
    }

    let post_login_html = post_login
        .text()
        .await
        .expect("ERROR FATAL: no se ha podido obtener la respuesta tras el post de login");

    let post_login_scraper = Html::parse_document(&post_login_html);
    let select_form_login = Selector::parse("form#login")
        .expect("ERROR FATAL: no se ha podido crear el selector de form form#login");

    let mut coincidencias = 0;
    for _ in post_login_scraper.select(&select_form_login) {
        coincidencias += 1;
    }

    if coincidencias > 0 {
        eprintln!(
            "ERROR FATAL: ha fallado la autentificación con el usuario y password proporcionados"
        );
        exit(1);
    }
}
