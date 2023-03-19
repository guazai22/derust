fn f() {
    (
        match x {
            | 1 => 123,
        },
        match x {
            | some(x) => 123,
            | _ => 223,
        },
        match x {
            | Some(x) => {
                print(1);
                123
            },
            | _ => 423,
        },
        match x {
            | Some(x) => {
                print(1);
                123
            },
            | Ok(x) => 423,
            | _ => 523,
        },
    )
}
