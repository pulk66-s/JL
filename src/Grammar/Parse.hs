module Grammar.Parse (
  parseGrammar,
) where

import Data.Char
import Grammar.Data
import String
import Data.List.Split

parseGrammar :: String -> Maybe Grammar
parseGrammar content  = case createBlocks (nonEmpty (lines content)) of
  Just blocks -> Just Grammar { grammarBlocks = blocks }
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
createBlock x = case mysplit '=' (init x) of
  [name, body] -> case createExpr body of
    Just expr -> Just Block { blockName = trim name, blockExpr = expr }
    Nothing   -> Nothing
  _ -> Nothing

unwrapMaybe :: [Maybe a] -> Maybe [a]
unwrapMaybe [] = Just []
unwrapMaybe (Just x:xs) = case unwrapMaybe xs of
  Just xs' -> Just (x:xs')
  Nothing -> Nothing
unwrapMaybe _ = Nothing

chainedExpr :: Char -> String -> ([Expr] -> Expr) -> Maybe Expr
chainedExpr c x f = case unwrapMaybe $ map (createExpr . trim) $ mysplit c x of
  Just exprs -> Just $ f exprs
  Nothing    -> Nothing

createExpr :: String -> Maybe Expr
createExpr (' ':xs) = createExpr xs
createExpr ('[':xs)
  | last xs == ']' && contains ".." xs = createGeneratorExpr (init xs)
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
createExpr ('\'':'\\':x:'\'':_) = Just $ Char $ case x of
  'n' -> '\n'
  'r' -> '\r'
  't' -> '\t'
  _   -> x
createExpr ('\'':x:'\'':_) = Just $ Char x
createExpr x  = Just $ ExprCall $ trim x

--
-- Generator logic
-- 

checkGeneratorType :: String -> String -> Maybe GeneratorExpr
checkGeneratorType ('\'':x:'\'':_) ('\'':y:'\'':_)
  | x < y = Just $ CharGenerator x y
checkGeneratorType x y
  | all (== True) (map isDigit x) && all (== True) (map isDigit y) && (read x :: Int) < (read y :: Int)
    = Just $ NumberGenerator (read x :: Int) (read y :: Int)
checkGeneratorType _ _ = Nothing

createGeneratorExpr :: String -> Maybe Expr
createGeneratorExpr x = case splitOn ".." x of
  [a, b] -> case checkGeneratorType a b of
    Just gen -> Just $ Generator gen
    Nothing  -> Nothing
  _ -> Nothing
