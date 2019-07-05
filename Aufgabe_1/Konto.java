public class Konto {
    int kontostand; // Diese Klasse speichert die bisherigen Ausgaben

    public Konto() {
        kontostand = 0;
    }

    // Wenn eine Frage gestellt wird, werden die Kosten hier erhoeht
    public void naechste_zahlung() {
        kontostand++;
    }

    // Diese Methode gibt einen String zurueck, in dem der Preis steht
    public String wie_teuer_war_es() {
        return "Das Erfragen des Superstars hat " + kontostand + " Euro gekostet.";
    }
}