use chrono::Local;

pub fn ahora() -> String {
    let momento = Local::now();
    return momento.format("%d/%m/%Y %H:%M:%S").to_string();
}
