# Mahjong-Calculator
A simple Rust-based CLI program that calculates your Fu (符) and Han (飯)when you go out by either Tsumo or Ron.

(From Wikipedia)
The scoring system uses structural criteria as well as bonuses. Player start scores may be set to any value. Usually, it is set to 20,000 to 30,000 points. Scores are counted using sticks of 10,000 points, 5,000 points, 1,000 points and 100 points. A game often ends when all the points of a player are lost, which is a situation called hakoten,[nb 1] dobon,[nb 2] buttobi,[nb 3] etc.
There are two criteria in determining the winning points: han and fu, which correspond to a points table. Han is the unit for the value of yaku, which are particular patterns or conditions of a hand. Fu is the value of melds, waits and "going out".

This program is written in pure Rust for studying the grammar and some other awesome feature of the language.

Example when you win:

```txt
East seat | East round | 1 Honba | Dora: 5s
456m123p123s555z1z | 1z Ron after Riichi
```

```
The Fu for this hand:
20 + 10 + 8 + 4 Rounded => 50 Fu

The Han for this hand:
立直　x１
役牌 白 X1
一発　x１

20 is the base score, 10 is for Monzennkafu (門前加符), 8 is for Yaokyu Anko （ヤオキュウ暗刻）, 2 for Tanki (単騎), and 2 for Yakuhai no Atama (役牌の頭)

Result：3翻50符
Score: 9900 Point
```
