import qualified Day01
import qualified Day02
import Test.Hspec (Spec, it, shouldBe)
import Test.Hspec.Runner (configFastFail, defaultConfig, hspecWith)
import Text.Printf

lineInput :: String -> IO [String]
lineInput file = do
  fmap lines (readFile file)

input01 :: IO [Int]
input01 = do
  l <- lineInput "./input/input01.txt"
  return $ map read l

main01 :: IO ()
main01 = do
  input <- input01
  printf "day01 = %s\n" $ show (Day01.part1 input, Day01.part2 input)

main02 :: IO ()
main02 = do
  input <- lineInput "./input/input02.txt"
  printf "day02 = %s\n" $ show (Day02.part1 input, Day02.part2 input)

main :: IO ()
main = do
  main01
  main02
