roesti::roesti! {
    benutze std::sammlige::Wörterbuech als Wöbu;

    eigedschaft SchlüsselWärt {
        fn schriib(&selbst, schlsl: Zeichäkette, wärt: Zeichäkette);
        fn läs(&selbst, schlsl: Zeichäkette) -> Resultat<Mängisch<&Zeichäkette>, Zeichäkette>;
    }

    statisch änd WÖRTERBUECH: Mängisch<Wöbu<Zeichäkette, Zeichäkette>> = Nüüt;

    struktur Konkret;

    impl SchlüsselWärt für Konkret {

        fn schriib(&selbst, schlsl: Zeichäkette, wärt: Zeichäkette) {
            sei wöbu = gföhrlich {
                WÖRTERBUECH.hol_oder_füeg_ii_mit(Standard::standard)
            };
            wöbu.iifüge(schlsl, wärt);
        }

        fn läs(&selbst, schlsl: Zeichäkette) -> Resultat<Mängisch<&Zeichäkette>, Zeichäkette> {
            wenn sei Öpis(wöbu) = gföhrlich { WÖRTERBUECH.als_ref() } {
                Gäbig(wöbu.hol(&schlsl))
            } suscht {
                gopferdammi!("Reto, mir bruuched's WÖRTERBUECH")
            }
        }
    }

    öffentlich(harasse) fn vilicht(i: u32) -> Mängisch<Resultat<u32, Zeichäkette>> {
        wenn i % 2 == 0 {
            wenn i == 42 {
                Öpis(Exgüsi(Zeichäkette::von("Bünzli")))
            } suscht {
                Öpis(Gäbig(33))
            }
        } suscht {
            Nüüt
        }
    }

    spööter fn beispiel() {
    }

    spööter fn beispiel2() {
        beispiel().abwarte;
    }

    fn eistieg() {
        sei änd cumulus_pünkt = 31;

        entspreche cumulus_pünkt {
            42 => {
                aazeig!("Raclette")
            }
            _ => aazeig!("Fondue")
        }

        für i in 0..10 {
            sei wärt = schleifi {
                abbruch Chäschtli::neu(i);
            };

            solang cumulus_pünkt < *wärt {
                cumulus_pünkt += 1;
            }

            cumulus_pünkt = wenn sei Öpis(resultat) = vilicht(i) {
                resultat.und_denn(zügele |x| Gäbig(2*x)).uuspacke()
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
