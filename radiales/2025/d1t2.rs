use std::fs::File;
use std::io::{ BufRead, BufReader};

fn main() {
    let file = File::open("d1.txt").expect("Datei konnte nicht geöffnet werden");
    let reader = BufReader::new(file);
    let mut position = 50;
    let mut zero_counter = 0;

    for line in reader.lines() {
        // Jede Zeile besteht aus einem Buchstaben (L oder R) und einer Zahl.
        // L: Drehe das Rad nach links (subtrahiere die Zahl von der aktuellen Position).
        // R: Drehe das Rad nach rechts (addiere die Zahl zur aktuellen Position).
        let line = line.expect("Fehler beim Lesen der Zeile");
        let first_char = line.chars().next();
        let number_str = &line[1..]; // Alles nach dem ersten Zeichen
        let mut number: i32 = number_str.trim().parse().expect("Keine gültige Zahl");

        match first_char {
            Some('L') => {
                // Links drehen: Position dekrementieren und mit Modulo 100 im Bereich halten
                while number > 0 {
                    // Bei position = 0: (0 + 99) % 100 = 99 ist man somit wieder am ende vom Raf
                    position = (position + 99) % 100;
                    number -= 1;
                    if position == 0 {
                        zero_counter += 1;
                    }
                }
            }
            Some('R') => {
                // Rechts drehen: Position inkrementieren und mit Modulo 100 im Bereich halten
                while number > 0 {
                    position = (position + 1) % 100;
                    number -= 1;
                    if position == 0 {
                        zero_counter += 1;
                    }
                }
            }
            _ => {
                // Fehlerbehandlung
            }
        }

    }

    println!("Nullen erfasst {}", zero_counter);
}