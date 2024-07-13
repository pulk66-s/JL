module Logic.Parse (
    parseFile
) where

import Grammar.Data
import Logic.Data
import Logic.Parser
import Logic.ParserCreation

parseFile :: String -> Grammar -> Either String ([Logic], String)
parseFile content grammar   = runParser (parser grammar) content
