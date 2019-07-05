#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define MAXKANTE 100 // Hier wird die maximale Kantenlaenge fuer das auszuprobierende Gebiet definiert.
#define MAXGRUND 10  // Hier wird die maximale Zahl an setzbaren Grundstuecken festgelegt
#define true 't'     // Weil es in C keine standardmaessigen Booleans gibt, muss dieser Datentyp erst definiert werden.
#define false 'f'
typedef char bool;

int flaeche[MAXKANTE][MAXKANTE]; // Hier wird der 2D-Array definiert, in dem die Grundstuecke angeordnet werden.

int anzahlgrund;
int xkante;
int ykante;
int gesamtflaeche;

typedef struct Wunschstueck
{
    int x;
    int y;
    int grundstuecksflaeche;
} Wunschstueck;

Wunschstueck *liste[MAXGRUND];

Wunschstueck *new_stueck(int ux, int uy) // In dieser Struktur werden die wichtigen Eigenschaften der Grundstuecke gespeichert. Der Array in der oberen Zeile speichert die Zeiger auf diese Grundstuecke.
{
	Wunschstueck *mystueck = (Wunschstueck *)malloc(sizeof(Wunschstueck));
	mystueck->x = ux;
	mystueck->y = uy;
	mystueck->grundstuecksflaeche = ux * uy;
	return mystueck;
}

void init_flaeche() // Diese Funktion bereitet die Flaeche so vor, dass sie fuer das Ausprobieren nutzbar ist, indem ueberall -1 hineingeschrieben wird
{
	for (int i = 0; i < MAXKANTE; i++)
	{
		for (int j = 0; j < MAXKANTE; j++)
			flaeche[i][j] = -1;
	}
}

void calc_total_area() // Diese Funktion summiert die Einzelflaechen der Grundstuecke
{
	for (int i = 0; i < anzahlgrund; i++)
		gesamtflaeche += liste[i]->grundstuecksflaeche;
}

void printflaeche() // Diese Funktion stellt die Anordnung grafisch dar.
{
	for (int i = 0; i < ykante; i++)
	{
		for (int j = 0; j < xkante; j++)
		{
			if (flaeche[j][i] == -1)
				printf(" .");
			else
				printf("%2d", flaeche[j][i]);
		}
		printf("\n");
	}
}

bool reserve(int depth, int x, int y) // Dise Funktion reserviert den Platz fuer ein Grundstueck
{
	if (x + liste[depth]->x > xkante || y + liste[depth]->y > ykante) // Wenn das Grundstueck den benutzbaren Bereich verlaesst, ist dieser Platz fuer diesen Garten ungueltig.
		return false;
	for (int i = 0; i < liste[depth]->y; i++)
	{
		for (int j = 0; j < liste[depth]->x; j++)
		{
			if (flaeche[x + j][y + i] != -1) // Wenn an einer Stelle, an der der Platz reserviert werden soll, bereits ein Garten geplant wird, ist der Platz ebenfalls ungueltig.
				return false;
		}
	}
	for (int i = 0; i < liste[depth]->y; i++)
	{
		for (int j = 0; j < liste[depth]->x; j++)
			flaeche[x + j][y + i] = depth; // Hier wird der Platz reserviert.
	}
	return true;
}

void reset(int depth, int x, int y) // Diese Funktion loescht das Grundstueck der Nummer "depth" wieder von der Flaeche
{
	for (int i = 0; i < liste[depth]->y; i++)
	{
		for (int j = 0; j < liste[depth]->x; j++)
		{
			flaeche[x + j][y + i] = -1;
		}
	}
}

bool ausprobieren(int depth)
{
	for (int i = 0; i < xkante; i++) // Diese verschachtelte Zaehlschleife probiert alle denkbaren Positionen auf dem Feld aus.
	{
		for (int j = 0; j < ykante; j++)
		{
			for (int z = 0; z < 2; z++)
			{
				if (z == 1) // Anhand des Wertes der inneren Zaehlschleife wird bestimmt, ob das Grundstueck gedreht wird, um diese Moeglichkeiten auch zu beruecksichtigen.
				{
					int hilf = liste[depth]->x;
					liste[depth]->x = liste[depth]->y;
					liste[depth]->y = hilf;
				}
				if (reserve(depth, j, i) == true) // Der Platz fuer das Grundstueck wird reserviert.
				{
					if (depth + 1 < anzahlgrund)  // Falls noch nicht alle Grundstuecke verteilt wurden, wird mit den Grundstueck der naechsten Stufe fortgefahren
					{
						if (ausprobieren(depth + 1) == true)
						{
							reset(depth, j, i);
							return true;
						}
					}
					else                          // Andernfalls wurden bereits alle Grundstuecke gueltig verteilt. Dann ist die Aufgabe geloest!
					{
						printflaeche();
						reset(depth, j, i);
						return true;
					}
					reset(depth, j, i); // Nach Abschluss des Durchlaufes muss die Reservierung wieder geloescht werden.
				}
			}
		}
	}
	return false;
}

