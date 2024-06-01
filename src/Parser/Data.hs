module Parser.Data (
  Logic(..)
) where

data Logic = NumberValue Int
  | CharValue Char
  | KeywordValue String
  | VarDef String Logic
  | VarRef String
  deriving Show
