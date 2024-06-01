module Args.Parse (
  parseArgs,
  Args(..)
) where

data Args = Args {
  grammarFile :: String,
  files :: [String]
} deriving Show

defaultArg :: Args
defaultArg = Args {
  grammarFile = "",
  files = []
}

parseArgs :: [String] -> Args
parseArgs l = p' l defaultArg
  where
    p' [] a = a
    p' (c:x:xs) a
      | c == "-g" || c == "--grammar" = p' xs a { grammarFile = x }
    p' (x:xs) a = p' xs (a { files = x : files a})
