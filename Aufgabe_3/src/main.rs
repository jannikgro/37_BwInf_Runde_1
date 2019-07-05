extern crate rand;
use rand::Rng;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const MAXSTEINE: usize = 10; // Die Zahl der Steine, die Al setzt
const MAXEINSATZ: usize = 500; // Die maximal mögliche Zahl an Einsätzen. Sie wird in der Regel unterschritten
const SPEICHERLOESUNGEN: usize = 50; // Die Zahl der Anordnungen, die pro Generation weiterverfolgt werden
const CHILDREN: usize = 200; // Die Anzahl der Anordnungen, die pro Anordnung durch Mutation entstehen

const RETURN1: u16 = 6789;
const KONST: u16 = 5678;

#[derive(Clone, Copy)]
struct Loesung {
    // Diese Struktur stellt eine Anordnung dar. Sie enthält die Nummern der Einsätze, auf denen Al setzt, und die Abweichung.
    alssteine: [u16; MAXSTEINE],
    abweichung: u16,
}

impl Loesung {
    fn get_loesung() -> Loesung {
        // Der Konstruktor für die Instanzen des Typs "Loesung"
        Loesung {
            alssteine: [0; MAXSTEINE],
            abweichung: 0,
        }
    }

    fn startloesung(&mut self, numeinsatz: u16) {
        // Gemäß der in der Erklärung erklärten Formel wird hier die Startaufstellung definiert.
        for i in 0..MAXSTEINE {
            self.alssteine[i] = ((i + 1) * (numeinsatz as usize) / (MAXSTEINE + 1)) as u16;
        }
    }

    fn gleicher(&self, value: usize) -> bool {
        // Diese Funktion prüft, ob ein eingegebener Wert durch einen Stein dieser Anordnung direkt abgedeckt wird.
        let mut rueckgabe = false;
        for i in 0..MAXSTEINE {
            if self.alssteine[i] == value as u16 {
                rueckgabe = true;
                break;
            }
        }
        rueckgabe
    }

    fn voriger(&self, value: usize) -> usize {
        // Diese Funktion gibt den nächst kleineren Nachbarn zurück.
        let mut voriger = KONST;
        for i in 0..MAXSTEINE {
            if self.alssteine[i] > value as u16 {
                if i == 0 {
                    voriger = RETURN1 // Wenn bereits der von Als Steinen nach dem gesuchten Wert liegt, wird der Schlüssel 6789 in die Variable geschrieben. Dieser Fall wird dann gesondert behandelt.
                } else {
                    voriger = self.alssteine[i - 1]; // Andernfalls wird die Nummer des Vorgängers im Array in die Variable geschrieben.
                }
                break;
            } // Wenn die Schleife durchgelaufen ist, und noch immer 6789 in der Variable steht, ist der Vorgänger der letzte von Als Steinen. Dieser Fall wird dann gesondert behandelt.
        }
        if voriger == KONST {
            voriger = self.alssteine[MAXSTEINE - 1];
        }
        voriger as usize
    }

    fn folgender(&self, value: usize) -> usize {
        // Diese Funktion gibt den nächst größeren Nachbarn zurück.
        let mut nachfolgender = RETURN1;
        for i in 0..MAXSTEINE {
            if self.alssteine[i] > value as u16 {
                nachfolgender = self.alssteine[i]; // In diesem Fall wird die Nummer des Nachfolgers im Array in die Variable geschrieben
                break;
            }
        }
        nachfolgender as usize
    }

    fn mutate(&self, numeinsatz: u16) -> Loesung {
        // Diese Funktion gibt eine mutierte Anordnung zurück.
        let mut child = Loesung::get_loesung(); // Hier wird eine Instanz vom Typ "Loesung" erzeugt.
        let mut rng = rand::thread_rng(); // Hier wird eine Instanz des Zufallsgenerators erzeugt.
        for i in 0..MAXSTEINE {
            let randomnum = rng.gen_range(0, 30); // Hier wird eine Zufallszahl zwischen 0 und 30 erzeugt. Ein Zehntel dieser Zahl wird dann addiert oder subtrahiert
            let positionchange: i16 = {
                if randomnum % 2 == 0 {
                    randomnum / 10 // Gerade Zahlen führen zu einer Addition
                } else {
                    -randomnum / 10 // Ungerade Zahlen führen zu einer Subtraktion
                }
            };
            let position = positionchange + self.alssteine[i] as i16;
            if position >= 0 && position < numeinsatz as i16 {
                child.alssteine[i] = position as u16; // Wenn die Positionsänderung einen gültigen Wert aufweist, bekommt die Anordnung der nächsten Generation diesen Wert
            } else {
                child.alssteine[i] = self.alssteine[i]; // Andernfalls wird die Position der vorigen Anordnung übernommen.
            }
        }
        child
    }

