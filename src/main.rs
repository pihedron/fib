use malachite::{
    Natural,
    base::num::{
        arithmetic::traits::Square,
        basic::traits::{One, Two, Zero},
        conversion::traits::ExactFrom,
    },
};
use std::{error::Error, io, time::Instant};

const FIVE: Natural = Natural::const_from(5);

fn fib_luc(n: Integer) -> (Integer, Integer) {
    if n == Integer::ZERO {
        return (Integer::ZERO, Integer::TWO);
    }

    if &n < &Integer::ZERO {
        let (fib, luc) = fib_luc(-&n);
        let k = ((-n & Integer::TWO) << 1) - Integer::TWO;
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

fn main() -> Result<(), Box<dyn Error>> {
    let mut number = String::new();
    io::stdin().read_line(&mut number)?;
    let number = number.trim().parse::<isize>()?;

    let start = Instant::now();
    let (fib, _) = fib_luc(Natural::exact_from(number));
    let elapsed = start.elapsed();
    //println!("{}", fib);
    println!("{:?}", elapsed);

    Ok(())
}
