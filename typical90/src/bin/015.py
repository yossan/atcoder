n = int(input())

mod = 10 ** 9 + 7

fact = [0] * (n + 1)
invfact = [0] * (n + 1)
fact[0] = 1
for i in range(1, n + 1):
    fact[i] = fact[i - 1] * i % mod

invfact[n] = pow(fact[n], mod - 2, mod)
for i in range(n)[::-1]: # 3,2,1,0
    invfact[i] = invfact[i + 1] * (i + 1) % mod

def nCrm(n, r, mod):
    print(f"{n} C {r}")
    if r < 0 or n < r:
        res = 0
    else:
        res = fact[n] * invfact[r]
        res2 = res * invfact[n - r]
    return res2

ans = []
for k in range(1, n + 1):
    cnt = 0
    print(f"k = {k}, ({n} - 1) // {k} + 1 = ", (n-1) // k + 1)
    for j in range((n - 1) // k + 1): # jは何個ずつ選ぶか
        i = n - (k - 1) * j # k-1ずつステップしていく
        cnt += nCrm(i, j + 1, mod)
    ans.append(cnt % mod)
print('\n'.join(map(str, ans)))
