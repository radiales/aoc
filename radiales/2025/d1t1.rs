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
        // Das Rad hat 100 Positionen (0 bis 99). Wenn die Position < 0 oder > 99 wird sie mit Modulo 100 korrekt gesetzt.
        // Nach jeder Bewegung wird geprüft, ob die aktuelle Position 0 ist und der Zähler erhöht.
        let line = line.expect("Fehler beim Lesen der Zeile");
        let first_char = line.chars().next();
        let number_str = &line[1..]; // Alles nach dem ersten Zeichen
        let number: i32 = number_str.trim().parse().expect("Keine gültige Zahl");

        match first_char {
            Some('L') => {
                position = (position - number + 100) % 100;
            }
            Some('R') => {
                position = (position + number) % 100;
            }
            _ => {
                // Fehlerbehandlung
            }
        }

        if position == 0 {
            zero_counter += 1;
        }
    }

    println!("Nullen erfasst {}", zero_counter);
}