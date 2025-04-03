use anyhow::anyhow;
use malachite::{
    Integer,
    base::num::{
        arithmetic::traits::Square,
        basic::traits::{One, Two, Zero},
        conversion::traits::{FromSciString, FromStringBase},
    },
};
use std::{env, io, io::Write, time::Instant};

fn input(prompt: &str) -> anyhow::Result<String> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut n = String::new();
    io::stdin().read_line(&mut n)?;
    Ok(n.trim().to_string())
}

fn starts_with(string: &str, prefixes: &[&str]) -> bool {
    prefixes.iter().any(|&prefix| string.starts_with(prefix))
}

fn bigint(n: String) -> anyhow::Result<Integer> {
    let parsed = if starts_with(&n, &["0x", "0X"]) {
        Integer::from_string_base(16, &n[2..])
    } else {
        Integer::from_sci_string(&n)
    };

    parsed.ok_or_else(|| anyhow!("Invalid integer format"))
}

const FIVE: Integer = Integer::const_from_unsigned(5);

fn fib_luc(n: Integer) -> (Integer, Integer) {
    if n == Integer::ZERO {
        return (Integer::ZERO, Integer::TWO);
    }

    if &n < &Integer::ZERO {
        let k = ((-&n & Integer::ONE) << 1) - Integer::ONE;
        let (fib, luc) = fib_luc(-n);
        return (fib * &k, luc * k);
    }

    if &n & &Integer::ONE == 1 {
        let (fib, luc) = fib_luc(n - Integer::ONE);
        return ((&fib + &luc) >> 1, (FIVE * &fib + &luc) >> 1);
    }

    let (fib, luc) = fib_luc(&n >> 1);
    (
        &fib * &luc,
        luc.square() + ((n & Integer::TWO) << 1) - Integer::TWO,
    )
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let print_all = args.contains(&"--print".to_string());
    let print_fib = args.contains(&"--fib".to_string());
    let print_lucas = args.contains(&"--lucas".to_string());

    // Display description before prompt
    println!("This program computes the Fibonacci and Lucas numbers of a given integer.");

    let number = bigint(input("Enter a number: ")?)
        .map_err(|_| anyhow!("Unable to parse integer"))?;

    let start = Instant::now();
    let (fib, luc) = fib_luc(number.clone());
    let elapsed = start.elapsed();

    if print_all {
        println!("Fibonacci:\t{}", fib);
        println!("Lucas:\t\t{}", luc);
    } else {
        if print_fib {
            println!("Fibonacci:\t{}", fib);
        }
        if print_lucas {
            println!("Lucas:\t\t{}", luc);
        }
    }

    println!("Elapsed time:\t{:?}", elapsed);

    Ok(())
}
