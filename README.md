# CHIP-8 Emulator
Un émulateur **CHIP-8** écrit en [Rust](https://www.rust-lang.org/).<br>
Utilise la [SDL](https://www.libsdl.org/) et [OpenGL](https://en.wikipedia.org/wiki/OpenGL) pour effectuer le rendu graphique.

## Table des matières
- [Installation](#installation)
- [Compilation](#compilation)
- [Utilisation](#utilisation)

## Installation
Actuellement, seule la version **Windows** peut être téléchargée directement depuis la page [Release](https://github.com/Tytraman/CHIP-8-Emulator/releases/latest).

Sous **Linux**, il est plus simple de compiler soi-même l'émulateur, voir [Compilation](#compilation).

## Compilation
### Windows
TODO: Écrire cette section.
### Linux
```
$ cargo build --release
```
## Utilisation
Les roms **CHIP-8** à émuler doivent se situer dans le dossier `./Builtin/Programs` et doivent avoir comme extension `.ch8`.


```
$ ./chip-8-main --program <rom>
```
Par exemple :
```
$ ./chip-8-main --program tetris
```
