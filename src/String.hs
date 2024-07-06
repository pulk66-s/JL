module String (
    trim,
    mysplit,
    contains
) where

import Data.List.Split

trim :: String -> String
trim = reverse . dropWhile (== ' ') . reverse . dropWhile (== ' ')

mysplit :: Char -> String -> [String]
mysplit _ [] = []
mysplit c xs = case break (== c) xs of
  (a, []) -> [a]
  (a, b)  -> a : mysplit c (tail b)

contains :: String -> String -> Bool
contains _ [] = False
contains x y = case splitOn x y of
  (_:_) -> True
  _     -> False