module Parser.ParserCreation (
    parser,
    parseGenerator
) where

import Parser.Parser
import Grammar.Data
import Parser.Data
import Data.Char

digit :: Parser Char
digit = satisfy (`elem` "0123456789")

charC :: Char -> Parser Char
charC c = satisfy (== c)

inRangeC :: Char -> Char -> Parser Char
inRangeC c1 c2 = satisfy (\x -> c1 <= x && x <= c2)

inRangeN :: Int -> Int -> Parser Int
inRangeN n1 n2 = satisfy (\x -> n1 <= (digitToInt x) && (digitToInt x) <= n2) >>= return . read . return

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
executeBlocks blocks bname  = do
    let block = filter (\x -> bname == blockName x) blocks
    if null block then parseEmpty else parseBlock (head block) blocks

parseExpr :: [Block] -> Expr -> Parser [Logic]
parseExpr _ (Number n)          = do
    d <- digit
    if read [d] == n then return [NumberValue n] else parseEmpty
parseExpr blocks (Or exprs)     = parseOr (parseExpr blocks) exprs
parseExpr blocks (And exprs)    = parseAnd (parseExpr blocks) exprs
parseExpr blocks (ExprCall s)   = executeBlocks blocks s
parseExpr blocks (Many expr)    = parseMany (parseExpr blocks) expr
parseExpr _ (Generator gen)     = parseGenerator gen
parseExpr _ (Char c)            = do
    d <- charC c
    return $ [CharValue d]
parseExpr _ (Keyword s)         = do
    d <- stringS s
    return $ [KeywordValue d]
parseExpr blocks (Maybe expr)   = parseMaybe (parseExpr blocks) expr

parseBlock :: Block -> [Block] -> Parser [Logic]
parseBlock block blocks = parseExpr blocks (blockExpr block)

parseBlocks :: [Block] -> [Block] -> Parser [Logic]
parseBlocks [] _            = return []
parseBlocks (x:xs) blocks   = do
    logic <- parseBlock x blocks
    logics <- parseBlocks xs blocks
    return (logic ++ logics)

parser :: Grammar -> Parser [Logic]
parser grammar = parseBlock ((grammarBlocks grammar) !! 0) (grammarBlocks grammar)

parseGenerator :: GeneratorExpr -> Parser [Logic]
parseGenerator (CharGenerator c1 c2) = do
    d <- inRangeC c1 c2
    return [CharValue d]
parseGenerator (NumberGenerator n1 n2) = do
    d <- inRangeN n1 n2
    return [NumberValue d]
