N = process.argv[2] || 1000
nsqrt = Math.pow N, 0.5

is_prime = []

for x from 1 to nsqrt
  for y from 1 to nsqrt
    continue if n > N
    n = 4 * (x ^ 2) + (y ^ 2)
    is_prime[n] = true if (n % 12 is 1 or n % 12 is 5)
    
    n = 3 * (x ^ 2) + y ^ 2
    is_prime[n] = true if n % 12 is 7

    n = 3 * (x ^ 2) - y ^ 2
    is_prime[n] = true if x > y and n % 12 == 11

primes = is_prime.map((v, k) -> v and k).filter -> it
console.log primes.length
