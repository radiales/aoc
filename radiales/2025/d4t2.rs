use std::fs;

fn main() {
    // Datei einlesen
    let eingabe = fs::read_to_string("d4.txt").expect("Datei konnte nicht geöffnet werden");

    // Eingabe in eine 2D-Matrix umwandeln
    let mut gitter: Vec<Vec<char>> = eingabe
        .lines()
        .map(|zeile| zeile.chars().collect())
        .collect();

    let zeilen = gitter.len(); // Anzahl der Zeilen
    let spalten = gitter[0].len(); // Anzahl der Spalten
    let mut answer = 0; // Gesamtanzahl entfernter Papierrollen

    loop {
        let mut entfernt_in_dieser_runde = 0; // Anzahl entfernter Rollen in dieser Runde

        // Durch das Gitter iterieren
        let mut zu_entfernen = vec![];
        for i in 0..zeilen {
            for j in 0..spalten {
                if gitter[i][j] == '@' {
                    let mut nachbarn_anzahl = 0; // Zähler für benachbarte Papierrollen

                    // Alle 8 Nachbarn überprüfen
                    for di in -1..=1 {
                        for dj in -1..=1 {
                            if di == 0 && dj == 0 {
                                continue; // Überspringe die aktuelle Position
                            }

                            let ni = i as isize + di;
                            let nj = j as isize + dj;

                            // Überprüfen, ob der Nachbar innerhalb der Grenzen liegt
                            if ni >= 0 && ni < zeilen as isize && nj >= 0 && nj < spalten as isize {
                                if gitter[ni as usize][nj as usize] == '@' {
                                    nachbarn_anzahl += 1;
                                }
                            }
                        }
                    }

                    // Überprüfen, ob die Papierrolle zugänglich ist
                    if nachbarn_anzahl < 4 {
                        zu_entfernen.push((i, j));
                    }
                }
            }
        }

        // Entferne die markierten Rollen
        for (i, j) in zu_entfernen {
            gitter[i][j] = '.'; // Rolle entfernen
            entfernt_in_dieser_runde += 1;
        }

        // Aktualisiere die Gesamtanzahl
        answer += entfernt_in_dieser_runde;

        // Wenn keine Rollen mehr entfernt werden können, beende die Schleife
        if entfernt_in_dieser_runde == 0 {
            break;
        }
    }

    println!("Gesamt entfernte Papierrollen: {}", answer);
}