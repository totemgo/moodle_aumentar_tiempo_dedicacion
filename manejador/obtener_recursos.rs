use reqwest::Client;
use scraper::{Html, Selector};

pub async fn buscar_enlaces(hostname: &str, id: String, cliente: &Client) -> (String, Vec<String>) {
    let mut nombre_curso = "".to_string();
    let mut recursos_encontrados = vec![
        format!("https://{}/course/view.php?id={}", hostname, id),
        format!("https://{}/user/index.php?id={}", hostname, id),
        format!(
            "https://{}/grade/report/grader/index.php?id={}",
            hostname, id
        ),
    ];

    let respuesta;
    match cliente.get(&recursos_encontrados[0]).send().await {
        Err(error) => {
            let mensaje_error = format!("ADVERTENCIA: ha fallado la petición a la página principal del curso {}:{}. Esto significa que únicamente se intentará visitar la página principal, de participantes y de calificaciones", id, error);
            eprintln!("{}", mensaje_error);
            return (nombre_curso, recursos_encontrados);
        }
        Ok(ok) => respuesta = ok,
    }
    let respuesta_html;
    match respuesta.text().await {
        Err(error) => {
            let mensaje_error = format!("ADVERTENCIA: ha fallado la lectura del html de la página principal del curso {}:{}. Esto significa que únicamente que se intentará visitar la página principal, de participantes y de calificaciones", id, error);
            eprintln!("{}", mensaje_error);
            return (nombre_curso, recursos_encontrados);
        }
        Ok(ok) => respuesta_html = ok,
    }
    let respuesta_scraper = Html::parse_document(&respuesta_html);
    let selector_titulo =
        Selector::parse("title").expect("ERROR FATAL: no se ha podido crear el selector de title");
    let titulo = respuesta_scraper.select(&selector_titulo);
    for t in titulo {
        nombre_curso = t.inner_html();
        break;
    }

    let selector_recursos = Selector::parse(".activityname a").unwrap();
    for a in respuesta_scraper.select(&selector_recursos) {
        match a.attr("href") {
            None => (),
            Some(href) => {
                recursos_encontrados.push(href.to_string());
            }
        }
    }

    return (nombre_curso, recursos_encontrados);
}
