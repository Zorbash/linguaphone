static N: uint = 100000000;
fn main() {
  stieve_atkin();
}

fn stieve_atkin() {
  let nsqrt: uint = ((N as float).sqrt() + 1.0) as uint;
  let mut is_prime = ~[false, ..N];
  let mut n: uint;
  let mut primes = ~[];

  for x in range(1, nsqrt) {
    for y in range(1, nsqrt) {
      n = 4 * (x*x) + (y*y);
      if n <= N && (n % 12 == 1 || n % 12 == 5) { is_prime[n] = !is_prime[n]; }

      n = 3 * (x*x) + y*y;
      if n <= N && n % 12 == 7 { is_prime[n] = !is_prime[n]; }

      n = 3 * (x*x) - y*y;
      if x > y && n <= N && n % 12 == 11 { is_prime[n] = !is_prime[n]; }
    }
  }

  for x in range(5, nsqrt) {
    if is_prime[x] {
      let mut y = x*x;
      while y < N {
        is_prime[y] = false;
        y += x*x;
      }
    }
  }

  is_prime[2] = true;
  is_prime[3] = true;

  for x in range(0, N) { if is_prime[x] { primes.push(x) } }
}
