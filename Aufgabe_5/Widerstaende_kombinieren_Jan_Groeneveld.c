#include <stdio.h>
#include <stdlib.h>

#define MAXres 4 // Hier wird die maximale Anzahl der zu verbauenden Widerstaende definiert
#define MAXgrabbel 24 // Hier wird die maximale Anzahl der Widerstaende in der Grabbelkiste definiert

typedef char connection; // Fuer die Verschaltung der Widerstaende wird ein neuer Datentyp "connection" definiert. Er kann unbenutzt, seriell oder parallel sein.
#define seriell 's'
#define parallel 'p'
#define unused 'u'

typedef char bool; // Definition eines boolschen Datentyps
#define true 't'
#define false 'f'
int grabbelkiste[MAXgrabbel]; // In diesem Array werden die Werte der Widerstaende in der Grabbelkiste gespeichert.
bool verwendet[MAXgrabbel]; // In diesem Array wird gespeichert, ob ein Widerstand bereits verwendet wird.
int reihenfolge[MAXres]; // In diesem Array werden die Werte fuer die Widerstaende nach der Reihenfolge gespeichert
double bestlow = 10000.0; // Hier wird die niedrigste bereits erreichte Abweichung gespeichert. 
double angestrebt; // Hier wird der Widerstandswert gespeichert, der erreicht werden soll.

int totalgrabbel; // Die Zahl der in der Grabbelkiste vorhandenen Widerstaende
int totalres; // Die Zahl der zu verwendenden Widerstaende

typedef struct Resistor // Die Struktur, die fuer den komplexen Datentyp des Widerstandbauplans benoetigt wird
{
	struct Resistor *pre; // Der Vorgaenger wird gespeichert
	struct Resistor *r1; // Die Zeiger auf die beiden Widerstaende, die den einen ersetzen
	struct Resistor *r2;
	connection con; // Die Art der Verschaltung der zwei Widerstaende
	double resistance; // Der Widerstandswert, der dem Widerstand zugeordnet ist. Er kann auch der Ersatzwert sein
	int untergeordnete; // Die Anzahl der insgesamt diesem Widerstand untergeordneten Widerstaende
} Resistor;

Resistor *bestcombi; // Der Zeiger auf die Kombination mit der niedrigsten Abweichung. Der Speicherplatz, der hinter diesem Zeiger steht, ist unabhaengig vom rekursiven Erstellen neuer Bauplaene.

Resistor *new_Resistor() // Funktion zum Erstellen einer neuen Zeigerstruktur "Resistor"
{
	Resistor *nres = (Resistor *)malloc(sizeof(Resistor));
	nres->pre = NULL;
	nres->r1 = NULL;
	nres->r2 = NULL;
	nres->con = unused;
	nres->resistance = 0;
	nres->untergeordnete = 0;
	return nres;
}

Resistor *clone_Resistor(Resistor*myresistor, Resistor*preresistor) // Funktion, die eine bestehende Zeigerstruktur auf einen neuen Speicherplatz klont
{
	Resistor *nres = (Resistor *)malloc(sizeof(Resistor));
	nres->pre = preresistor;
	nres->r1 = (myresistor->r1 != NULL) ? clone_Resistor(myresistor->r1, myresistor) : NULL;
	nres->r2 = (myresistor->r2 != NULL) ? clone_Resistor(myresistor->r2, myresistor) : NULL;
	nres->con = myresistor->con;
	nres->resistance = myresistor->resistance;
	nres->untergeordnete = myresistor->untergeordnete;
	return nres;
}

void remove_resistor(Resistor *myresistor) // Funktion, die eine Zeigerstruktur sauber wieder vom Speicher entfernt
{
	if (myresistor != NULL)
	{
		remove_resistor(myresistor->r1);
		remove_resistor(myresistor->r2);
		free(myresistor);
	}
}

int auslesen() // Auslesen der Werte fuer die Grabbelkiste aus der Textdatei
{
	FILE *fp;
	fp = fopen("widerstaende.txt", "r");
	for (int i = 0; i < MAXgrabbel; i++)
		grabbelkiste[i] = 0;
	int temp;
	int zahl = 0;
	int zaehler = 0;
	while ((temp = fgetc(fp)) != EOF)
	{
		if (temp >= 48 && temp <= 58)
		{
			zahl *= 10;
			zahl += ((int)temp - 48);
		}
		else if (zahl != 0)
		{
			grabbelkiste[zaehler] = zahl;
			zahl = 0;
			zaehler++;
		}
		else continue;
	}
	totalgrabbel = zaehler;
	fclose(fp);
	return 0;
}

