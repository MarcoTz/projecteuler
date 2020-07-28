n = 10001
primes = []
k = 2
while len(primes)<n:
	isprime = True
	for p in primes:
		if k%p == 0:
			isprime = False
			break
	if isprime:
		primes.append(k)
	k += 1
print(primes[-1])
	
