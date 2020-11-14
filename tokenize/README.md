# Tokens

Simple trait to iterate over all words (=`tokens`) in a given sentence to yield individual tokens.

## Why is this useful?

Used to construct index table (`Vec<(Token, DocId, Position)>`) from a document, where
Token is a String (Word in a sentence)
DocId is a unique identifier for a document
Position is the amount of words coming before this index, in a given document
