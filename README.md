# AUMENTAR TIEMPO DE DEDICACIÓN EN MOODLE
Script para aumentar el tiempo de dedicación en cursos de Moodle.

```bash
utomatización de visitas en recursos de cursos de Moodle para aumentar el tiempo de dedicación

Usage: moodle_aumentar_tiempo_dedicacion [OPTIONS] --usuario <USUARIO> --password <PASSWORD>

Options:
  -u, --usuario <USUARIO>    usuario de Moodle
  -p, --password <PASSWORD>  password de Moodle
  -i, --inferior <INFERIOR>  número de minutos para el valor inferior del intervalo de tiempo aleatorio en el que se realizarán las visitas [default: 3]
  -s, --superior <SUPERIOR>  número de minutos para el valor máximo del intervalo de tiempo aleatorio en el que se realizarán las visitas [default: 8]
  -r, --ruta <RUTA>          ruta absoluta del archivo en el que se recogen los cursos de Moodle en los que se desea aumentar la dedicación [default: cursos.txt]
  -h, --help                 Print help
  -V, --version              Print version
```

## Información y uso
El script puede aumentar la dedicación en distintos cursos. Para ello debe facilitarse un archivo de .txt con cada curso en una línea
```bash
https://school.moodledemo.net/course/view.php?id=69
https://school.moodledemo.net/course/view.php?id=70
https://school.moodledemo.net/course/view.php?id=71
```
El script visitará la página principal del curso y recopilará todos los enlaces de las pantallas principales de los recursos, sin profundizar más (eso significa que, por ejemplo, en un recurso de tipo "Foro" no entrará en hilos que no estén en la pantalla principal).

Las visitas y el tiempo de espera se hacen de manera aleatoria.
