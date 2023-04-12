// This was pair programmed by Andrew Gautier and Jay Bhardwaj
// This program calculates the number of hypercakes for a given number of dimensions and cuts
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_hypercake() {
      
        assert_eq!(hypercake(0, 2), 1);
        assert_eq!(hypercake(0, 3), 1);
        assert_eq!(hypercake(0, 4), 1);
        assert_eq!(hypercake(0, 5), 1);
        assert_eq!(hypercake(1, 2), 2);
        assert_eq!(hypercake(1, 3), 2);
        assert_eq!(hypercake(1, 4), 2);
        assert_eq!(hypercake(1, 5), 2);
        assert_eq!(hypercake(2, 2), 4);
        assert_eq!(hypercake(2, 3), 4);
        assert_eq!(hypercake(2, 4), 4);
        assert_eq!(hypercake(2, 5), 4);
        assert_eq!(hypercake(3, 2), 7);
        assert_eq!(hypercake(3, 3), 8);
        assert_eq!(hypercake(3, 4), 8);
        assert_eq!(hypercake(3, 5), 8);
        assert_eq!(hypercake(4, 2), 11);
        assert_eq!(hypercake(4, 3), 15);
        assert_eq!(hypercake(4, 4), 16);
        assert_eq!(hypercake(4, 5), 16);
        assert_eq!(hypercake(5, 2), 16);
        assert_eq!(hypercake(5, 3), 26);
        assert_eq!(hypercake(5, 4), 31);
        assert_eq!(hypercake(5, 5), 32);
        assert_eq!(hypercake(6, 2), 22);
        assert_eq!(hypercake(6, 3), 42);
        assert_eq!(hypercake(6, 4), 57);
        assert_eq!(hypercake(6, 5), 63);
        assert_eq!(hypercake(7, 2), 29);
        assert_eq!(hypercake(7, 3), 64);
        assert_eq!(hypercake(7, 4), 99);
        assert_eq!(hypercake(7, 5), 120);
        assert_eq!(hypercake(8, 2), 37);
        assert_eq!(hypercake(8, 3), 93);
        assert_eq!(hypercake(8, 4), 163);
        assert_eq!(hypercake(8, 5), 219);
        assert_eq!(hypercake(9, 2), 46);
        assert_eq!(hypercake(9, 3), 130);
        assert_eq!(hypercake(9, 4), 256);
        assert_eq!(hypercake(9, 5), 382);
        assert_eq!(hypercake(10, 2), 56);
        assert_eq!(hypercake(10, 3), 176);
        assert_eq!(hypercake(10, 4), 386);
        assert_eq!(hypercake(10, 5), 638);
        
    }
}
