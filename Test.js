const factorialMemo = {};
function factorial(n) {
    if (n <= 1) {
      return 1;
    } else if (factorialMemo[n]) {
      return factorialMemo[n];
    } else {
      const result = n * factorial(n - 1);
      factorialMemo[n] = result;
      return result;
    }
  }
  function combinations(n, r) {
    // memoization data structure for combinations
    const memo = {};

    function memoizedCombinations(n, r) {
      const memoKey = `${n},${r}`;
      if (memo[memoKey]) {
        return memo[memoKey];
      } else {
        const result = factorial(n) / (factorial(r) * factorial(n - r));
        memo[memoKey] = result;
        return result;
      }
    }

    return memoizedCombinations(n, r);
  }
  console.log(combinations(6, 5));