    fn equals(&self, vergleich: &Loesung) -> bool {
        // Diese Funktion prüft, ob zwei Lösungen gleich sind
        let mut equals = true;
        for i in 0..MAXSTEINE {
            if self.alssteine[i] != vergleich.alssteine[i] {
                equals = false;
                break;
            }
        }
        equals
    }
}

#[derive(Clone, Copy)]
struct Speicher {
    // In dieser Struktur werden die Einsätze, die besten Lösungen und die Anzahl der Einsätze gespeichert.
    einsaetze: [u16; MAXEINSATZ],
    beste_loesungen: [Loesung; SPEICHERLOESUNGEN],
    numeinsatz: u16,
}

impl Speicher {
    fn get_speicher() -> Speicher {
        // Konstruktor für Strukturen des Typs "Speicher"
        let newspeicher: Speicher = Speicher {
            einsaetze: [0; MAXEINSATZ],
            beste_loesungen: [Loesung::get_loesung(); SPEICHERLOESUNGEN],
            numeinsatz: 0,
        };
        newspeicher
    }

    fn clonespeicher(&self) -> Speicher {
        // Methode, die einen Klon der eigenen Instanz zurückgibt.
        let mut myspeicher = Speicher::get_speicher();
        for i in 0..(self.numeinsatz as usize) {
            myspeicher.einsaetze[i] = self.einsaetze[i];
        }
        myspeicher.numeinsatz = self.numeinsatz;
        myspeicher
    }

    fn datei_auslesen(&mut self) {
        // Funktion zum Auslesen der Werte aus der Textdatei
        let args: Vec<String> = env::args().collect();
        if args.len() == 1 {
            panic!("Sie müssen als erstes Argument den Dateinamen übergeben!");
        }
        let filename = &args[1];

        println!("Datei {} wird ausgelesen.", filename);

        let f = File::open(filename).unwrap();
        let file = BufReader::new(&f);
        for (i, line) in file.lines().enumerate() {
            let bet = line.unwrap().parse::<u16>().unwrap();
            self.einsaetze[i] = bet;
            self.numeinsatz = (i + 1) as u16;
        }
    }

    fn sorteinsaetze(&mut self) {
        // Bubblesort-Verfahren, um die Einsätze zu sortieren
        for _i in 0..(self.numeinsatz as usize) {
            for j in 0..((self.numeinsatz as usize) - 1) {
                if self.einsaetze[j] > self.einsaetze[j + 1] {
                    let hilf = self.einsaetze[j];
                    self.einsaetze[j] = self.einsaetze[j + 1];
                    self.einsaetze[j + 1] = hilf;
                }
            }
        }
    }

    fn printbest(&self) {
        // Methode, um die beste Anordnung einer Generation auszudrucken
        for i in 0..MAXSTEINE {
            print!(
                "{}, ",
                self.einsaetze[self.beste_loesungen[0].alssteine[i] as usize]
            );
        }
        println!("");
        println!("'-> notwendige Auszahlung: {}", self.beste_loesungen[0].abweichung);
    }

    fn gesamtabweichung(&self, versuch: &mut Loesung) {
        // Methode, die die Gesamtabweichung einer Anordnung bestimmt
        let mut gesamtabweichung = 0;
        for i in 0..(self.numeinsatz as usize) {
            // Diese Schleife zählt durch den Array "einsaetze"
            if versuch.gleicher(i) {
                continue; // Wenn die Position mit einem von Als Steinen besetzt ist, ist keine Abweichung zu addieren
            } else {
                let voriger = versuch.voriger(i); // Die Position des vorher liegenden von Als Steinen wird besimmt
                let folgender = versuch.folgender(i); // Die Position des nachfolgenden wird bestimmt
                if voriger == RETURN1 as usize {
                    gesamtabweichung += self.einsaetze[folgender] - self.einsaetze[i]; // Wenn es keinen von Als Steinen gibt, dessen Wert kleiner ist, wird die Differenz aus dem Wert des folgenden Steins und dem eigenen Einsatz addiert
                } else if folgender == RETURN1 as usize {
                    gesamtabweichung += self.einsaetze[i] - self.einsaetze[voriger]; // Wenn es keinen von Als Steinen gibt, dessen Wert größer ist, wird die Differenz aus dem eigenen Einsatz und dem Wert des vorigen Steins addiert
                } else {
                    let zum_vorigen = self.einsaetze[i] - self.einsaetze[voriger]; // Wenn es Vorgänger und Nachfolger gibt, wird der kleinere Abstand addiert
                    let zum_folgenden = self.einsaetze[folgender] - self.einsaetze[i];
                    if zum_vorigen < zum_folgenden {
                        gesamtabweichung += zum_vorigen;
                    } else {
                        gesamtabweichung += zum_folgenden;
                    }
                }
            }
        }
        versuch.abweichung = gesamtabweichung; // Die Abweichung wird in die Variable der Instanz geschrieben
    }

