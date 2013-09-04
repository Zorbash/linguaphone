use std::uint::range;

static N: uint = 5000000;

fn main() {
  stieve_atkin();
}

fn stieve_atkin() {
  let nsqrt: uint = ((N as float).sqrt() + 1.0) as uint;
  let mut n: uint = 0;
  let mut primes = [false, ..N];

  for range(1, nsqrt) |x| {
    for range(1, nsqrt) |y| {
      // println(fmt!("%?", x));
      n = 4 * (x*x) + (y*y);
      if n <= N && (n % 12 == 1 || n % 12 == 5) { primes[n] = !primes[n]; }

      n = 3 * (x*x) + y*y;
      if n <= N && n % 12 == 7 { primes[n] = !primes[n]; }

      n = 3 * (x*x) - y*y;
      if x > y && n <= N && n % 12 == 11 { primes[n] = !primes[n]; }
    }
  }

  for range(5, nsqrt) |x| {
    if primes[x] {
    }
  }
}
