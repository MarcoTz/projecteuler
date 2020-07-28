n = 4000000
fib = [1,1]
sum = 0
i = 0
while fib[-1]<n:
	fibi = fib[i] + fib[i+1]
	fib.append(fibi)
	if fibi%2 == 0:
		sum += fibi
	i = i+1
print(sum)

