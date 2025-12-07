use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn timelines_ab(zeile: usize, spalte: usize, raster: &[Vec<char>], hoehe: usize, breite: usize, memo: &mut HashMap<(usize, usize), i64>) -> i64 {
    if let Some(&wert) = memo.get(&(zeile, spalte)) {
        return wert;
    }

    let mut r = zeile;
    while r < hoehe {
        let ch = raster[r][spalte];
        if ch == '^' {
            let nzeile = r + 1;
            // Links-Zweig: wenn aus dem Feld raus, endet der Pfad direkt
            let links = if spalte == 0 || nzeile >= hoehe {
                1
            } else {
                timelines_ab(nzeile, spalte - 1, raster, hoehe, breite, memo)
            };
            // Rechts-Zweig
            let rechts = if spalte + 1 >= breite || nzeile >= hoehe {
                1
            } else {
                timelines_ab(nzeile, spalte + 1, raster, hoehe, breite, memo)
            };
            let wert = links + rechts;
            memo.insert((zeile, spalte), wert);
            return wert;
        }
        r += 1;
    }

    // Kein weiterer Splitter: genau eine Timeline
    memo.insert((zeile, spalte), 1);
    1
}

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

    // Einheitliche Breite auffüllen
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

    // Beam startet unterhalb von S
    if start_zeile + 1 >= hoehe {
        println!("Anzahl Timelines: 1");
        return;
    }

    let mut memo: HashMap<(usize, usize), i64> = HashMap::new();
    let timelines = timelines_ab(start_zeile + 1, start_spalte, &raster, hoehe, breite, &mut memo);

    println!("Anzahl Timelines: {}", timelines);
}
