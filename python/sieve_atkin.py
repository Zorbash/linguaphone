import sys
import math

args = sys.argv

N = len(args) > 1 and int(sys.argv[1]) or 1000
nsqrt = int((math.pow(N, 0.5)))
is_prime = [False] * N

for x in xrange(1, nsqrt):
    for y in xrange(1, nsqrt):
        n = 4 * (x * x) + (y * y)
        if n <= N and n % 12 in [1,5]:
            is_prime[n] = True

        n = 3 * (x * x) + (y * y)
        if n <= N and n % 12 is 7:
            is_prime[n] = True

        n = 3 * (x * x) - (y * y)
        if n <= N and x > y and n % 12 is 11:
            is_prime[n] = True

is_prime[2] = True
is_prime[3] = True

primes = []
for k, v in enumerate(is_prime):
    v and primes.append(k)

print(len(primes))
