module Logic.Data (
  Logic(..),
  LogicBlock(..),
  FuncDef(..),
  FuncBody(..),
  FuncParam(..),
  Type(..),
) where

data LogicBlock = LogicBlock {
  logicBlockName :: String,
  logicBlockBody :: [Logic]
} deriving Show

data Logic = NumberValue Int
  | CharValue Char
  | KeywordValue String
  | SubBlock LogicBlock
  deriving Show

data FuncDef = FuncDef [FuncParam] FuncBody Type
  deriving Show

data FuncBody = FuncBody [Logic]
  deriving Show

data FuncParam = FuncParam Type String
  deriving Show

data Type = String
  deriving Show
