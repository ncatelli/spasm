# MOS Technology 6502 Backend
## Grammar

```
instructions   = ( whitespace | newline )* ( labeldef | symboldef | comment | instruction )+ ( newline | EOF ) ;

instruction    = whitespace* mnemonic ( whitespace+ ( operand  ) )? whitespace+ comment? ;

mnemonic       = "LDA" | "lda" | "LDX" | "ldx" | "LDY" | "ldy"
               | "STA" | "sta" | "STX" | "stx" | "STY" | "sty"
               | "ADC" | "adc" | "SBC" | "sbc"
               | "INC" | "inc" | "INX" | "inx" | "INY" | "iny"
               | "DEC" | "dec" | "DEX" | "dex" | "DEY" | "dey"
               | "ASL" | "asl" | "LSR" | "lsr"
               | "ROL" | "rol" | "ROR" | "ror"
               | "AND" | "and" | "ORA" | "ora" | "EOR" | "eor"
               | "CMP" | "cmp" | "CPX" | "cpx" | "CPY" | "cpy"
               | "BIT" | "bit"
               | "BCC" | "bcc" | "BCS" | "bcs"
               | "BNE" | "bne" | "BEQ" | "beq"
               | "BPL" | "bpl" | "BMI" | "bmi"
               | "BVC" | "bvc" | "BVS" | "bvs"
               | "TAX" | "tax" | "TXA" | "txa"
               | "TAY" | "tay" | "TYA" | "tya"
               | "TSX" | "tsx" | "TXS" | "txs"
               | "PHA" | "pha" | "PLA" | "pla"
               | "PHP" | "php" | "PLP" | "plp"
               | "JMP" | "jmp" | "JSR" | "jsr"
               | "RTS" | "rts" | "RTI" | "rti"
               | "CLC" | "clc" | "CLD" | "cld" | "CLI" | "cli" | "CLV" | "clv"
               | "SEC" | "sec" | "SED" | "sed" | SEI" | "sei"
               | "BRK" | "brk" | "NOP" | "nop"

comment        = ";" ( whitespace | character )* ;

symboldef      = "define" whitespace+ alphabetic* whitespace+ byte ;

symbol         = alphabetic* ;

labeldef       = alphabetic* ":" ;

label          = alphabetic* ;

operand        = accumulator
               | absolute
               | absolute_x_indexed
               | absolute_y_indexed
               | immediate
               | indirect
               | x_indexed_indirect
               | indirect_y_indexed
               | relative
               | zeropage
               | zeropage_x_indexed
               | zeropage_y_indexed

accumulator        = "A" ;
absolute           = word ;
absolute_x_indexed = word ",X" ;
absolute_y_indexed = word ",Y" ;
immediate          = "#" ( byte | symbol );
indirect           = "(" word ")";
x_indexed_indirect = "(" ( byte | symbol ) ",X)" ;
indirect_y_indexed = "(" ( byte | symbol ) "),Y" ;
relative           = "*" sign? ( byte | symbol ) ;
zeropage           = byte ;
zeropage_x_indexed = byte ",X" ;
zeropage_y_indexed = byte ",Y" ;

character      = lower|upper|digit|special ;
whitespace     = " " | "\t" ;
newline        = "\n" ;
alphabetic     = (lower|upper) ;
lower          = "a"|"b"|"c"|"d"|"e"|"f"|"g"|"h"|"i"|"j"|"k"|"l"|"m"
               |"n"|"o"|"p"|"q"|"r"|"s"|"t"|"u"|"v"|"w"|"x"|"y"|"z" ;
upper          = "A"|"B"|"C"|"D"|"E"|"F"|"G"|"H"|"I"|"J"|"K"|"L"|"M"
               |"N"|"O"|"P"|"Q"|"R"|"S"|"T"|"U"|"V"|"W"|"X"|"Y"|"Z" ;
word           = ( "$" hex hex hex hex ) | digit+ 
               | binarybyte binarybyte ;
byte           = ( "0x" hex hex ) | digit+ | binarybyte ;
hex            = "0"|"1"|"2"|"3"|"4"|"5"|"6"|"7"|"8"|"9"|"a"|"b"|"c"
               |"d"|"e"|"f"|"A"|"B"|"C"|"D"|"E"|"F" ;
number         = digit+ ;
sign           = "-" | "+" ;
binarybyte     = binary binary binary binary binary binary binary binary ;
binary         = "0" | "1" ;
digit          = "0"|"1"|"2"|"3"|"4"|"5"|"6"|"7"|"8"|"9" ;
special        = "-"|"_"|"\""|"#"|"&"|"’"|"("|")"|"*"|"+"|","|"."|"/"
               |":"|";"|"<"|"="|">" ;
```
