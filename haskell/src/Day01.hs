module Day01 (solution) where

part1 :: [Int] -> Int
part1 (f : s : xs)
  | f < s = 1 + part1 (s : xs)
  | otherwise = part1 (s : xs)
part1 _ = 0

windows :: [Int] -> [[Int]]
windows (a : b : c : rest) = [a, b, c] : windows (b : c : rest)
windows _ = []

solution :: [Int] -> Int
solution = part1 . map sum . windows
