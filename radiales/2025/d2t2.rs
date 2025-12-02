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
                    let mut found_pattern = false;

                    // Wir prüfen alle möglichen Teilungen der Zahl
                    for part_size in 1..=len / 2 {
                        if len % part_size == 0 {
                            let chunk = &value_str[0..part_size];
                            let repeated = chunk.repeat(len / part_size);

                            // Überprüfen, ob die Zahl aus wiederholten Mustern besteht
                            if repeated == value_str {
                                found_pattern = true;
                                break;
                            }
                        }
                    }

                    // Wenn ein Muster gefunden wurde, wird die Zahl nur einmal addiert
                    if found_pattern {
                        answer += value_to_check;
                    }
                }
            }
        }
    }

    println!("Alle invaliden IDs addiert ergeben {}", answer)
}