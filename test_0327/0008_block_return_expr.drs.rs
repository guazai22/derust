fn f() {
    (
        loop {
            return a;
        },
        loop {
            println(a);
            return a;
        },
        loop {
            println(a);
            return a;
        },
    )
}
