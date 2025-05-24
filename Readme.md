# moo - yet another terminal editor

## Why
Mostly just idle curiosity on TUIs

## Usage

* Insert Mode - Bearbeiten
    * CTRL + <-/-> prev/next buffer
* Normal
	* Öffnen (o)
	* Speichern (w)
	* Schließen (c)
	* Neuer Buffer (n)
	* Buffer wechseln (b)
	* Buffer benennen (a)
* Navigation (?)
	* Wort überspringen (w)
	* Zeilenende (e)
	* Zeilenanfang (b)
	* Hoch/ Runter (u/d)
	* Seite hoch/runter (PgU/PgD)
	* Suchen (f)
* Auswählen
	* Auswahl vergrößern (+)
	* Auswahl vergrößern bis Ende des Worts (.)
	* Auswahl bis Ende der Zeile (-)
	* Auswahl des aktuellen Worts (w)
	* Auswahl kopieren (c)
	* Auswahl ausschneiden (x)
	* Auswahl aufheben (q)
	* (Einfügen an aktueller Stelle) (p)

## Planned Features

* Navigation
* Fuzzy Open File
* New File with paths
* LSP Client ... maybe


## Bugs
* Crash, when line gets too long
* Crash, when writing diacritics (Umlaute)