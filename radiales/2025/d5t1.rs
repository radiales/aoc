use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Datei öffnen und Reader vorbereiten
    let datei = File::open("d5.txt").expect("Datei konnte nicht geöffnet werden");
    let reader = BufReader::new(datei);

    // Alle Ranges oben sammeln und die Zahlen unten in eine Queue packen
    // i64 nutzen, weil die Eingabezahlen sehr groß sein können
    let mut bereichsliste: Vec<(i64, i64)> = Vec::new();
    let mut zahlen: VecDeque<i64> = VecDeque::new();
    let mut oberer_teil = true;

    for zeilen_ergebnis in reader.lines() {
        let zeile = zeilen_ergebnis.expect("Zeile konnte nicht gelesen werden");
        let zugeschnitten = zeile.trim();

        if zugeschnitten.is_empty() {
            oberer_teil = false;
            continue;
        }

        if oberer_teil {
            let (start, ende) = zugeschnitten
                .split_once('-')
                .expect("Range ohne '-' gefunden");
            let start: i64 = start.trim().parse().expect("Start konnte nicht geparst werden");
            let ende: i64 = ende.trim().parse().expect("Ende konnte nicht geparst werden");
            bereichsliste.push((start, ende));
        } else {
            let zahl: i64 = zugeschnitten.parse().expect("Zahl konnte nicht geparst werden");
            zahlen.push_back(zahl);
        }
    }

    // Zahlen nacheinander prüfen und bei Treffer frisch erhöhen
    let mut frisch: usize = 0;
    while let Some(zahl) = zahlen.pop_front() {
        let in_bereich = bereichsliste
            .iter()
            .any(|(start, ende)| zahl >= *start && zahl <= *ende);

        if in_bereich {
            frisch += 1;
        }
        // Zahl wird immer entfernt, egal ob sie in einer Range lag oder nicht
    }

    println!("frisch: {}", frisch);
}