void printway2(Resistor *myresistor) // Ausdruck des Bauplans fuer die Widerstaende
{
	if (myresistor->r1 != NULL)
	{
		printf("(");
		printway2(myresistor->r1);
		printf("%c", (myresistor->con == seriell) ? '-' : '=');
		printway2(myresistor->r2);
		printf(")");
	}
	else
		printf("%d", (int)myresistor->resistance);
}

int count(Resistor *myresistor) // Funktion zum Zaehlen der Widerstaende, die einem Widerstand untergeordnet sind
{
	if (myresistor->r1 != NULL)
		myresistor->untergeordnete = count(myresistor->r1) + count(myresistor->r2);
	else
		myresistor->untergeordnete = 1;
	return myresistor->untergeordnete;
}

void weiterreichen(Resistor *myresistor, int einzelwiderstaende[]) // Funktion, die einen Array der Widerstandswerte an die untergeordneten Widerstaende weitergibt
{
	if (myresistor->r1 != NULL)
	{
		int links[MAXres]; // Falls der aktuelle Widerstand kein Endwiderstand ist, werden neue Arrays zur Weitergabe deklariert.
		int rechts[MAXres];
		for (int i = 0; i < myresistor->r1->untergeordnete; i++) // Diese Arrays werden mit jeweils so vielen Widerstandswerten aufgefuellt, wie die Nachgeordneten untergeordnete Widerstaende besitzen
			links[i] = einzelwiderstaende[i];
		for (int i = 0; i < myresistor->r2->untergeordnete; i++)
			rechts[i] = einzelwiderstaende[i + myresistor->r1->untergeordnete];
		weiterreichen(myresistor->r1, links); // Die Arrays mit den Widerstandswerten werden an die Nachgeordneten weitergegeben
		weiterreichen(myresistor->r2, rechts);
	}
	else // Falls der aktuelle Widerstand ein Endwiderstand ist, hat der Array noch genau ein Element, das der Wert des Endwiderstandes ist.
		myresistor->resistance = (double)einzelwiderstaende[0];
}

double calculate(Resistor *myresistor) // Diese Funktion berechnet rekursiv die Werte der Ersatzwiderstaende
{
	if (myresistor->r1 != NULL)
	{
		double wider1 = calculate(myresistor->r1);
		double wider2 = calculate(myresistor->r2);

		if (myresistor->con == seriell)
			myresistor->resistance = wider1 + wider2;
		else
			myresistor->resistance = wider1 * wider2 / (wider1 + wider2);
	}
	return myresistor->resistance;
}

void recombine(Resistor *startresistor, int depth) // Diese Funktion wird dann aufgerufen, wenn ein fertiger Bauplan erstellt wurde. Sie kombiniert durch rekursive Aufrufe die Widerstandswerte.
{
	if (depth < totalres)
	{
		for (int i = 0; i < totalgrabbel; i++) // Wenn noch nicht fuer jeden Endwiderstand ein Wert zugeordnet wurde, zaehlt eine Schleife durch die Grabbelkiste
		{
			if (verwendet[i] == false) // Wenn der entsprechende Widerstand noch nicht verwendet wurde, wird er ausprobiert
			{
				verwendet[i] = true; // Er wird reserviert, damit er nicht doppelt verwendet wird
				reihenfolge[depth] = grabbelkiste[i]; // Der Endwiderstand der entsprechenden Tiefe bekommt den Wert
				if (bestlow != 0) recombine(startresistor, depth + 1); // Die Aufgabe wird rekursiv weitergegeben
				verwendet[i] = false; // Die Reservierung des Widerstandes wird aufgeloest
			}
		}
	}
	else
	{
		weiterreichen(startresistor, reihenfolge); // Wenn so viele Werte wie Widerstaende kombiniert wurden, werden die Werte an die Endwiderstaende weitergegeben.
		double erg = calculate(startresistor); // Der Widerstand des Bauplans mit den Werten wird berechnet.
		double differenz = angestrebt - erg;
		if (differenz < 0) differenz = -differenz; // Der Betrag der Differenz wird gebildet.
		if (differenz < bestlow)
		{
			remove_resistor(bestcombi); // Wenn die Differenz geringer ist als die bisher geringste Differenz, wird der Speicher, der den Bauplan speichert, freigegeben.
			bestcombi = clone_Resistor(startresistor, NULL); // Anschliessend wird er mit dem neuen Bauplan besetzt.
			bestlow = differenz; // Die geringste Differenz wird angepasst.
		}
	}
}

