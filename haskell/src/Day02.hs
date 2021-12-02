module Day02 (part1, part2) where

data Command = Forward Int | Down Int | Up Int deriving (Show, Eq)

parse :: String -> Command
parse line = case words line of
  ["forward", value] -> Forward $ read value
  ["down", value] -> Down $ read value
  ["up", value] -> Up $ read value
  _ -> undefined

navigate :: Command -> (Int, Int) -> (Int, Int)
navigate (Up value) (x, y) = (x, y - value)
navigate (Down value) (x, y) = (x, y + value)
navigate (Forward value) (x, y) = (x + value, y)

part1 :: [String] -> Int
part1 input = x * y
  where
    (x, y) = foldr (navigate . parse) (0, 0) input

navigate' :: Command -> (Int, Int, Int) -> (Int, Int, Int)
navigate' (Up value) (aim, x, y) = (aim - value, x, y)
navigate' (Down value) (aim, x, y) = (aim + value, x, y)
navigate' (Forward value) (aim, x, y) = (aim, x + value, y + aim * value)

part2 :: [String] -> Int
part2 input = x * y
  where
    (_, x, y) = foldr (navigate' . parse) (0, 0, 0) input
