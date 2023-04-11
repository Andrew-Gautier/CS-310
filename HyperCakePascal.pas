program HyperCakePascal;
uses SysUtils;

function factorial(n: Integer): Int64;
begin
  if n <= 1 then
    factorial := 1
  else
    factorial := n * factorial(n - 1);
end;

// OLD factorial function
// begin
//   if n <= 1 then
//     factorialArray[n] := 1
//   else if factorialArray[n] = 0 then
//     factorialArray[n] := n * factorial(n - 1);
//   factorial := factorialArray[n];
// end;
// rewrite the combinations function to use the factorial function above


function combinations(n, r: Integer): Int64;
begin
  combinations := factorial(n) div (factorial(r) * factorial(n - r));
end;

function hypercake(n, k: Integer): Int64;
var
  i: Integer;
  result: Int64;
begin

  result := 0;
  for i := 0 to k do
    result := result + combinations(n, i);
  hypercake := result;
end;
// initiialize n and k to integer variables
var
  n, k: Integer; 
begin
  writeln('Enter number of cuts: ');
  readln(n);
  writeln('Enter the number of dimensions: ');
  readln(k);
  writeln('Solution to the hypercake problem: ', hypercake(n, k));
end.

