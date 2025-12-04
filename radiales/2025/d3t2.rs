use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    const K: usize = 12; // wir brauchen genau 12 Ziffern pro Bank

    let file = File::open("d3.txt").expect("Datei konnte nicht geöffnet werden");
    let reader = BufReader::new(file);
    let mut ergebnis: i64 = 0; // groß genug für 12-stellige Zahlen

    for line in reader.lines() {
        let line = line.expect("Fehler beim Lesen der Zeile");

        // Zeile in Ziffern umwandeln
        let digits: Vec<u8> = line
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as u8))
            .collect();

        // Sanity-Check: genug Batterien?
        if digits.len() < K {
            panic!(
                "Zu wenig Batterien in Zeile ({} Ziffern, brauche {}): {}",
                digits.len(),
                K,
                line
            );
        }

        // Debug-Ausgabe
        println!("Digits ({}): {:?}", line, digits);

        // Greedy-Algorithmus: größte Teilfolge der Länge K
        let mut stack: Vec<u8> = Vec::with_capacity(digits.len());
        let mut to_remove = digits.len() - K; // so viele dürfen wir entfernen

        for &d in &digits {
            while let Some(&last) = stack.last() {
                if to_remove > 0 && last < d {
                    stack.pop();
                    to_remove -= 1;
                } else {
                    break;
                }
            }
            stack.push(d);
        }

        // Falls noch zu viele Ziffern im Stack sind, auf K kürzen
        stack.truncate(K);

        // Aus den 12 Ziffern die Zahl bauen
        let mut bank_value: i64 = 0;
        for d in &stack {
            bank_value = bank_value * 10 + (*d as i64);
        }

        println!(
            "Max für Zeile '{}': {:?} -> {}",
            line, stack, bank_value
        );

        ergebnis += bank_value;
    }

    println!("Gesamt-Ergebnis (Part 2): {}", ergebnis);
}
