extern crate clap;
extern crate encoding_rs;
extern crate encoding_rs_io;
extern crate rand;
extern crate regex;
#[macro_use]
extern crate lazy_static;

use clap::{App, AppSettings, Arg, SubCommand};
use encoding_rs::*;
use encoding_rs_io::DecodeReaderBytesBuilder;
use rand::{thread_rng, Rng};
use regex::Captures;
use regex::Regex;
use std::collections::BTreeMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::path::Path;

// In diesem Enum werden die Befehle der Kommandozeile übergeben.
enum Command {
    None,
    Twist(String, String),
    Detwist(String, String, String),
    CreateDictionary(String, String),
}

// In der folgenden Funktion werden die Übergabeparameter von der Kommandozeile ausgelesen und verarbeitet
fn get_command() -> Command {
    static AUTHOR: &'static str = "Jan Niklas Groeneveld <jan.groeneveld@rhs-bs.de>";
    static VERSION: &'static str = "1.0";
    let matches = App::new("twister")
        .about("Twistet und enttwistet Wörter")
        .version(VERSION)
        .author(AUTHOR)
        .subcommand(
            SubCommand::with_name("twist")
                .about("Twistet Wörter")
                .arg(Arg::with_name("input").help("Eingabedatei").required(true))
                .arg(Arg::with_name("output").help("Ausgabedatei").required(true)),
        ).subcommand(
            SubCommand::with_name("detwist")
                .about("Enttwistet Wörter")
                .arg(Arg::with_name("input").help("Eingabedatei").required(true))
                .arg(Arg::with_name("output").help("Ausgabedatei").required(true))
                .arg(
                    Arg::with_name("dictionary")
                        .help("Wörterbuch")
                        .required(true),
                ),
        ).subcommand(
            SubCommand::with_name("create")
                .about("Erzeugt ein Wörterbuch")
                .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
                .arg(Arg::with_name("input").help("Eingabedatei").required(true))
                .arg(
                    Arg::with_name("dictionary")
                        .help("Wörterbuch")
                        .required(true),
                ),
        ).get_matches();

    match matches.subcommand() {
        ("twist", Some(twist_matches)) => {
            let input = twist_matches.value_of("input").unwrap();
            let output = twist_matches.value_of("output").unwrap();
            println!("Twiste {} nach {}", input, output);
            return Command::Twist(input.to_string(), output.to_string());
        }
        ("detwist", Some(detwist_matches)) => {
            let input = detwist_matches.value_of("input").unwrap();
            let output = detwist_matches.value_of("output").unwrap();
            let dictionary = detwist_matches.value_of("dictionary").unwrap();
            println!(
                "Enttwiste {} nach {} mit dem Wörterbuch {}",
                input, output, dictionary
            );
            return Command::Detwist(
                input.to_string(),
                output.to_string(),
                dictionary.to_string(),
            );
        }
        ("create", Some(create_matches)) => {
            let input = create_matches.value_of("input").unwrap();
            let dictionary = create_matches.value_of("dictionary").unwrap();
            println!("Erstelle das Wörterbuch {} aus {}", dictionary, input);
            return Command::CreateDictionary(input.to_string(), dictionary.to_string());
        }
        ("", None) => {
            println!("Bitte Subkommando eingeben. Für weitere Informationen bitte twist.exe --help eingeben");
            return Command::None;
        }
        _ => unreachable!(), // Falls alle Subcommands definiert sind, wird dieser Punkt nicht erreicht
    }
}

