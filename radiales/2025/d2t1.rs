use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("d2.txt").expect("Datei konnte nicht geöffnet werden");
    let reader = BufReader::new(file);
    let mut answer: i64 = 0;

    // Etwas overkill da es nur eine line gibt aber who cares
    for line in reader.lines() {
        let line = line.expect("Fehler beim Lesen der Zeile");
        let pairs: Vec<&str> = line.split(',').collect();
        

        for pair in pairs {
            // Prefix ist hier die erste Zahl, Postfix die zweite Zahl
            if let Some((prefix, postfix)) = pair.split_once('-') {
                let prefix = prefix.parse::<i64>().expect("Fehler beim Parsen von Prefix");
                let postfix = postfix.parse::<i64>().expect("Fehler beim Parsen von Postfix");

                // Checkt alle werte im Bereich von Prefix bis Postfix
                for value_to_check in prefix..=postfix {
                    let value_str = value_to_check.to_string();
                    let len = value_str.len();

                    // Wir splitten die Zahl in der Mitte und vergleichen die Hälften
                    if len % 2 == 0 {
                        let (left, right) = value_str.split_at(len / 2);

                        // Wenn beide Hälften gleich sind, wird die Zahl zum Answer addiert
                        if left == right {
                            answer += value_to_check;
                        }


                    } else {
                        // println!("Die Zahl {} hat eine ungerade Anzahl an Ziffern ist somit irrelevant für die Aufgabe", value_to_check);
                    }
                }
            }
        }
    }

    println!("Alle invaliden IDs addiert ergeben {}", answer)
}