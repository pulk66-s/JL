module Lib
  ( compile
  ) where

import Data.Char
import Args.Parse
import Grammar.Parse
import Grammar.Data
import System.IO
import System.Environment
import Parser.Parse
import Parser.ParserCreation

createLogic :: String -> Grammar -> IO ()
createLogic file grammar = do
  content <- readFile file
  let result = parseFile content grammar
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
  grammarFile <- readFile (grammarFile args)
  print (grammarFile)
  print (map init (nonEmpty (lines grammarFile)))
  case parseGrammar grammarFile of
    Nothing -> error "Error while parsing gramar file"
    Just g  -> do
      print "Grammar:"
      print g
      createLogic ((files args) !! 0) g
  where
    nonEmpty = filter (not . all isSpace)
