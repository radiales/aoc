use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Datei laden
    let datei = File::open("d6.txt").expect("Datei konnte nicht geöffnet werden");
    let reader = BufReader::new(datei);

    // Alle Zeilen einlesen
    let mut zeilen: Vec<String> = Vec::new();
    for zeile in reader.lines() {
        zeilen.push(zeile.expect("Zeile konnte nicht gelesen werden"));
    }

    if zeilen.is_empty() {
        println!("Keine Zeilen gefunden");
        return;
    }

    // Alle Zeilen auf gleiche Breite mit Leerzeichen auffüllen
    let breite = zeilen.iter().map(|z| z.len()).max().unwrap_or(0);
    let mut raster: Vec<Vec<char>> = zeilen
        .into_iter()
        .map(|mut z| {
            if z.len() < breite {
                z.push_str(&" ".repeat(breite - z.len()));
            }
            z.chars().collect()
        })
        .collect();

    let zeilen_anzahl = raster.len();
    let operator_zeile_index = zeilen_anzahl - 1;

    // Spalten finden, die komplett leer sind (Trenner)
    let mut ist_trenner = vec![false; breite];
    for spalte in 0..breite {
        let leer = raster.iter().all(|zeile| zeile[spalte] == ' ');
        ist_trenner[spalte] = leer;
    }

    // Blöcke (Aufgaben) finden: Bereiche aus nicht-trennenden Spalten
    let mut bloecke: Vec<(usize, usize)> = Vec::new();
    let mut spalte = 0;
    while spalte < breite {
        if ist_trenner[spalte] {
            spalte += 1;
            continue;
        }
        let start = spalte;
        while spalte < breite && !ist_trenner[spalte] {
            spalte += 1;
        }
        let ende = spalte - 1;
        bloecke.push((start, ende));
    }

    // Jede Aufgabe auswerten
    let mut gesamt_summe: i64 = 0;

    for (start, ende) in bloecke {
        // Zahlen einsammeln aus allen Zeilen außer der letzten (Operatorzeile)
        let mut zahlen: Vec<i64> = Vec::new();
        for zeile_idx in 0..operator_zeile_index {
            let teil: String = raster[zeile_idx][start..=ende].iter().collect();
            let zugeschnitten = teil.trim();
            if !zugeschnitten.is_empty() {
                let wert: i64 = zugeschnitten
                    .parse()
                    .expect("Zahl konnte nicht geparst werden");
                zahlen.push(wert);
            }
        }

        // Operator aus der letzten Zeile lesen
        let op_text: String = raster[operator_zeile_index][start..=ende].iter().collect();
        let op = op_text.trim();
        let ergebnis: i64 = match op {
            "+" => zahlen.iter().copied().sum(),
            "*" => zahlen.iter().copied().product(),
            _ => panic!("Unbekannter Operator: {}", op),
        };

        gesamt_summe += ergebnis;
    }

    println!("Gesamtsumme: {}", gesamt_summe);
}
