# RESTAPI with Rust and Rocket

ApiRest escrita en Rust usando el framework Rocket.

Rocket necesita de la version `nightly` de Rust para funcionar.

Si ya tienes instalado Rust debes ejecutar en siguiente comando para descargar la versión `nightly`

````bash
rustup toolchain install nightly
````

Luego para poner como default la version `nightly`:

````bash
rustup default nigthly
````

La version `nightly` de Rust es su ultima version, podría llegar a ser inestable.

Para ejecutar el proyecto:

```bash
make local_run
```

Este proyecto tiene un [`Makefile`](https://hernandis.me/2017/03/20/como-hacer-un-makefile.html)

GNU make es una utilidad disponible en Linux que agiliza la tarea de compilar código desde la terminal. Nos evita tener que escribir los comandos de compilación a mano, que suelen ser muy largos, y en cambio nos permite escribir algo mucho más corto que al final hace lo mismo.

![ferris](https://i.redd.it/vp0a1tf4jc911.png)
