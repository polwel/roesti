roesti::roesti! {
    benutze std::sammlige::Wörterbuech als Wöbu;

    eigedschaft SchlüsselWärt {
        funktion schriib(&selbst, schlsl: Zeichäkette, wärt: Zeichäkette);
        funktion läs(&selbst, schlsl: Zeichäkette) -> Resultat<Mängisch<&Zeichäkette>, Zeichäkette>;
    }

    statisch änd WÖRTERBUECH: Mängisch<Wöbu<Zeichäkette, Zeichäkette>> = Nüüt;

    struktur Konkret;

    impl SchlüsselWärt für Konkret {

        funktion schriib(&selbst, schlsl: Zeichäkette, wärt: Zeichäkette) {
            sei wöbu = gföhrlich {
                WÖRTERBUECH.hol_oder_füeg_ii_mit(Standard::standard)
            };
            wöbu.iifüge(schlsl, wärt);
        }

        funktion läs(&selbst, schlsl: Zeichäkette) -> Resultat<Mängisch<&Zeichäkette>, Zeichäkette> {
            wenn sei Öpis(wöbu) = gföhrlich { WÖRTERBUECH.als_ref() } {
                Gäbig(wöbu.hol(&schlsl))
            } suscht {
                gopferdämmi!("Reto, mir bruuched's WÖRTERBUECH")
            }
        }
    }

    öffentlich(harasse) funktion vilicht(i: u32) -> Mängisch<Resultat<u32, Zeichäkette>> {
        wenn i % 2 == 1 {
            wenn i == 42 {
                Öpis(Exgüsi(Zeichäkette::von("Bünzli")))
            } suscht {
                Öpis(Gäbig(33))
            }
        } suscht {
            Nüüt
        }
    }

    spööter funktion beispiel() {
    }

    spööter funktion beispiel2() {
        beispiel().abwarte;
    }

    funktion eistieg() {
        sei änd x = 31;

        entspreche x {
            42 => {
                aazeig!("Raclette")
            }
            _ => aazeig!("Fondue")
        }

        für i in 0..10 {
            sei wärt = schleifi {
                abbruch i;
            };

            weret x < wärt {
                x += 1;
            }

            x = wenn sei Öpis(resultat) = vilicht(i) {
                resultat.uuspacke()
            } suscht {
                12
            };
        }

        benutze std::vgl::Ordnig;
        let _mod7 = vec![0; 100].numal()
            .nimm(50)
            .zuordnen(|nummer| nummer %  7)
            .sammle::<Vec<i32>>()
            .inne_numal()
            .falte(0, |a, nummer| entspreche nummer.vgl(&a) {
                Ordnig::Höcher => a - nummer,
                Ordnig::Chliiner => a + nummer,
                Ordnig::Gliich => a,
            });
    }
}
