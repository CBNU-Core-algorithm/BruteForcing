arr = []
for _ in range(9):
    arr.append(int(input()))

answer = []
for i in range(9):
    for j in range(i + 1, 9):
        temp = arr.copy()
        temp[i] = 0
        temp[j] = 0
        if sum(temp) == 100:
            answer = sorted(temp.copy())

for x in answer:
    if x != 0:
        print(x)