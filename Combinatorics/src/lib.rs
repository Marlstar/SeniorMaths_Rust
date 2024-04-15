pub fn factorial(n: usize) -> usize {
    let mut total: usize = 1;
    let mut current: usize = n;
    while current > 0 {
        total *= current;
        current -= 1;
    }
    return total;
}

pub fn permutation(n: usize, r: usize) -> usize {
    return factorial(n) / factorial(n-r)
}

pub fn combination(n: usize, r: usize) -> usize {
    return permutation(n, r) / factorial(r);
}