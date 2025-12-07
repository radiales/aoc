use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Matrix laden
    let datei = File::open("d7.txt").expect("Datei konnte nicht geöffnet werden");
    let reader = BufReader::new(datei);

    // Zeilen einlesen
    let mut zeilen: Vec<String> = Vec::new();
    for zeile in reader.lines() {
        zeilen.push(zeile.expect("Zeile konnte nicht gelesen werden"));
    }

    if zeilen.is_empty() {
        println!("Keine Zeilen gefunden");
        return;
    }

    // Einheitliche Breite, damit Spaltenindizes immer gültig sind
    let breite = zeilen.iter().map(|z| z.len()).max().unwrap_or(0);
    let raster: Vec<Vec<char>> = zeilen
        .into_iter()
        .map(|mut z| {
            if z.len() < breite {
                z.push_str(&" ".repeat(breite - z.len()));
            }
            z.chars().collect()
        })
        .collect();

    let hoehe = raster.len();

    // Startposition S finden
    let mut start: Option<(usize, usize)> = None;
    for (r, zeile) in raster.iter().enumerate() {
        if let Some(c) = zeile.iter().position(|&ch| ch == 'S') {
            start = Some((r, c));
            break;
        }
    }
    let (start_zeile, start_spalte) = start.expect("Kein Start 'S' gefunden");

    // Warteschlange für aktive Beams (Beginn unterhalb von S)
    let mut front: VecDeque<(usize, usize)> = VecDeque::new();
    let mut gesehen: HashSet<(usize, usize)> = HashSet::new();
    let mut besucht: HashSet<(usize, usize)> = HashSet::new();
    if start_zeile + 1 < hoehe {
        front.push_back((start_zeile + 1, start_spalte));
        gesehen.insert((start_zeile + 1, start_spalte));
    }

    let mut splits: i64 = 0;

    while let Some((zeile, spalte)) = front.pop_front() {
        let mut r = zeile;
        // Beam läuft nach unten, bis er rausfällt oder einen Splitter trifft
        while r < hoehe {
            // Wenn hier schon ein anderer Beam entlang lief, verschmelzen sie -> stoppen
            if !besucht.insert((r, spalte)) {
                break;
            }

            let ch = raster[r][spalte];
            if ch == '^' {
                splits += 1;
                let naechste_zeile = r + 1;
                // Neuer Beam links
                if naechste_zeile < hoehe && spalte > 0 {
                    let pos = (naechste_zeile, spalte - 1);
                    if gesehen.insert(pos) {
                        front.push_back(pos);
                    }
                }
                // Neuer Beam rechts
                if naechste_zeile < hoehe && spalte + 1 < breite {
                    let pos = (naechste_zeile, spalte + 1);
                    if gesehen.insert(pos) {
                        front.push_back(pos);
                    }
                }
                break;
            }
            r += 1;
        }
        // Wenn r == hoehe: Beam verlässt die Matrix, nichts weiter zu tun
    }

    println!("Anzahl Splits: {}", splits);
}
