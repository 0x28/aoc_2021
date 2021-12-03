{-# OPTIONS_GHC -Wno-type-defaults #-}
module Day03 where

import Data.Char (digitToInt)
import Data.List (transpose)

common :: String -> Int
common xs
  | ones >= zeros = 1
  | otherwise = 0
  where
    ones = length $ filter (== '1') xs
    zeros = length $ filter (== '0') xs

invert :: [Int] -> [Int]
invert (1 : xs) = 0 : invert xs
invert (0 : xs) = 1 : invert xs
invert _ = []

binToInt' :: [Int] -> Int
binToInt' [] = 0
binToInt' (x : xs) = x + 2 * binToInt' xs

binToInt :: [Int] -> Int
binToInt = binToInt' . reverse

part1 :: [String] -> Int
part1 xs = binToInt eta * binToInt gamma
  where
    eta = map common $ transpose xs
    gamma = invert eta

rate :: (Int -> Int -> Char) -> [String] -> String
rate _ [] = []
rate _ [s] = s
rate r xs = bit : rate r (map tail (filter (\s -> head s == bit) xs))
  where
    currentBits = map head xs
    ones = length (filter ('1' ==) currentBits)
    zeros = length (filter ('0' ==) currentBits)
    bit = r ones zeros

oxygen :: [String] -> String
oxygen = rate (\ones zeros -> if ones >= zeros then '1' else '0')

co2 :: [String] -> String
co2 = rate (\ones zeros -> if ones < zeros then '1' else '0')

part2 :: [String] -> Int
part2 xs = co2Rating * oxygenRating
  where
    co2Rating = binToInt $ map digitToInt $ co2 xs
    oxygenRating = binToInt $ map digitToInt $ oxygen xs
