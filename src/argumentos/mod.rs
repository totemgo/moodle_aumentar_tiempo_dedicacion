use std::path::PathBuf;

use clap::Parser;

/// Automatización de visitas en recursos de cursos de Moodle para aumentar el tiempo de dedicación
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Argumentos {
    /// usuario de Moodle
    #[arg(short, long)]
    pub usuario: String,

    /// password de Moodle
    #[arg(short, long)]
    pub password: String,

    /// número de minutos para el valor inferior del intervalo de tiempo aleatorio en el que se realizarán las visitas
    #[arg(short, long, default_value_t = 3)]
    pub inferior: u16,

    /// número de minutos para el valor máximo del intervalo de tiempo aleatorio en el que se realizarán las visitas
    #[arg(short, long, default_value_t = 8)]
    pub superior: u16,

    /// ruta del archivo en el que se recogen los cursos de Moodle en los que se desea aumentar la dedicación
    #[arg(short, long, default_value = "cursos.txt")]
    pub ruta: PathBuf,
}

pub fn parsear_argumentos() -> Argumentos {
    return Argumentos::parse();
}
