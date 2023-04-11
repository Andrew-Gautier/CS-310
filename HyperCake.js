// This program was written by Andrew Gautier
function hypercake(n, k) {
  // memoization data structure for factorial
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
const sums = {};
  let sum = 0;
  for (let i = 0; i <= k; i++) {
    sum += combinations(n, i);  // n choose i
    // memoize the sum
    sums[`${n},${i}`] = sum;
  }
  return sum;
 
}
console.log(hypercake(8, 5));
// prompt the user for input and print the result to the console
const readline = require('readline').createInterface({
  input: process.stdin,
  output: process.stdout
});
readline.question('Enter n and k: ', (input) => {
  const [n, k] = input.split(' ').map(Number);
  console.log(hypercake(n, k));
  readline.close();
}
);