// Diese Funktion twistet ein Wort. Wenn die Länge zwischen 1 und 3 liegt, geschieht nichts, bei einer Länge 4 werden die Buchstaben in der Mitte getauscht, in jedem anderen Fall wird das ganze Wort bis zu 20 mal durchgewürfelt.
fn twist_word(word: String) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    let len = chars.len();
    match len {
        1...3 => return word,
        4 => {
            chars.swap(1,2);
            chars.into_iter().collect::<String>()
        }
        _ => {
            let mut counter = 0;
            loop {
                counter += 1;
                let slice: &mut [char] = &mut chars[1..len - 1];
                let c0 = slice[0];
                let c1 = slice[1];
                thread_rng().shuffle(slice);
                if c0 != slice[0] || c1 != slice[1] || counter > 20 {
                    break;
                }
            }
            chars.into_iter().collect::<String>()
        }
    }
}

// Diese Funktion sortiert alle Buchstaben eines Wortes, außer dem Ersten und dem Letzten, nach dem Alphabet. Dadurch erhält man für jedes Wort einen Schlüssel, dem im Binärbaum das ursprüngliche Wort zugeordnet ist.
fn sort_word(word: String) -> String {
    if word.len() < 4 {
        return word;
    }
    let mut chars: Vec<char> = word.chars().collect();
    let len = chars.len();
    chars[1..len - 1].sort_by(|a, b| a.cmp(b));
    chars.into_iter().collect::<String>()
}

// Aus dem übergebenen String werden die Wörter mithilfe eines regulären Ausdruckes herausgesucht. Diese werden dann einzeln durch das Wort ersetzt, das als Rückgabeparameter der "shuffle_word()"-Funktion zurückgegeben wird.
fn twist_text(text: &str) -> String {
    lazy_static! {
        static ref re: Regex = Regex::new(r"(\w+)").unwrap();
    }
    let output = re.replace_all(&text, |captures: &Captures| {
        let text1 = captures.get(1).map_or("", |m| m.as_str());
        //println!("{:?}", text1);
        twist_word(text1.to_string())
    });
    output.to_string()
}

// Aus dem übergebenen String werden die Wörter mithilfe eines regulären Ausdruckes herausgesucht. Dann wird eine Routine aufgerufen, die jedes Wort einzeln durch das Wort ersetzt, das im Binärbaum mit dem entsprechenden Schlüssel abgespeichert ist.
fn detwist_text(text: &str, btm: &BTreeMap<String, String>) -> String {
    lazy_static! {
        static ref re: Regex = Regex::new(r"(\w+)").unwrap();
    }
    let output = re.replace_all(&text, |captures: &Captures| {
        let text1 = captures.get(1).map_or("", |m| m.as_str());
        //println!("{:?}", text1);
        let key = sort_word(text1.to_string());
        if let Some(value) = btm.get(&key) {
            /*println!(
                "FOUND: twist : {}, sorted: {}, correct: {}",
                text1, key, value
            ); */
            value
        } else {
            // Wort wurde nicht gefunden. Sollte nicht vorkommen. Das urspruengliche Wort wird beibehalten
            println!(
                "NOT FOUND: twist : {}, sorted: {}, correct: {}",
                text1, key, text1
            );
            text1
        }
    });
    output.to_string()
}

// Hier wird ein String mit dem Inhalt des Wörterbuches übergeben, dann wird er mithilfe eines regulären Ausdruckes in die einzelnen Wörter zerlegt, die dann in den Binärbaum eingefügt werden.
fn create_btree_map_2(input_path: &str) -> BTreeMap<String, String> {
    lazy_static! {
        static ref re: Regex = Regex::new(r"(\w+)").unwrap();
    }

    re.find_iter(&input_path)
        .map(|mat| mat.as_str())
        .fold(BTreeMap::new(), |mut acc, word| {
            acc.entry(sort_word(word.to_string()))
                .or_insert(word.to_string());
            acc
        })
}

