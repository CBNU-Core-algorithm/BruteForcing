import sys
input = sys.stdin.readline

a, b = map(int, input().strip().split())
ans = 0

def DFS(n):
    global ans
    if n > b:
        return
    if n >= a:
        ans += 1

    for i in [4, 7]:
        DFS(10*n + i)

DFS(0)
print(ans)
