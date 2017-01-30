# Spezielle Pointer in Rust

---

Rust bietet mehrere spezielle Pointer-Typen an, um verschiedene Szenarien zu handhaben.

Generell ist ihnen gemein: sie werden über Ownership gemanagt.

---

## std::rc::Rc<T>

Laufzeit-Reference-Counter innerhalb eines Threads.

```rust
use std::rc::Rc;

struct Point {
   x: i32,
   y: i32,
}

fn main() {
    let rced_point = Rc::new(Point { x: 1, y: 1});
    let first_handle = rced_point.clone();
    let second_handle = rced_point.clone();
}
```

---

## Semantik

* Rc ist ein Handle auf die enthaltenen Daten
* Das Handle kann geklont werden
* Wenn das letzte Handle droppt, droppen die Daten mit
* Rc<T> implementiert Deref<Target=T>

---

## std::rc::Weak<T>

Schwacher Pointer auf Daten.

```rust
use std::rc::Rc;

struct Point {
   x: i32,
   y: i32,
}

fn main() {
    let rced_point = Rc::new(Point { x: 1, y: 1});
    let first_handle = rced_point.clone();
    let weak = Rc::downgrade(first_handle);
}
```

---

## Semantik

* Ähnlich Rc, die Existenz der Daten ist aber nicht garantiert
* Single-Threaded: Die Daten sind garantiert über die Zeit einer Operation verfügbar
* Wird _nicht_ automatisch dereferenziert
* Rc-Kreise sind Speicherlecks, Weak-Pointer verhindern das

---

## Nutzen

* Häufig in Datenstrukturen gebraucht, die komplexe Querreferenzierungen brauchen
* Höhere Laufzeitkosten für mehr Flexibilität

---

## std::sync::Arc

Ein teurer `Rc`, der über Threadgrenzen hinweg funktioniert, da ein atomarer Zähler
zum Inkrementieren verwendet wird.

---

## Anmerkung

Nutzen Sie `Arc` nicht "auf Verdacht" - `rustc` lehnt Code ab, der `Rc` über Threadgrenzen verwendet.

---

## std::borrow::Cow

Kuhverleih. Standard. Auffem Dorf.

---



