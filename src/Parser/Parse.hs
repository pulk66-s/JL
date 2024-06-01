module Parser.Parse (
    parseFile
) where

import Grammar.Data
import Parser.Data
import Parser.Parser
import Parser.ParserCreation
import System.IO

parseFile :: String -> Grammar -> Either String ([Logic], String)
parseFile content grammar   = runParser (parser grammar) content

