public class Main {

    public static void main(String[] args) {
        // In den folgenden Zeilen wird die vom Nutzer geforderte Textdatei ausgelesen
        SimpleInput si = new SimpleInput();
        String input = si.getString("Bitte geben Sie den Namen der Textdatei ein!\nSie sollte sich im gleichen Verzeichnis befinden.");
        ZeilenweiseAuslesen za = new ZeilenweiseAuslesen(input);
        String text = za.auslesen(1);
        // Die erste Zeile wird so in die einzelnen Namen zerlegt, weil die Namen durch Leerzeichen getrennt sind
        String[] personenstring = text.split(" ");
        int anzahlpersonen = personenstring.length;
        // Hier werden Instanzen der Klasse "Person" mit den jeweiligen Namen angelegt
        Person[] personen = new Person[anzahlpersonen];
        for (int i = 0; i < anzahlpersonen; i++)
            personen[i] = new Person(personenstring[i], anzahlpersonen);
        // In der folgenden Routine werden die folgenden Zeilen, in denen die Beziehungen der Personen gesschrieben sind, ausgelesen
        int zaehler = 2;
        while (true) {
            String ausgelesen = za.auslesen(zaehler);
            if (ausgelesen == null) break;
            if (ausgelesen.length() <= 1) break;
            zaehler++;
            anfuegen(ausgelesen, personen);
        }
        // Bei Bedarf kann hier die Liste derjenigen ausgegeben werden, denen von einer Person gefolgt wird
        /*for (int i = 0; i < anzahlpersonen; i++)
            personen[i].wem_ich_folge();*/
        Konto konto = new Konto(); // Hier wird das Konto, das die Kosten zaehlt, initialisiert
        String ausgabe = "Die Gruppe enthaelt " + anzahlpersonen + " Personen\n";
        Person superstar = personen[0].wer_ist_der_superstar(konto, personen); // Diese Routine beginnt die Backtracking-Suche nach dem Superstar
        // In der folgenden Abfrage wird der Ausgabetext gesetzt
        if (superstar != null) ausgabe += "Der Superstar ist " + superstar.name + "!";
        else ausgabe += "In dieser Gruppe gibt es keinen Superstar.";
        ausgabe += "\n";
        ausgabe += konto.wie_teuer_war_es();
        si.message(ausgabe);
        //System.out.println(ausgabe);
        System.exit(0);
    }

    static void anfuegen(String eingabe, Person[] personen) {
        String[] connection = eingabe.split(" "); // Die beiden Namen werden getrennt
        Person folgender = findperson(connection[0], personen); // Die erste der beiden Personen ist die, die folgt
        Person verfolgter = findperson(connection[1], personen); // Die zweite der beiden Personen ist die, der gefolgt wird
        folgender.hinzufuegen_folgen(verfolgter); // Es wird der folgenden Person die Person angefuegt, der gefolgt wird
    }

    static Person findperson(String name, Person[] personen) // Diese Methode sucht im Array der Personen nach einem Namen und gibt einen Zeiger auf die Instanz zurueck
    {
        for (int i = 0; i < personen.length; i++) {
            if (personen[i].isPerson(name))
                return personen[i];
        }
        return null;
    }
}