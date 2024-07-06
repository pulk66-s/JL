module Grammar.Data (
  Grammar(..),
  Block(..),
  Expr(..),
  GeneratorExpr(..)
) where

data Expr = Keyword String
  | Number Int
  | Char Char
  | Or [Expr]
  | And [Expr]
  | Many Expr
  | Maybe Expr
  | ExprCall String
  | Generator GeneratorExpr
  deriving Show

data GeneratorExpr = CharGenerator Char Char
  | NumberGenerator Int Int
  deriving Show

data Block = Block {
  blockName :: String,
  blockExpr :: Expr
} deriving Show

data Grammar = Grammar {
  grammarBlocks :: [Block]
} deriving Show
