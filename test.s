define thisisatest $12

init:
    nop
    lda #thisisatest
    sta $1234
    jmp init