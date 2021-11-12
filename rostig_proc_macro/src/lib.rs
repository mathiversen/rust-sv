use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Nej" => "Err",
        "Ja" => "Ok",
        "Text" => "String",
        "Lexikon" => "HashMap",
        "Standard" => "Default",
        "Fel" => "Error",
        "Alternativ" => "Option",
        "Något" => "Some",
        "Inget" => "None",
        "Utfall" => "Result",
        "Själv" => "Self",
        "skrivrad" => "println",
        "avbryt" => "break",
        "asynk" => "async",
        "vänta" => "await",
        "upprepa" => "loop",
        "flytta" => "move",
        "paket" => "crate",
        "omöjligt" => "unreachable_code",
        "som" => "as",
        "varaktig" => "const",
        "egenskap" => "trait",
        "osäker" => "unsafe",
        "i" => "in",
        "från" => "from",
        "rörlig" => "dyn",
        "öppna" => "unwrap",
        "standard" => "default",
        "som_ref" => "as_ref",
        "io" => "io",
        "yttre" => "extern",
        "falskt" => "false",
        "funk" => "fn",
        "över" => "super",
        "infoga" => "insert",
        "hämta" => "get",
        "tillåt" => "allow",
        "vafan" | "jävlar" | "ojsan" => "panic",
        "enhet" => "mod",
        "skift" => "mut",
        "ny" => "new",
        "där" => "where",
        "för" => "for",
        "hämta_eller_sätt_in" => "get_or_insert_with",
        "start" => "main",
        "allmän" => "pub",
        "än" => None?,
        "retur" => "return",
        "verkställ" => "impl",
        "ref" => "ref",
        "jämför" => "match",
        "om" => "if",
        "annars" => "else",
        "själv" => "self",
        "var" => "let",
        "statisk" => "static",
        "data" => "struct",
        "förvänta" => "expect",
        "medan" => "while",
        "använd" => "use",
        "till" => "into",
        "rätt" => "true",
        "lista" => "enum",
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
pub fn oxidera(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
