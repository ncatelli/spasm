# spasm
An experimental multi-target assembler assembler.

## Preparser
The preparser functions to provide many quality-of-life features that were previously introduced in a single backend. These include:

- Constants
    - Sized
- Labels
- Comment parsing

This feature-set is small to begin with but functions to standardize and consolidate the grammar accross multiple backends, leaving the backend to only handle the mapping to opcodes.

### Grammar

```
program        = ( origin | statement )+ ;

statement      = ( whitespace | newline )* ( labeldef | symboldef | origin | instruction | comment )+ comment?  ( newline | EOF );

instruction    = ( alphabetic | digit | special | ";"! )+ ;

comment        = ";" ( whitespace | character )* ;

symboldef      = bytedef | twobytedef | fourbytedef ;

bytedef        = ".1byte" whitespace+ alphabetic* whitespace+ byte ;
twobytedef     = ".2byte" whitespace+ alphabetic* whitespace+ byte byte ;
fourbytedef    = ".4byte" whitespace+ alphabetic* whitespace+ byte byte byte byte ;

origin         = ".origin" whitespace+ byte byte byte byte ;

labeldef       = alphabetic* ":" ;

character      = lower|upper|digit|special ;
whitespace     = " " | "\t" ;
newline        = "\n" ;
alphabetic     = (lower|upper) ;
lower          = "a"|"b"|"c"|"d"|"e"|"f"|"g"|"h"|"i"|"j"|"k"|"l"|"m"
               |"n"|"o"|"p"|"q"|"r"|"s"|"t"|"u"|"v"|"w"|"x"|"y"|"z" ;
upper          = "A"|"B"|"C"|"D"|"E"|"F"|"G"|"H"|"I"|"J"|"K"|"L"|"M"
               |"N"|"O"|"P"|"Q"|"R"|"S"|"T"|"U"|"V"|"W"|"X"|"Y"|"Z" ;
byte           = ( "0x" hex hex ) | digit+ | ( "0b" binarybyte ) ;
hex            = "0"|"1"|"2"|"3"|"4"|"5"|"6"|"7"|"8"|"9"|"a"|"b"|"c"
               |"d"|"e"|"f"|"A"|"B"|"C"|"D"|"E"|"F" ;
binarybyte     = binary binary binary binary binary binary binary binary ;
binary         = "0" | "1" ;
digit          = "0"|"1"|"2"|"3"|"4"|"5"|"6"|"7"|"8"|"9" ;
special        = "-"|"_"|"\""|"#"|"&"|"’"|"("|")"|"*"|"+"|","|"."|"/"
               |":"|";"|"<"|"="|">" ;
```

## Backends

- [MOS6502](./src/backends/mos6502/README.md)

## Warnings
Please nobody use this. This is entirely an experiment to support insane restrictions I've imposed on myself to build a computer from first principles.

Instead, please use the wonderful [vasm](http://sun.hasenbraten.de/vasm/) project that is better supported, better engineered and WAY more mature... frankly I don't even know why you are even looking at this.
