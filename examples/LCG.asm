LI R1, 50        # number of random numbers
LI R2, 123       # seed
LI R3, 0         # memory pointer
LI R0, 0         # zero register
LI R4, 75        # multiplier a
LI R5, 74        # increment c

rng_loop:
    MULT R7, R2, R4
    ADD R7, R7, R5
    SD R7, R3
    ADDI R3, R3, 1
    ADD R2, R7, R0
    SUBI R1, R1, 1
    JLT R0, R1, rng_loop   # continue if R1 > 0

END
