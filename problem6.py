n = 100

squaresum = 0
sumsquare = 0

for i in range(0,n+1):
	squaresum += i*i
	sumsquare += i
sumsquare = sumsquare*sumsquare

print(sumsquare-squaresum)
