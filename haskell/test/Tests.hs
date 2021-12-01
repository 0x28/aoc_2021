import Test.Hspec        (Spec, it, shouldBe)
import Test.Hspec.Runner (configFastFail, defaultConfig, hspecWith)

import Day01 (solution)

input01 :: IO [Int]
input01 = do
  l <- fmap lines (readFile "./input/input01.txt")
  return $ map read l

main :: IO ()
main = do putStr "day01 = "
          input <- input01
          print (solution input)
