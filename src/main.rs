use anyhow::anyhow;
use malachite::{
    Integer,
    base::num::{
        arithmetic::traits::Square,
        basic::traits::{One, Two, Zero},
    },
};
use std::{io, time::Instant};

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
        return (&fib + &luc >> 1, FIVE * &fib + &luc >> 1);
    }

    let (fib, luc) = fib_luc(&n >> 1);
    (
        &fib * &luc,
        luc.square() + ((n & Integer::TWO) << 1) - Integer::TWO,
    )
}

fn main() -> anyhow::Result<()> {
    let mut number = String::new();
    io::stdin().read_line(&mut number)?;
    let number = number
        .trim()
        .parse::<Integer>()
        .map_err(|_| anyhow!("unable to parse integer"))?;

    let start = Instant::now();
    let (fib, _) = fib_luc(number.clone());
    let elapsed = start.elapsed();
    //println!("{}", fib);
    println!("{:?}", elapsed);

    Ok(())
}
