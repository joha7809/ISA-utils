# Store first N primes in memory[0..N-1] (optimized)
# R1 = N (number of primes to store)
# R2 = current candidate
# R3 = count of primes found
# R4 = memory pointer
# R5, R6, R7, R8 = temp registers

LI R1, 800       # N = 800 primes
LI R2, 3         # start candidate from 3
LI R3, 1         # primes found so far (2 is already counted)
LI R4, 1         # memory pointer (M[0] = 2)
LI R0, 0         # zero register

# store first prime
LI R5, 2
SD R5, R0        # M[0] = 2

prime_loop:
    LI R5, 1        # assume candidate is prime
    LI R6, 2        # divisor

sqrt_check:
    MULT R7, R6, R6
    JLT R2, R7, sqrt_done   # if divisor^2 > candidate, done checking
    # check divisibility using repeated subtraction
    ADD R8, R2, R0          # temp = candidate
mod_loop:
    JLT R8, R6, mod_done
    SUB R8, R8, R6
    JR mod_loop
mod_done:
    JEQ R8, R0, not_prime_candidate
    ADDI R6, R6, 1
    JR sqrt_check

sqrt_done:
    # candidate is prime
    SD R2, R4
    ADDI R4, R4, 1
    ADDI R3, R3, 1
    JEQ R3, R1, end_program

not_prime_candidate:
    # move to next candidate
    ADDI R2, R2, 2   # skip even numbers
    JR prime_loop

end_program:
END
