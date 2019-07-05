public class Person {
    final int maxpers; // Die Zahl der Personen in der Gruppe
    Person folgen[]; // Hier werden von Anfang an alle Personen gespeichert, denen die Person folgt
    //Person gefolgt[];

    Person folgenfrei[]; // Hier werden die Personen gespeichert, bei denen bereits bekannt ist, dass die Person ihnen folgt
    Person nichtfolgen[]; // Hier werden die Personen gespeichert, bei denen bereits bekannt ist, dass die Person ihnen nicht folgt
    //Person gefolgtfrei[];

    public String name;

    boolean bin_ich_gerade_kandidat;
    public boolean bin_ich_ueberhaupt_noch_kandidat; // Hier wird gespeichert, ob die Person noch Kandidat ist (Wenn bereits feststeht, dass diese Person kein Superstar ist, steht hier false)

    // Der Konstruktor der Klasse "Person"
    public Person(String uname, int umaxpers) {
        maxpers = umaxpers;
        name = uname;
        folgen = new Person[maxpers];
        folgenfrei = new Person[maxpers];
        nichtfolgen = new Person[maxpers];
        bin_ich_gerade_kandidat = false;
        bin_ich_ueberhaupt_noch_kandidat = true;
        //System.out.println(name + " ist angemeldet.");
    }

    // Diese Methode zaehlt durch den Array, der die Personen speichert, denen gefolgt wird
    public void hinzufuegen_folgen(Person kontakt) {
        int i = 0;
        while (true) {
            if (folgen[i] == null)
                break;
            if (folgen[i].name.equals(kontakt.name))
                return;
            i++;
        }
        folgen[i] = kontakt;
    }

    // Diese Methode zaehlt durch den Array, der die Personen speichert, bei denen bereits bekannt ist, dass ihnen von der Person gefolgt wird, und fuegt eine neue ein
    void hinzufuegen_folgenfrei(Person kontakt) {
        int i = 0;
        while (true) {
            if (folgenfrei[i] == null)
                break;
            if (folgenfrei[i].name.equals(kontakt.name))
                return;
            i++;
        }
        folgenfrei[i] = kontakt;
    }

    // Diese Methode zaehlt durch den Array, der die Personen speichert, bei denen bereits bekannt ist, dass ihnen von der Person nicht gefolgt wird, und fuegt eine neue ein
    public void hinzufuegen_nichtfolgen(Person kontakt) {
        int i = 0;
        while (true) {
            if (nichtfolgen[i] == null)
                break;
            if (nichtfolgen[i].name.equals(kontakt.name))
                return;
            i++;
        }
        nichtfolgen[i] = kontakt;
    }

    // Diese Funktion prueft, ob die Person den Namen traegt
    public boolean isPerson(String persname) {
        return persname.equals(name);
    }

    int get_num(Person[] personen) {
        for (int i = 0; i < personen.length; i++)
            if (this.isPerson(personen[i].name))
                return i;
        return 0;
    }

    // Diese Methode ist das Zentralelement des Backtracking-Algorithmus. Sie den Zeiger auf die Person zurueck, falls es einen Superstar gibt, und "null", wenn es keinen gibt
    public Person wer_ist_der_superstar(Konto konto, Person[] personen) {
        if (bin_ich_gerade_kandidat || !bin_ich_ueberhaupt_noch_kandidat)
            return null; // Sollte diese Person bereits einmal in der Rekursion vorkommen, wird zurueckgesprungen
        bin_ich_gerade_kandidat = true; // Damit eine Person nicht zweimal abgefragt wird, wird das hier vermerkt
        // Diese Schleife zaehlt durch die Personen, um rekursiv durchzufragen
        int start = get_num(personen);
        for (int z = start; z < start + personen.length; z++) {
            int i = z % personen.length;
            if (personen[i].name.equals(this.name))
                continue; // Die Aufgabe wird von einer Person nicht an sich selbst weitergegeben
            if (!personen[i].bin_ich_ueberhaupt_noch_kandidat && !bin_ich_ueberhaupt_noch_kandidat)
                continue; // Wenn eine Person kein Kandidat mehr ist, und man selber auch nicht, ist diese Person uninteressant
            if (folgst_Du_(personen[i], konto)) {
                // Wenn die Person einer anderen folgt, ist sie der Definition entsprechend kein Kandidat mehr. Einer Person, der gefolgt wird, wird nun die Aufgabe des Durchfragens uebergeben
                bin_ich_ueberhaupt_noch_kandidat = false;
                Person superstar = null;
                superstar = personen[i].wer_ist_der_superstar(konto, personen);
                if (superstar != null)
                    return superstar; // Wenn eine andere Person der Superstar ist, wird ein Zeiger auf ihn zurueckgegeben
            } else personen[i].bin_ich_ueberhaupt_noch_kandidat = false;
        }
        if (bin_ich_ueberhaupt_noch_kandidat) // Wenn die Person noch Kandidat ist, muss geprueft werden, ob ihr alle anderen folgen
        {
            for (int i = 0; i < personen.length; i++) {
                if (personen[i].name.equals(this.name))
                    continue; // Niemand kann sich selber folgen, deswegen muss dieser Fall uebersprungen werden
                if (!personen[i].folgst_Du_(this, konto)) {
                    bin_ich_ueberhaupt_noch_kandidat = false;
                    return null; // Wenn irgendeine Person dieser nicht folgt, ist sie kein Superstar
                }
            }
        } else {
            bin_ich_gerade_kandidat = false;
            return null; // Wenn eine Person nach dieser Ueberpruefung kein Superstar mehr ist, muss "null" zurueckgegeben werden
        }
        bin_ich_gerade_kandidat = false;
        return this; // Hier kommt die Funktion genau dann an, wenn die Person der Superstar ist. Auf ihn wird ein Zeiger zurueckgegeben
    }

    // Diese Methode ist dafuer zustaendig, die Frage, die von der Methode "wer_ist_der_superstar" gestellt wird, zu beantworten
    public boolean folgst_Du_(Person gesucht, Konto konto) {
        //Die unter der naechsten Zeile eingeleitete Schleife, prueft zuerst, ob sich die Frage bereits mit dem bisherigen Wissen beantworten laesst. Wenn der Name der erfragten Person in dem Array "folgenfrei" steht, ist klar, dass dieser Person gefolgt wird, wenn der Name im Array "nichtfolgen" enthalten ist, kann zurueckgegeben werden, dass der gefragten Person nicht gefolgt wird.
        int maxloop = (nichtfolgen.length > folgenfrei.length) ? nichtfolgen.length : folgenfrei.length;
        for (int i = 0; i < maxloop; i++) {
            if (folgenfrei[i] != null)
                if (folgenfrei[i].name.equals(gesucht.name)) {
                    System.out.printf("Ich weiss bereits, dass %s %s folgt.\n", this.name, gesucht.name);
                    return true;
                }
            if (nichtfolgen[i] != null)
                if (nichtfolgen[i].name.equals(gesucht.name)) {
                    System.out.printf("Ich weiss bereits, dass %s %s nicht folgt.\n", this.name, gesucht.name);
                    return false;
                }
        }
        // Sollte noch kein Wert zurueckgegeben sein, bedeutet das, dass das Verhaeltnis ungeklaert ist. Dann wird eine Zahlung faellig
        konto.naechste_zahlung();
        // Diese Schleife zaehlt durch den Array der Personen, in dem eingangs alle gespeichert wurden, denen die Person folgt
        for (int i = 0; i < folgen.length; i++) {
            if (folgen[i] == null) {
                hinzufuegen_nichtfolgen(gesucht);
                //System.out.printf("Ich habe jetzt erfragt, dass %s %s nicht folgt.\n", this.name, gesucht.name);
                return false;
            }
            if (folgen[i].name.equals(gesucht.name)) {
                hinzufuegen_folgenfrei(gesucht);
                //System.out.printf("Ich habe jetzt erfragt, dass %s %s folgt.\n", this.name, gesucht.name);
                return true;
            }
        }
        return false;
    }

    // Diese Methode kann bei Bedarf die Personen ausdrucken, denen die Person folgt
    /*public void wem_ich_folge() {
        System.out.printf("Hallo, ich bin %s und folge folgenden Personen: ", name);
        for (int i = 0; i < folgen.length; i++) {
            if (folgen[i] == null) break;
            System.out.printf("%s, ", folgen[i].name);
        }
        System.out.printf("\n");
    }*/
}