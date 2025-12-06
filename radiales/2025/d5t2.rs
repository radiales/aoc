use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Datei öffnen und Reader vorbereiten
    let datei = File::open("d5.txt").expect("Datei konnte nicht geöffnet werden");
    let reader = BufReader::new(datei);

    // Alle Ranges oben sammeln; i64 nutzen, weil die Eingabezahlen sehr groß sein können
    let mut bereichsliste: Vec<(i64, i64)> = Vec::new();
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
            // Unterer Teil ist für Teil 2 irrelevant
        }
    }

    // Ranges zusammenfassen, damit Überlappungen nicht doppelt gezählt werden
    bereichsliste.sort_by_key(|(start, _)| *start);
    let mut zusammengelegt: Vec<(i64, i64)> = Vec::new();

    for (start, ende) in bereichsliste {
        if let Some((letzter_start, letzter_ende)) = zusammengelegt.last_mut() {
            if start <= *letzter_ende + 1 {
                // Überlappung oder direkt angrenzend: Range erweitern
                *letzter_ende = (*letzter_ende).max(ende);
                continue;
            }
        }
        zusammengelegt.push((start, ende));
    }

    // Länge aller zusammengelegten Ranges aufsummieren
    let mut anzahl_frisch = 0i64;
    for (start, ende) in &zusammengelegt {
        let laenge = ende - start + 1;
        anzahl_frisch += laenge;
    }

    println!("Anzahl FrischeIDs: {}", anzahl_frisch);

}
