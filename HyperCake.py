# def hypercake(n, k):
#     memo = {}

#     def combinations(n, r):
#         def factorial(n):
#             if n == 0:
#                 return 1
#             elif n in memo:
#                 return memo[n]
#             else:
#                 memo[n] = n * factorial(n-1)
#                 return memo[n]

#         if r > n:
#             return 0
#         else:
#             return factorial(n) // (factorial(r) * factorial(n-r))

#     if k == 0:
#         return 1
#     elif n == 0 or k > n:
#         return 0
#     else:
#         total = 0
#         for r in range(k+1):
#             total += combinations(n, r) * hypercake(n-r, k-r)
#         return total
# print(hypercake(5, 3))
def hypercake(n, k):
    memo = [[0]*(k+1) for _ in range(n+1)]
    memo[0][0] = 1

    for i in range(1, n+1):
        for j in range(1, k+1):
            for r in range(j+1):
                memo[i][j] += memo[i-1][r] * binom(j, r) * hypercake(i-r, j-r)
    
    return memo[n][k]

def binom(n, k):
    res = 1
    for i in range(1, k+1):
        res = res * (n-i+1) // i
    return res
print(hypercake(5, 3))