s1,s2,s3 = map(int,input().split())
fq = [0 for _ in range(100)]
many = []
sum = 0

for i in range(1,s1+1):
    for j in range(1,s2+1):
        for k in range(1,s3+1):
            fq[i+j+k] += 1

frequency = max(fq)

for i in range(100):
    if fq[i] == frequency:
       many.append(i)

print(min(many))