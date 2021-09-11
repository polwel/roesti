rost::rost! {
    benutze std::collections::Wörterbuch als Wöbu;

    vereinbarung SchlüsselWert {
        funktion schreibe(&selbst, schlsl: Zeichenkette, wert: Zeichenkette);
        funktion lese(&selbst, schlsl: Zeichenkette) -> Ergebnis<Möglichkeit<&Zeichenkette>, Zeichenkette>;
    }

    statisch änd WÖRTERBUCH: Möglichkeit<Wöbu<Zeichenkette, Zeichenkette>> = Nichts;

    struktur Konkret;

    umstz SchlüsselWert für Konkret {

        funktion schreibe(&selbst, schlsl: Zeichenkette, wert: Zeichenkette) {
            lass wöbu = gefährlich {
                WÖRTERBUCH.hole_oder_füge_ein_mit(Standard::standard)
            };
            wöbu.einfügen(schlsl, wert);
        }

        funktion lese(&selbst, schlsl: Zeichenkette) -> Ergebnis<Möglichkeit<&Zeichenkette>, Zeichenkette> {
            wenn lass Etwas(wöbu) = gefährlich { WÖRTERBUCH.als_ref() } {
                Gut(wöbu.hole(&schlsl))
            } anderenfalls {
                Fehler("Holt das Wörterbuch".hinein())
            }
        }
    }

    öffentlich(kiste) funktion vielleicht(i: u32) -> Möglichkeit<Ergebnis<u32, Zeichenkette>> {
        wenn i % 2 == 1 {
            wenn i == 42 {
                Etwas(Fehler(Zeichenkette::von("Scheiße")))
            } anderenfalls {
                Etwas(Gut(33))
            }
        } anderenfalls {
            Nichts
        }
    }

    asynchron funktion beispiel() {
    }

    asynchron funktion beispiel2() {
        beispiel().erwarte;
    }

    funktion einstieg() {
        lass änd x = 31;

        entspreche x {
            42 => {
                ausgabe!("Wienerschnitzel")
            }
            _ => ausgabe!("Na geht doch")
        }

        für i in 0..10 {
            lass val = schleife {
                abbruch i;
            };

            während keins x < val {
                x += 1;
            }

            x = wenn lass Etwas(ergebnis) = vielleicht(i) {
                ergebnis.entpacken()
            } anderenfalls {
                12
            };
        }
    }
}