// Diese Funktion implementiert das Einlesen: Weil Rust mit UTF-8 arbeitet, wird zuerst versucht, die Datei als UTF-8 einzulesen. Falls das nicht gelingt, wird die Windows 1252-Codierung genutzt
fn read_file_to_string(file: &str) -> Result<String, io::Error> {
    {   // Erster Versuch: Versuche UTF-8 Codierung
        let mut read1 = File::open(&Path::new(file))?;
        let mut dest1 = String::new();
        match read1.read_to_string(&mut dest1) {
            Ok(_) => return Ok(dest1),
            Err(e) => println!("{}\nBenutze Windows 1252 Codierung", e),
        };
        // Datei wird geschlossen
    }

    // Zweiter Versuch: Benutze Windows 1252 Codierung
    let mut buf = Vec::new();

    let mut read = File::open(&Path::new(file))?;
    read.read_to_end(&mut buf)?;
    let enc = Encoding::for_label("windows-1252".as_bytes());
    let mut decoder = DecodeReaderBytesBuilder::new()
        .encoding(enc)
        .utf8_passthru(true)
        .build(buf.as_slice());
    let mut dest2 = String::new();
    decoder.read_to_string(&mut dest2)?;
    Ok(dest2)
}

// Diese Funktion lässt einen Text einlesen, ruft die Funktion zum Twisten des Textes auf, und schreibt das Ergebnis in einen neue Textdatei mit entsprechendem Namen
fn twist_file(input: &str, output: &str) -> std::io::Result<()> {
    let text = read_file_to_string(input)?;
    let twisted_text = twist_text(&text);
    let mut file = File::create(output)?;
    file.write_all(twisted_text.as_bytes())?;
    file.sync_all()?;
    Ok(())
}

// Diese Funktion lässt einen getwisteten Text einlesen, erstellt einen Binärbaum mit den Wörtern des Wörterbuches, und ruft die Funktion zum Enttwisten auf. Das Ergebnis wird in eine neue Textdatei mit entsprechendem Namen geschrieben.
fn detwist_file(input: &str, output: &str, dictionary: &str) -> std::io::Result<()> {
    let text = read_file_to_string(input)?;
    let dictionary = read_file_to_string(dictionary)?;
    let btm = create_btree_map_2(&dictionary);
    //print_btree_map_2(&btm);
    let text2 = detwist_text(&text, &btm);
    let mut file = File::create(output)?;
    file.write_all(text2.as_bytes())?;
    file.sync_all()?;
    Ok(())
}

// In dieser Funktion wird zuerst geprüft, ob ein Wörterbuch mit dem übergebenen Namen schon existiert. Wenn das der Fall ist, wird es erweitert, ansonsten wird ein neues erstellt
fn extend_dictionary(input_path: &str, dic_path: &str) -> std::io::Result<()> {
    // println!("Dic Path {}, input path {}", dic_path, input_path);
    let mut dic = if !Path::new(dic_path).exists() {
        OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(dic_path)
            .unwrap()
    } else { 
        OpenOptions::new()
            .write(true)
            .append(true)
            .open(dic_path)
            .unwrap()
    };

    let dictionary = read_file_to_string(dic_path)?;
    let mut btm = create_btree_map_2(&dictionary);
    let input = read_file_to_string(input_path)?;

    lazy_static! {
        static ref re: Regex = Regex::new(r"(\w+)").unwrap();
    }
    for m in re.find_iter(&input) {
        let word = m.as_str();
        match btm.get(word) {
            Some(_) => {} //println!("{}", word),
            None => {
                //println!("{} not found", word);
                btm.insert(word.to_string(), word.to_string());
                writeln!(dic, "{}", word)?
            }
        }
    }

    Ok(())
}

// Die main()-Funktion liest die Übergabeparameter aus der Kommandozeile, und twistet, enttwistet oder erweitert das Wörterbuch
fn main() -> std::io::Result<()> {
    match get_command() {
        Command::Twist(input, output) => {
            twist_file(&input, &output)?;
        }
        Command::Detwist(input, output, dictionary) => {
            detwist_file(&input, &output, &dictionary)?;
        }
        Command::CreateDictionary(input, dictionary) => {
            extend_dictionary(&input, &dictionary)?;
        }
        Command::None => {}
    }

    Ok(())
}
