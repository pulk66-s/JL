module Parser.Parser where

import Control.Applicative
import Grammar.Data
import Parser.Data

newtype Parser a = Parser {
    run :: String -> Either String (a, String)
}

instance Functor Parser where
    fmap f (Parser p) = Parser $ \s -> case p s of
        Left err -> Left err
        Right (x, s') -> Right (f x, s')

instance Applicative Parser where
    pure a                  = Parser func
        where
            func str         = return (a, str)
    Parser p1 <*> Parser p2 = Parser func
        where
            func str = do
                (f, str') <- p1 str
                (a, str'') <- p2 str'
                return (f a, str'')

instance Monad Parser where
    Parser p >>= f  = Parser func
        where
            func str = do
                (a, str') <- p str
                let (Parser b) = f a
                b str'

instance Alternative Parser where
    empty = Parser run
        where
            run str = Left "error: empty"
    many p = Parser func
        where
            func str = case runParser (some p) str of
                Left _ -> return ([], str)
                Right (a, str') -> return (a, str')
    some p = do
        a <- p
        as <- many p
        return (a:as)
    Parser p1 <|> Parser p2 = Parser func
        where
            func str = case p1 str of
                Left _  -> p2 str
                other   -> other

runParser :: Parser [a] -> String -> Either String ([a], String)
runParser (Parser f) = f

parseOr :: (a -> Parser [b]) -> [a] -> Parser [b]
parseOr f []     = empty
parseOr f (x:xs) = f x <|> parseOr f xs

parseAnd :: (a -> Parser [b]) -> [a] -> Parser [b]
parseAnd f []     = return []
parseAnd f (x:xs) = do
    a <- f x
    as <- parseAnd f xs
    return (a ++ as)

satisfy :: (Char -> Bool) -> Parser Char
satisfy p = Parser func
    where
        func [] = Left "error: satisfy empty"
        func (x:xs)
            | p x       = return (x, xs)
            | otherwise = Left "error: satisfy false"

parseSome :: Parser a -> Parser [a]
parseSome p = do
    a <- p
    as <- many p
    return (a:as)

parseEmpty :: Parser a
parseEmpty = empty

parseMany :: (a -> Parser [b]) -> a -> Parser [b]
parseMany f x = do
    a <- f x
    as <- many (f x)
    return (a ++ flat as)
    where
        flat [] = []
        flat (x:xs) = x ++ flat xs

parseMaybe :: (a -> Parser [b]) -> a -> Parser [b]
parseMaybe f x = f x <|> return []
