fn fib(n: u128) -> u128 {
    let mut a = vec![0, 1];
    for i in 0..n + 1 {
        a[0] = a[0] + a[1];
        a[1] = a[0] - a[1];
    }
    a[1]
}
