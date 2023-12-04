module Main where

import Data.Bits
import Data.Void
import Text.Megaparsec
import Text.Megaparsec.Char
import Text.Megaparsec.Char.Lexer qualified as L

unwrap :: forall s e v. (VisualStream s, TraversableStream s, ShowErrorComponent e) => Either (ParseErrorBundle s e) v -> v
unwrap (Left bundle) = error (errorBundlePretty bundle)
unwrap (Right value) = value

type Parser = Parsec Void String

main :: IO ()
main = do
    input <- readFile "day-04.txt"
    let cards = cards' input
    print $ part1 cards
    print $ part2 cards

cards' :: String -> [Card]
cards' input = map (unwrap . (parse card' "")) (lines input)

type Card = ([Int], [Int])

card' :: Parser Card
card' = do
    _ <- string "Card" >> space1 >> int >> string ":" >> space1
    need <- sepEndBy1 int space1
    _ <- string "|" >> space1
    have <- sepEndBy1 int space1
    return (need, have)

int :: Parser Int
int = L.decimal

part1 :: [Card] -> Int
part1 cards = sum $ map getScore cards

countWinning :: Card -> Int
countWinning (need, have) = length $ filter (`elem` have) need

getScore :: Card -> Int
getScore (need, have) = case countWinning (need, have) of
    0 -> 0
    n -> shiftL 1 (n - 1)

type WithMultiplier = (Card, Int)

part2 :: [Card] -> Int
part2 cards = play 0 $ map (\card -> (card, 1)) cards

play :: Int -> [WithMultiplier] -> Int
play n [] = n
play n ((card, multi) : cards) = play (n + multi) rest
  where
    score = countWinning card
    indexed = zip cards [1 ..]
    update idx num = if idx <= score then num + multi else num
    rest = map (\((next, num), idx) -> (next, update idx num)) indexed
