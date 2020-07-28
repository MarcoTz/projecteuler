ndigits = 3
def palindrome(n):
	return (str(n)[::-1] == str(n))

factors = list(range(10**(ndigits-1), 10**ndigits))
factors.reverse()
palindromes = []
pfactors = {}

for i in factors:
	for j in factors:
		candidate = i*j
		if palindrome(candidate):
			palindromes.append(candidate)	
			pfactors[candidate] = (i,j)
palindromes.sort()
res = palindromes[-1]
print(res)
print(pfactors[res])
