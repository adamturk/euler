/// Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:
/// 	1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...

#[test]
fn test(){
	let expected = vec![1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
	let result: Vec<u32> = Fibonacci::new().take(10).collect();
	assert_eq!(result, expected);
}

struct Fibonacci {
    last: u32,
    current: u32
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci {
            last: 0,
            current: 1
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let temp = self.last;
        self.last = self.current;
        self.current = self.last + temp;
        Some(self.current)
    }
}

fn sum_upto(summands: Fibonacci, max: u32) -> u32 {
    let mut sum = 0;
    for summand in summands {
        if summand >= max {
            break;
        }
        if summand % 2 == 0 && summand < max {
            sum += summand;
        }
    }
	sum
}

/// By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.

fn main() {
	let generator = Fibonacci::new();
    let sum = sum_upto(generator, 4_000_000);
	println!("{}", sum);
}