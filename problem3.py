import math
n = 600851475143

def prime(i):
	if i <= 1:	
		return False

	for j in range(2,i-1):
		if i%j == 0:
			return False
	
	return True

primefac = []
for i in range(2,n):
	if n%i == 0:
		print(i)
		if prime(i):
			primefac.append(i)
			while n%i == 0:
				n = n/i

	if n==1:
		break

print(primefac[-1])
