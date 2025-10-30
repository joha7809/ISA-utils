# Generate Fibonacci numbers up to N terms
# Memory layout: memory[0..N-1] = Fib sequence

LI R1, 40        # R1 = N (number of Fibonacci numbers)
LI R2, 0         # R2 = index i
LI R3, 0         # R3 = Fib[i-2] (prev)
LI R4, 1         # R4 = Fib[i-1] (current)
LI R0, 0         # R0 = 0 (used for "move")

# Store first two Fibonacci numbers
SD R3, R2        # memory[0] = 0
ADDI R2, R2, 1
SD R4, R2        # memory[1] = 1
ADDI R2, R2, 1

fib_loop:
    ADD R5, R3, R4   # R5 = Fib[i-2] + Fib[i-1]
    SD R5, R2        # memory[i] = R5
    ADDI R2, R2, 1   # i++

    # shift previous numbers: R3 = R4, R4 = R5
    ADD R3, R4, R0
    ADD R4, R5, R0

    JLT R2, R1, fib_loop

END
