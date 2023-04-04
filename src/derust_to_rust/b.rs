pub fn to_rust_function(ident: &str) -> &str {
    match ident {
        | "println" => "println!",
        | "print" => "print!",
        | _ => ident,
    }
}
