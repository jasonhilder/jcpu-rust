START:
    DATA R1, 2
    DATA R2, 2
    DATA R4, 2
    DATA R3, 1
ADDER:
    ADD R1, R2
    PUSH R2
    CMP R4, R3
    JMPIFZ $END
    DEC R4
    JMP $ADDER
END:
    POP R3
    PUSH R1
    PUSH R2
    HLT
