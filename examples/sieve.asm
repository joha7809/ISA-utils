# Sieve of Eratosthenes up to N=100
# Memory layout:
# for i in  0..N  -> (i = prime, 0 = not prime)

# Initialize N and loop counters
LI R1, 100        # R1 = N
LI R2, 2         # R2 = i (current prime candidate)
LI R3, 2         # R3 = j (inner loop counter)

# Initialize prime flags to 1
init_flags:
    SD R3, R3    # memory[R3] = 1 (assume all are prime initially)
    ADDI R3, R3, 1
    JLT R3, R1, init_flags

# Outer loop over i (prime candidates)
outer_loop:
    LD R4, R2         # R4 = memory[R2] (check if R2 is prime)
    JEQ R4, R0, skip_outer   # if not prime, skip marking multiples

    # Inner loop: mark multiples of i as not prime
    ADD R5, R2, R2     # R5 = 2*i (start marking from 2*i)
inner_loop:
    JLT R5, R1, mark_multiple
    JR skip_outer_inner

mark_multiple:
    LI R6, 0
    SD R6, R5         # memory[R5] = 0 (not prime)
    ADD R5, R5, R2    # next multiple
    JR inner_loop

skip_outer_inner:
    ADDI R2, R2, 1
    JLT R2, R1, outer_loop

skip_outer:
END
