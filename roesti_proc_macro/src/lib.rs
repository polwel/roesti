use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Exgüsi" => "Err",
        "Gäbig" => "Ok",
        "Zeichäkette" => "String",
        "Wörterbuech" => "HashMap",
        "Standard" => "Default",
        "Fähler" => "Error",
        "Mängisch" => "Option",
        "Öpis" => "Some",
        "Nüüt" | "Keis" => "None",
        "Resultat" => "Result",
        "Selbst" => "Self",
        "sammlige" => "collections",
        "aazeig" => "println",
        "abbruch" => "break",
        "wiiter" => "continue",
        "spööter" => "async",
        "abwarte" => "await",
        "schleifi" => "loop",
        "zügele" => "move",
        "harasse" => "crate",
        "Chäschtli" => "Box",
        "unerreichbarer_code" => "unreachable_code",
        "als" => "as",
        "konstante" => "const",
        "eigedschaft" => "trait",
        "typ" => "type",
        "gföhrlich" => "unsafe",
        "in" => "in",
        "von" => "from",
        "dynamisch" => "dyn",
        "uuspacke" => "unwrap",
        "standard" => "default",
        "als_ref" => "as_ref",
        "ea" => "io",
        "extern" => "extern",
        "falsch" => "false",
        "huereguet" => "super",
        "iifüge" => "insert",

        // iterator functions
        "numal" => "iter",
        "inne_numal" => "into_iter",
        "zuordnen" => "map",
        "und_denn" => "and_then",
        "falte" => "fold",
        "leere" => "drain",
        "sammle" => "collect",
        "finde" => "find",
        "nimm" => "take",
        "produkt" => "product",

        // ordering
        "vgl" => "cmp",
        "Ordnig" => "Ordering",
        "Höcher" => "Greater",
        "Chliiner" => "Less",
        "Gliich" => "Equal",

        "hol" => "get",
        "erlaube" => "allow",
        "panik" | "gopferdammi" | "ups" => "panic",
        "modul" => "mod",
        "änd" => "mut",
        "neu" => "new",
        "wo" => "where",
        "für" => "for",
        "hol_oder_füeg_ii_mit" => "get_or_insert_with",
        "eistieg" => "main",
        "öffentlich" => "pub",
        "uusschaffe" => "return",
        "ref" => "ref",
        "entspreche" => "match",
        "wenn" => "if",
        "suscht" => "else",
        "selbst" => "self",
        "sei" => "let",
        "statisch" => "static",
        "struktur" => "struct",
        "mol" => "expect",
        "weret" => "while",
        "benutze" => "use",
        "inne" => "into",
        "wahr" => "true",
        "uufzählig" => "enum",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn roesti(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
