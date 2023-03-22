fn f() {
    (
        match x {
            | some(x) => 123,
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
            | Ok(x) => 223,
            | Ok(x) => 323,
            | _ => 423,
        },
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
            | _ => 623,
        },
        match x {
            | Some(x) => {
                print(1);
                123
            },
            | Ok(x) => 423,
            | Ok(x) => 423,
            | _ => 523,
        },
        match x {
            | some(x) => 123,
            | some(x) => 223,
            | _ => 423,
        },
        match x {
            | some(x) => 123,
            | _ => 423,
        },
        match x {
            | some(x) => 123,
            | _ => 423,
        },
    )
}
