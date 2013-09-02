N = 100000000 
nsqrt = N^(0.5)

is_prime = {}

--Initialize array
for i=1, N do
  is_prime[i] = false
end

for x = 1, nsqrt do
  for y = 1, nsqrt do
    n = 4 * (x^2) + y^2
    if n <= N and (n % 12 == 1 || n % 12 == 5) then
      is_prime[n] = !is_prime[n]
    end
    n = 3 * (x^2) + y^2
    if n <= N and n % 12 == 7 then
      is_prime[n] = !is_prime[n]
    end
    n = 3 * (x^2) - y^2
    if x > y and n <= N and n % 12 == 11 then
      is_prime[n] = !is_prime[n]
    end
  end
end

for n = 5, nsqrt do
  if is_prime[n] then
    for y = n^2, N, n^2 do
      is_prime[y] = false
    end
  end
end

is_prime[2] = true
is_prime[3] = true
primes = {}

for x = 0, #is_prime - 1 do
  if is_prime[x] then
    table.insert(primes, x)
  end
end
