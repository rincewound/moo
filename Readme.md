# moo - yet another terminal editor

## Why
Mostly just idle curiosity on TUIs

## Usage

* Insert Mode - Bearbeiten
    [x] CTRL + <-/-> prev/next buffer
* Normal
	[x] Öffnen (o)
	[x] Speichern (w)
	[x] Schließen (c)
	[x] Neuer Buffer (n)
	[x] Buffer wechseln (b)
	[x] Buffer benennen (a)
* Navigation (?)
	[x] Wort überspringen (w)
	[x] Zeilenende (e)
	[x] Zeilenanfang (b)
	[x] Hoch/ Runter (u/d)
	[x] Scrollen
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


Neue Keymap

Nav:
Buchstabe vor: j
Wort vor: k
Zeilenende: l

Zeilenanfang: s
Wort zurück: d
Buchstabe zurück: f

Zeile runter: n
seite runter: m
dok ende: ,

Zeile hoch: v
seite hoch: c
dok anfang: x

+ Navigation ohne Moduswechsel, also mit modifier key?

-> CTRL + oben für navigation
-> CTRL Q/W/E für
	Normal
	Select
	Insert
	
## Planned Features
* New File with paths
* LSP Client ... maybe

## ToDo:
* Improve keymap
* better fuzzy open - inlcuding subdirectories
* Edit on open
* Cmdline open

## Bugs
* Tabs not rendered correctly