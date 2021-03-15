def fib(n):
    if (n <= 1):
        return n
    return fib(n - 2) + fib(n - 1);

for n in range(30):
    print(str(n) + ": " + str(fib(n)))
