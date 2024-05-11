# RCRYPTO

Algoritmos criptograficos con Rust

## Requisitos

- [Rust](https://www.rust-lang.org/tools/install)

## Compilación 

### Solo para Windows
Descargar el toolchain correcto
```bash
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

### Compilar

Dentro de la carpeta raíz del proyecto ejecutar el siguiente comando

```bash
cargo build --release
```

El ejecutable se encuentra en la carpeta **/target/release**

```bash
cd target/release
.\crypto.exe --help
```

En Linux se reemplaza el **.exe** 

```bash
cd ./target/release
./crypto --help
```

## Uso
La sintaxis basica es la siguiente
```bash
.\crypto.exe [-e|-d] <ALGORITMO> <DATA> <LLAVE>
```
Se indica la operacion **-e** para encriptar o **-d** para desencriptar. \\
El ALGORITMO puede ser uno de los siguientes 
|Algoritmo|Clave|
|---------|------|
|caesar|Numero Entero|
|spartan|Numero Entero|
|vigenere|Cadena de texto|

> [!NOTE]
> La clave para vigenere y la DATA solo pueden contener caracteres entre A-Z y espacios

### Ejemplos
Para encriptar "Este es un mensaje secreto" con el algoritmo caesar y la clave 10

```bash
.\crypto.exe -e caesar "Este es un mensaje secreto" 10
```
El resultado sera el mensaje cifrado. Para desencriptar la el mensaje cifrado se cambia la opcion **-e** a **-d** y la DATApor el mensaje cifrado.

```bash
.\crypto.exe -d caesar "obcojobjdxjwoxbktojbomaocy" 10
```

El resultado muestra por consola el mensaje descifrado **Este es un mensaje secreto**



