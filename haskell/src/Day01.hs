module Day01 where

increases :: [Int] -> Int
increases (f : s : xs)
  | f < s = 1 + increases (s : xs)
  | otherwise = increases (s : xs)
increases _ = 0

windows :: [Int] -> [[Int]]
windows (a : b : c : rest) = [a, b, c] : windows (b : c : rest)
windows _ = []

part1 :: [Int] -> Int
part1 = increases

part2 :: [Int] -> Int
part2 = increases . map sum . windows
