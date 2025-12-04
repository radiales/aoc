use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("d3.txt").expect("Datei konnte nicht geöffnet werden");
    let reader = BufReader::new(file);
    let mut ergebnis: i32 = 0;

    for line in reader.lines() {
        let line = line.expect("Fehler beim Lesen der Zeile");
        println!("Verarbeite Zeile: {}", line);

        // Zeile in Ziffern umwandeln
        let numbers: Vec<i32> = line.chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as i32))
            .collect();

        // Debug-Ausgabe
        println!("Numbers: {:?}", numbers);

        // Größtmögliche zweistellige Zahl finden, Reihenfolge beachten!
        let mut best = 0;

        for i in 0..numbers.len() {
            for j in (i + 1)..numbers.len() {
                let val = numbers[i] * 10 + numbers[j]; // erste Ziffer = i, zweite = j
                if val > best {
                    best = val;
                }
            }
        }

        // Debug-Ausgabe des Ergebnisses für diese Zeile
        println!("Max für Zeile '{}': {}", line, best);

        ergebnis += best;
    }

    println!("Ergebnis: {}", ergebnis);
}
