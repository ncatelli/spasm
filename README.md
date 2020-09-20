# spasm
An experimental 6502 assembler.

## Grammar

```
instructions   = (whitespace | newline)* (instruction | comment)* (newline | EOF) ;

instruction    = whitespace* mnemonic (whitespace+ operand)? whitespace+ comment? ;

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
               | "TAX" | "tax" | TXA" | "txa"
               | "TAY" | "tay" | "TYA" | "tya"
               | "TSX" | "tsx" | "TXS" | "txs"
               | "PHA" | "pha" | "PLA" | "pla"
               | "PHP" | "php" | "PLP" | "plp"
               | "JMP" | "jmp" | "JSR" | "jsr"
               | "RTS" | "rts" | "RTI" | "rti"
               | "CLC" | "clc" | "CLD" | "cld" | "CLI" | "cli" | "CLV" | "clv"
               | "SEC" | "sec" | "SED" | "sed" | SEI" | "sei"
               | "BRK" | "brk" | "NOP" | "nop"

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

comment        = ";" (whitespace | character)* ;

lower          = a|b|c|d|e|f|g|h|i|j|k|l|m|n|o|p|q|r|s|t|u|v|w|x|y|z
upper          = A|B|C|D|E|F|G|H|I|J|K|L|M|N|O|P|Q|R|S|T|U|V|W|X|Y|Z
digit          = 0|1|2|3|4|5|6|7|8|9
special        = -|_|"|#|&|’|(|)|*|+|,|.|/|:|;|<|=|>
character      = lower|upper|digit|special
whitespace     = " " | "\t"
newline        = "\n"

```

## Warnings
Please nobody use this. This is entirely an experiment to support insane restrictions I've imposed on myself to build a computer from first principles.

Instead, please use the wonderful [vasm](http://sun.hasenbraten.de/vasm/) project that is better supported, better engineered and WAY more mature... frankly I don't even know why you are even looking at this.