void go_back(Resistor *myresistor) // Diese Funktion dient dazu, von einem gebauten Widerstandsbauplan an seinen Anfang zurueckzugehen.
{
	if (myresistor->pre != NULL)
		go_back(myresistor->pre); // Wenn der Anfang noch nicht erreicht ist, wird die Aufgabe rekursiv weitergegeben.
	else
	{
		if (count(myresistor) == totalres) // Wenn die Anzahl der verbauten Transistoren der angestrebten Anzahl entspricht, beginnt die Kombination der Werte.
			recombine(myresistor, 0);
	}
}

void netzaufbau(Resistor *preresistor, int remainingnum) // Diese Funktion dient dazu, einen Bauplan fuer die Widerstaende zu erstellen, der noch keine Werte beinhaltet.
{
	remainingnum--; // Weil ein Widerstand durch zwei ersetzt werden soll, muss die Zahl der noch zu verbauenden Widerstaende verringert werden.
	for (int i = 0; i < 2; i++)
	{
		preresistor->con = (i == 0) ? seriell : parallel; // Die zwei Moeglichkeiten der Kombination werden durch eine Zaehlschleife, die von null bis eins zaehlt, ausprobiert.
		preresistor->r1 = new_Resistor(); // Die beiden Zeiger bekommen eine neue Zeigerstruktur
		preresistor->r2 = new_Resistor();
		preresistor->r1->pre = preresistor; // Die Nachfolger bekommen ihren Vorgaenger eingespeichert.
		preresistor->r2->pre = preresistor;
		if (remainingnum > 0) // Falls noch weitere Widerstaende zu verbauen sind, teilt eine Zaehlschleife die verbleibenden auf, sodass die Aufgabe rekursiv weitergegeben werden kann.
		{
			for (int j = 0; j < remainingnum; j++)
			{
				if (remainingnum - j > 0 && bestlow != 0)
					netzaufbau(preresistor->r1, remainingnum - j);
				if (j > 0 && bestlow != 0)
					netzaufbau(preresistor->r2, j);
			}
		}
		else
			go_back(preresistor);
        
        remove_resistor(preresistor->r1); // Die Nachfolger werden wieder sauber geloescht.
        remove_resistor(preresistor->r2);
		preresistor->r1 = NULL;
		preresistor->r2 = NULL;
	}
}

int main()
{
	auslesen(); // Die Textdatei wird gelesen.
	printf("Welchen Widerstandswert soll ich durch Kombination erreichen?\n"); // Nutzerabfrage
	scanf("%lf", &angestrebt);
	totalres = MAXres;
	for (int i = 0; i < MAXgrabbel; i++)
		verwendet[i] = false;
	for (int i = 2; i <= MAXres; i++) // Diese Schleife zaehlt von 2 bis zur maximalen Zahl von Widerstaenden, um diese jeweils auszuprobieren.
	{
		totalres = i;
		Resistor *startresistor = new_Resistor();
		netzaufbau(startresistor, i - 1); // Die Funktion fuer den Aufbau des Bauplans wird aufgerufen.
		if (count(bestcombi) != i) // Wenn die Zahl der verwendeten Widerstaende in der besten Kombination nicht erhoeht wurde, bringt eine Verwendung von mehr Widerstaenden keinen Vorteil.
			printf("Eine Kombination mit %d Widerstaenden bringt keinen Vorteil.\n", i);
		else
		{
			printf("Die beste Kombination mit %d Widerstaenden ist folgende:\n", i); // Ansonsten wird die Kombination ausgedruckt.
			printway2(bestcombi);
			double erg = bestcombi->resistance;
			double differenz = angestrebt - erg;
			if (differenz < 0) differenz = -differenz;
			printf("\n'-> %lf. Das bedeutet eine Differenz von nur %lf\n\n", erg, differenz);
			if (i != MAXres && differenz == 0) // Wenn bereits eine Abweichung von genau 0 erreicht wurde, ist ein weiteres Rechnen mit mehr Widerstaenden nicht noetig.
			{
				printf("Ich muss nicht weiterrechnen.\n");
				break;
			}
		}
        remove_resistor(startresistor);
	}
	printf("Das Programm ist beendet.\n");
	return 0;
}
