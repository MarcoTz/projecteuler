n = 1000
mult = []
sum = 0
for i in range(0,n): 
	if i%3==0 or i%5==0:
		mult.append(i)
		sum += i
print(sum)
