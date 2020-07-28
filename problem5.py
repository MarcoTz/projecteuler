n = 20

def prime(n):
	for i in range(2,n):
		if n%i == 0 and n != i:
			return False
	return True

primes = []
for i in range(2,n+1):
	if prime(i):
		primes.append(i)		

primecount = {}
for p in primes:
	primecount[p]=1
	for i in range(1,n):
		k = i
		count = 0
		while k%p == 0:
			count += 1
			k = k/p
		if primecount[p] < count:
			primecount[p] = count
			
res = 1
for p in primes:
	res *= p**primecount[p]
print(res)

