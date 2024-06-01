module Parser.ParserCreation (
    parser
) where

import Parser.Parser
import Grammar.Data
import Parser.Data

digit :: Parser Char
digit = satisfy (`elem` "0123456789")

charC :: Char -> Parser Char
charC c = satisfy (== c)

stringS :: String -> Parser String
stringS [] = return []
stringS (x:xs) = do
    c <- charC x
    cs <- stringS xs
    return (c:cs)

number :: Parser Int
number = parseSome digit >>= return . read

numberN :: Int -> Parser Int
numberN n = do
    num <- number
    if num == n then return num else parseEmpty

executeBlocks :: [Block] -> String -> Parser [Logic]
executeBlocks blocks blockName  = do
    let block = filter (\x -> blockName == name x) blocks
    if null block then parseEmpty else parseBlock (head block) blocks

parseExpr :: [Block] -> Expr -> Parser [Logic]
parseExpr _ (Number n)          = do
    d <- digit
    if read [d] == n then return [NumberValue n] else parseEmpty
parseExpr blocks (Or exprs)     = parseOr (parseExpr blocks) exprs
parseExpr blocks (And exprs)    = parseAnd (parseExpr blocks) exprs
parseExpr blocks (ExprCall s)   = executeBlocks blocks s
parseExpr blocks (Many expr)    = parseMany (parseExpr blocks) expr
parseExpr blocks (Char c)       = do
    d <- charC c
    return $ [CharValue d]
parseExpr blocks (Keyword s)    = do
    d <- stringS s
    return $ [KeywordValue s]
parseExpr blocks (Maybe expr)   = parseMaybe (parseExpr blocks) expr

parseBlock :: Block -> [Block] -> Parser [Logic]
parseBlock block blocks = parseExpr blocks (expr block)

parseBlocks :: [Block] -> [Block] -> Parser [Logic]
parseBlocks [] _            = return []
parseBlocks (x:xs) blocks   = do
    logic <- parseBlock x blocks
    logics <- parseBlocks xs blocks
    return (logic ++ logics)

parser :: Grammar -> Parser [Logic]
parser grammar = parseBlock ((blocks grammar) !! 0) (blocks grammar)
