# Installation

Florian Gilcher <florian.gilcher@asquera.de>

---

https://www.rust-lang.org/en-US/install.html

---

## Rustup

Rustup ist das Standard-Tool zum Managen von Rusts compiler-Toolchain.

---

## Wichtige Kommandos

```sh
# Installation einer toolchain
$ rustup install stable
# Auswählen einer standard-toolchain
$ rustup default stable
# Anzeigen der Dokumentation im Browser
$ rustup doc [--std]
# Auflisten unterstützter targets
$ rustup target list
# Hinzufügen eines Targets zum installieren
$ rustup target add <target>
# Auflisten/Hinzufügen einer Komponente
$ rustup component list|add
```

---

# Erste Schritte

Führen Sie folgende Kommandos aus:

```sh
$ rustup component add rust-src
$ rustup component add rust-docs
```

Dies läd die Quelltexte der Standardbibliothek und
die Dokumentation zur Komplettierung und Offlineverwendung runter.

---

## Die Toolchain

* rustc
* cargo
* rustdoc
* rust-(lldb|gdb)
* libcore/libstd

Die genaue Art des Debuggers ist Platformabhängig.

---

## rustc

```sh
$ rustc --help
```

Der Rust-Compiler übernimmt das kompilieren und linken von Rust-Code.

`rustc` ist annähernd komplett in Rust geschrieben.

---

## Ownership

---

## Borrowing

---
