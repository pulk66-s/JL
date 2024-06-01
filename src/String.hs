module String (
    trim,
    split
) where

trim :: String -> String
trim = reverse . dropWhile (== ' ') . reverse . dropWhile (== ' ')

split :: Char -> String -> [String]
split _ [] = []
split c xs = case break (== c) xs of
  (a, []) -> [a]
  (a, b)  -> a : split c (tail b)

