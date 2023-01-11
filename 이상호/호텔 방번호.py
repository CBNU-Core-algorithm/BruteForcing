while 1:
    try:
        count = 0
        a, b = map(int, input().split())
        # 0~9
        num = [0 for i in range(10)]

        for i in range(a, b + 1):
            if i >= 1000:
                num[i // 1000] += 1
                num[(i % 1000) // 100] += 1
                num[(i % 100) // 10] += 1
                num[i % 10] += 1

            elif i >= 100:
                num[(i % 1000) // 100] += 1
                num[(i % 100) // 10] += 1
                num[i % 10] += 1

            else:
                num[(i % 100) // 10] += 1
                num[i % 10] += 1

            if max(num) < 2:
                count += 1

            num = [0 for i in range(10)]

        print(count)
    except:
        break