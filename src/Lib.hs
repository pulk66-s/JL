module Lib
  ( compile
  ) where

import Data.Char
import Args.Parse
import Grammar.Parse
import Grammar.Data
import System.Environment
import Logic.Parse

createLogic :: String -> Grammar -> IO ()
createLogic file grammar = do
  content <- readFile file
  let result = parseFile content grammar
  print "Result:"
  case result of
    Left logic -> print logic
    Right str  -> print str

showExpr :: Expr -> IO ()
showExpr (Keyword str) = putStrLn ("Keyword: " ++ str)
showExpr (Number n) = putStrLn ("Number: " ++ show n)
showExpr (Or exprs) = do
  putStrLn "Or:"
  mapM_ showExpr exprs
showExpr (And exprs) = do
  putStrLn "And:"
  mapM_ showExpr exprs
showExpr (ExprCall str) = putStrLn ("ExprCall: " ++ str)
showExpr (Many expr) = do
  putStrLn "Many:"
  showExpr expr
showExpr (Generator (CharGenerator c1 c2)) = putStrLn ("CharGenerator: " ++ [c1] ++ " " ++ [c2])
showExpr (Generator (NumberGenerator n1 n2)) = putStrLn ("NumberGenerator: " ++ show n1 ++ " " ++ show n2)
showExpr (Char c) = putStrLn ("Char: " ++ [c])
showExpr (Maybe expr) = do
  putStrLn "Maybe:"
  showExpr expr

showBlock :: Block -> IO ()
showBlock (Block name expr) = do
  putStrLn ("Block: " ++ name)
  showExpr expr

showGrammar :: Grammar -> IO ()
showGrammar (Grammar blocks) = do
  mapM_ showBlock blocks

compile :: IO ()
compile = do
  argv        <- getArgs
  let args    = parseArgs argv
  gfile <- readFile (grammarFile args)
  print (gfile)
  print (map init (nonEmpty (lines gfile)))
  case parseGrammar gfile of
    Nothing -> error "Error while parsing gramar file"
    Just g  -> do
      print "Grammar:"
      print g
      createLogic ((files args) !! 0) g
  where
    nonEmpty = filter (not . all isSpace)
