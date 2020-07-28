import math
n = 1000

done = False
for b in range(1,n):
	for a in range(1,b):
		c = math.sqrt(a*a+b*b)
		if round(c) == c and a+b+c == n:
			done = True
			print('('+str(a)+','+str(b)+','+str(c)+')')
			print(a*b*c)
			break
	if done:
		break
	
