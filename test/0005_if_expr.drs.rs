fn f() {
    f(
        if true {
            123
        } else if true {
            123
        } else {
            123
        },
        if true {
            123
        } else if true {
            123
        },
        if true {
            123
        },
        if true {
            123
        } else if true {
            123
        } else {
            123
        },
        if f(true) {
            123
        } else if true {
            123
        } else {
            123
        },
        if f(true) {
            123
        } else if true {
            123
        } else {
            123
        },
    )
}
