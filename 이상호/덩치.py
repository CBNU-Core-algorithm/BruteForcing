# 사람 명 수 입력 받기
n = int(input())

# 덩치 리스트
people = []

for i in range(n):
    people.append(list(map(int,input().split())))

for i in range(n):
    rank = 1
    for j in range(n):
        # 몸무게가 더 큰 사람 있으면
        if people[j][0] > people[i][0]:
           # 키까지 비교
           if people[j][1] > people[i][1]:
              # 키까지 더 큰 사람 있으면
              rank += 1
    print(rank)

