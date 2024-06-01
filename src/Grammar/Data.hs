module Grammar.Data (
  Grammar(..),
  Block(..),
  Expr(..)
) where

data Expr = Keyword String
  | Number Int
  | Char Char
  | Or [Expr]
  | And [Expr]
  | Many Expr
  | Maybe Expr
  | ExprCall String
  deriving Show

data Block = Block {
  name :: String,
  expr :: Expr
} deriving Show

data Grammar = Grammar {
  blocks :: [Block]
} deriving Show
