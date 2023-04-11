
// Factorial function
fn factorial(n: i64) -> i64 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// Combinations function
fn combinations(n: i64, r: i64) -> i64 {
    if r > n {
        0
    } else {
        factorial(n) / (factorial(r) * factorial(n - r))
    }
}

// Hypercake function
fn hypercake(n: i64, k: i64) -> i64 {
    if k == 1 {
        n
    } else if n == 1 {
        1
    } else {
        let mut result = 0;
        for i in 0..=k {
            result += combinations(n,i);
        }
        result
    }
}
fn main() {
   
    println!("Enter n (number of dimensions) and k (number of cuts) separated by space");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n = iter.next().unwrap().parse::<i64>().unwrap();
    let k = iter.next().unwrap().parse::<i64>().unwrap();
    println!("Hypercake for n={} and k={} is {}", n, k, hypercake(n, k));
    
}

