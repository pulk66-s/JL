module Grammar.Parse (
  parseGrammar,
) where

import System.Environment
import Data.Char
import Grammar.Data
import String

parseGrammar :: String -> Maybe Grammar
parseGrammar content  = case createBlocks (nonEmpty (lines content)) of
  Just blocks -> Just Grammar { blocks = blocks }
  Nothing     -> Nothing
  where
    nonEmpty = filter (not . all isSpace)

createBlocks :: [String] -> Maybe [Block]
createBlocks []   = Just []
createBlocks (x:xs) = case createBlock (trim x) of
  Just block -> case createBlocks xs of
    Just blocks -> Just (block:blocks)
    Nothing     -> Nothing
  Nothing -> Nothing

createBlock :: String -> Maybe Block
createBlock x
  | all isSpace x = Nothing
createBlock x
  | last x /= ';' = Nothing
createBlock x = case createExpr body of
  Just expr -> Just Block { name = trim name, expr = expr }
  Nothing   -> Nothing
  where
    [name, body] = split '=' (init x)

unwrapMaybe :: [Maybe a] -> Maybe [a]
unwrapMaybe [] = Just []
unwrapMaybe (Just x:xs) = case unwrapMaybe xs of
  Just xs -> Just (x:xs)
  Nothing -> Nothing
unwrapMaybe _ = Nothing

chainedExpr :: Char -> String -> ([Expr] -> Expr) -> Maybe Expr
chainedExpr c x f = case unwrapMaybe $ map (createExpr . trim) $ split c x of
  Just exprs -> Just $ f exprs
  Nothing    -> Nothing

createExpr :: String -> Maybe Expr
createExpr (' ':xs) = createExpr xs
createExpr x
  | elem '&' x  = chainedExpr '&' x And
createExpr x
  | elem '|' x  = chainedExpr '|' x Or
createExpr x
  | last x == '?' = case createExpr (init x) of
    Just expr -> Just $ Maybe expr
    Nothing   -> Nothing
createExpr x
  | last x == '*' = case createExpr (init x) of
    Just expr -> Just $ Many expr
    Nothing   -> Nothing
createExpr x
  | all (== True) (map isDigit (trim x))
    = Just $ Number $ (read x :: Int)
createExpr ('\"':xs)  = Just $ Keyword $ takeWhile (/= '\"') xs
createExpr ('\'':'\\':x:'\'':xs) = Just $ Char $ case x of
  'n' -> '\n'
  'r' -> '\r'
  't' -> '\t'
  _   -> x
createExpr ('\'':x:'\'':xs) = Just $ Char x
createExpr x  = Just $ ExprCall $ trim x