    fn nachruecken(&mut self, position: usize) {
        // Diese Methode lässt alle Anordnungen im Speicher ab dem Wert "position" eine Position nach hinten wandern
        let mut zaehlvariable: usize = SPEICHERLOESUNGEN - 1;
        loop {
            if zaehlvariable == position {
                break;
            }
            self.beste_loesungen[zaehlvariable] = self.beste_loesungen[zaehlvariable - 1];
            zaehlvariable -= 1;
        }
    }

    fn insert(&mut self, neueloesung: Loesung) {
        // Diese Methode fügt eine neue Anordnung in den Speicher ein
        if neueloesung.abweichung < self.beste_loesungen[SPEICHERLOESUNGEN - 1].abweichung
            || self.beste_loesungen[SPEICHERLOESUNGEN - 1].abweichung == 0
        { // Eine neue Anordnung wird nur eingefügt, wenn ihre Abweichung kleiner als die schlechteste gespeicherte Anordnung der Vorgeneration ist
            for i in 0..SPEICHERLOESUNGEN {
                if self.beste_loesungen[i].equals(&neueloesung) {
                    break; // Wenn eine gleiche Anordnung bereits existiert, wird sofort abgebrochen
                }
                if neueloesung.abweichung < self.beste_loesungen[i].abweichung
                    || self.beste_loesungen[i].abweichung == 0
                {
                    self.nachruecken(i); // Wenn die Abweichung dieser neuen Anordnung eine geringere Abweichung aufweist als die an der Stelle i gespeicherte, werden die Anordnungen ab hier weitergerückt, und die neue eingefügt.
                    self.beste_loesungen[i] = neueloesung;
                    break;
                }
            }
        }
    }

    fn new_generation(&self, ziel: &mut Speicher) {
        // Diese Methode erstellt die neue Generation
        ziel.insert(self.beste_loesungen[0]); // Die beste Lösung der vorigen Generation wird auf jeden Fall unverändert übertragen.
        for i in 0..SPEICHERLOESUNGEN {
            if self.beste_loesungen[i].abweichung == 0 {
                break; // Dieser Fall ist nur am Anfang erfüllt, wenn der Speicher noch leer ist.
            }
            for _j in 0..CHILDREN {
                let mut help = self.beste_loesungen[i].mutate(self.numeinsatz); // So oft wie gefordert, wird eine bestimmte Anordnung mutiert
                self.gesamtabweichung(&mut help); // Für das Ergebnis dieser Mutation wird die Abweichung berechnet
                ziel.insert(help); // Die Mutation wird in den Speicher eingefügt
            }
        }
    }
}

fn main() {
    let mut myspeicher: [Speicher; 2] = [Speicher::get_speicher(); 2]; // In einem Array werden zwei Instanzen vom Typ "Speicher" erstellt.
    myspeicher[0].datei_auslesen(); // Die Werte werden ausgelesen
    let numeinsatz = myspeicher[0].numeinsatz;
    myspeicher[0].sorteinsaetze(); // Die Werte werden sortiert
    //myspeicher[0].einsaetze.sort();
    myspeicher[1] = myspeicher[0].clonespeicher(); // Der Speicher wird geklont und im Array ein zweites Mal gespeichert
    let mut zwischen = myspeicher[0].beste_loesungen[0];
    zwischen.startloesung(numeinsatz); // Die Ausgangsanordnung wird erstellt
    myspeicher[0].gesamtabweichung(&mut zwischen); // Die Abweichung dieser Anordnung wird berechnet
    myspeicher[0].beste_loesungen[0] = zwischen; // Diese Anordnung wird als beste bisher bekannte Anordnung gespeichert
    let maxgen = 10;
    for i in 0..maxgen {
        println!("Ich bin jetzt bei Generation {}:", i); // Abwechselnd werden aus einem Speicher die Anordnungen mutiert und in dem anderen gespeichert
        let start = myspeicher[i % 2];
        start.new_generation(&mut myspeicher[1 - (i % 2)]);
        myspeicher[1 - (i % 2)].printbest(); // Die jeweils beste Anordnung der neuen Generation wird ausgedruckt
    }
}