int teilersuchen()
{
	int teiler[MAXKANTE]; // In diesem Array wird die Anzahl der Grundstuecksausmasse gefuehrt, die durch den Wert des jeweiligen Elements teilbar sind.
	for (int i = 0; i < MAXKANTE; i++)
		teiler[i] = 0;
	for (int i = 0; i < anzahlgrund; i++) // Die aeussere Zaehlschleife prueft zaehlt alle Grundstuecke durch.
	{
		for (int j = 0; j < 2; j++)                           // Diese Zaehlschleife zaehlt nur von 0 bis 1.
		{
			int test = (j == 0) ? liste[i]->x : liste[i]->y;  // Mit den Werten aus dieser mittleren Zaehlschleife wird bestimmt, ob die x- oder y-Variable auf Teilbarkeit geprueft wird.
			for (int k = 2; k <= test; k++) // Alle Zahlen ab zwei bis zum Wert werden auf Teilbarkeit geprueft
			{
				if (test % k == 0)
					teiler[k]++;
			}
		}
	}
	int groessterteiler = 1;
	for (int i = 2; i < MAXKANTE; i++)
	{
		if (teiler[i] == anzahlgrund * 2)
			groessterteiler = i;
	}
	return groessterteiler;
}

void teilen(int teiler) // Diese Funktion teilt die Kantenlaengen der Grundstuecke durch ihren gemeinsamen Teiler, falls dieser ungleich 1 ist.
{
	for (int i = 0; i < anzahlgrund; i++)
	{
		liste[i]->x /= teiler;
		liste[i]->y /= teiler;
		liste[i]->grundstuecksflaeche = liste[i]->x * liste[i]->y;
	}
}

bool flaechen_ausprobieren()
{
	for (int i = gesamtflaeche; i < (MAXKANTE * MAXKANTE); i++) // Diese aeussere Zaehlschleife erhoeht die Flaeche immer um 1, falls zuvor keine gueltige Anordnung erreicht werden konnte.
	{
		for (int j = (int)sqrt(i); j > 0; j--) // Diese innere Zaehlschleife beginnt bei der Wurzel der Flaeche, um Teilerpaare zu suchen.
		{
			if (i % j == 0) // Fuer die auszuprobierende Flaeche werden ganzzahlige Teiler benoetigt. Nur wenn die Division
			{               // der Flaeche mit dem ersten Teiler keinen Rest uebrig laesst, ist das Wertepaar geeignet.
				xkante = j;
				ykante = i / j;
				if (ausprobieren(0) == true) // Mit dem Wertepaar fuer die Flaeche soll eine Kombination ausprobiert werden.
				{                            // Wenn eine Kombination moeglich ist, wurde in der Funktion bereits das Ergebnis ausgedruckt. Die Daten folgen hier:
					printf("Diese Loesung hat eine Flaeche von %d bei Kantenlaengen von %d und %d. Das bedeutet eine Effizienz von %3.2lf%c, weil die minimale Flaeche %d gewesen waere.\n", i, xkante, ykante, ((double)gesamtflaeche / (double)i) * 100, (char)37, gesamtflaeche);
					return true;
				}
			}
		}
	}
	return false;
}

int main()
{
	// Anzahl und Abmessungen der Gaerten werden erfragt
	printf("Wie viele Gaerten sollen angelegt weden? (maximal %d)\n", MAXGRUND);
	scanf("%d", &anzahlgrund);
	for (int i = 0; i < anzahlgrund; i++)
	{
		printf("Welche Abmessungen soll der naechste Garten mit der Nummer %d haben? (x*y)\n", i);
		int x = 0;
		int y = 0;
		scanf("%d*%d", &x, &y);
		liste[i] = new_stueck(x, y);
	}
	int groessterteiler = teilersuchen(); // Groessten gemeinsamen Teiler suchen, teilen und Meldung ausgeben
	if (groessterteiler != 1)
	{
		printf("Man kann diese Gaertenlaengen durch %d dividieren, damit weniger zu rechnen ist.\nDer Ausdruck ist um diesen Maßstabsfaktor verkleinert.\nBitte beachten Sie, dass sich die Angaben ueber die Flaeche zum Quadrat dieses Faktors verkleinern.\n", groessterteiler);
		teilen(groessterteiler);
	}
	init_flaeche(); // globaler 2D-Array der fuer das Ausprobieren wichtigen Flaeche wird so initialisiert, dass ueberall -1, also frei eingetragen wird.
	calc_total_area(); // Gesamtflaeche wird als Summe der Einzelflaechen berechnet und in die globale Variable "gesamtflaeche" geschrieben.
	bool test = flaechen_ausprobieren(); // fuer Funktionsweise: siehe Funktion
	printf("%s", (test == true) ? "Das Programm ist beendet.\n" : "Es ist ein Fehler aufgetreten");
	return 0;
}