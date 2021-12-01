import Test.Hspec        (Spec, it, shouldBe)
import Test.Hspec.Runner (configFastFail, defaultConfig, hspecWith)

import Day01 (solution)

main :: IO ()
main = do putStrLn "\n="
          print solution
