N = 100000000
is_prime = []
primes = []
nsqrt = Math::sqrt(N).floor

(1..nsqrt).each do |x|
  (1..nsqrt).each do |y|
    n = 4 * (x * x) + y * y
    is_prime[n] = !is_prime[n] if n <= N && (n % 12 == 1 || n % 12 == 5)

    n = 3 * (x * x) + y * y
    is_prime[n] = !is_prime[n] if n <= N && n % 12 == 7

    n = 3 * (x * x) - y * y
    is_prime[n] = !is_prime[n] if x > y && n <= N && n % 12 == 11
  end
end

(5..nsqrt).each do |x|
  next if !is_prime[x]

  y = x * x
  (y..N-1).step(y).each { |i| is_prime[i] = false }
end

is_prime[2] = true
is_prime[3] = true

is_prime.each_with_index { |prime, i| primes << i if prime }
puts primes.count
