program HyperCakePascal;

var
  factorialArray: array of Int64;

function factorial(n: Integer): Int64;
begin
  if n <= 1 then
    factorialArray[n] := 1
  else if factorialArray[n] = 0 then
    factorialArray[n] := n * factorial(n - 1);
  factorial := factorialArray[n];
end;

function combinations(n, r: Integer): Int64;
begin
  combinations := factorial(n) div (factorial(r) * factorial(n - r));
end;

function hypercake(n, k: Integer): Int64;
var
  i: Integer;
  result: Int64;
begin
  SetLength(factorialArray, n + 1);
  result := 1;
  for i := 1 to n do
    result := result + combinations(k, i);
  hypercake := result;
end;

begin
  writeln(hypercake(4, 2));
end.