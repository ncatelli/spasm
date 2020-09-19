# spasm
An experimental 6502 assembler.

## Grammar

```
program        = instruction* ;

instruction    = mnemonic operand ("\n" | EOF) ;

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
```

## Warnings
Please nobody use this. This is entirely an experiment to support insane restrictions I've imposed on myself to build a computer from first principles.

Instead, please use the wonderful [vasm](http://sun.hasenbraten.de/vasm/) project that is better supported, better engineered and WAY more mature... frankly I don't even know why you are even looking at this.
