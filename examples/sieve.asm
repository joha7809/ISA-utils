
        # --------------------------
        # Setup constants, input:
        # R1 should contain N before starting the program.
        # Example: LI R1, 30  (to find primes <= 30)
        # --------------------------

LI R1, 100; # set R1 to N=100.

start:
    LI   R0, 0          # R0 = 0 (machine-type: RI)
    LI   R6, 1          # R6 = 1
    LI   R7, 2          # R7 = 2
    LI   R2, 200        # base = 200 (sieve array start)
    LI   R12, 500       # out_base = 500 (where we write primes)
    ADDI R9, R1, 1      # R9 = N + 1

    # ------------- initialize sieve array: for k = 0..N set memory[base+k] = 0
    LI   R3, 0          # k = 0
init_loop:
    ADD  R8, R2, R3     # addr = base + k          (ADD is R3-type)
    SD   R0, R8         # memory[addr] = 0        (LD/SD is R2-type)
    ADDI R3, R3, 1      # k++
    JEQ  R3, R9, init_done  # if k == N+1 done
    JR   init_loop

init_done:
    # mark 0 and 1 as composite (if within range)
    ADD  R8, R2, R0     # addr = base + 0
    SD   R6, R8         # memory[base+0] = 1
    LI   R3, 1
    ADD  R8, R2, R3     # addr = base + 1
    SD   R6, R8         # memory[base+1] = 1

    # ------------- main sieve: for i = 2..N
    LI   R3, 2          # i = 2

sieve_loop:
    JGT  R3, R1, sieve_done    # if i > N goto sieve_done

    # load memory[base + i] into R5
    ADD  R8, R2, R3           # addr = base + i
    LD   R5, R8               # R5 = memory[addr]

    JEQ  R5, R0, mark_multiples  # if memory[base+i] == 0 -> prime -> mark multiples
    # else not prime -> skip to next i
    ADDI R3, R3, 1
    JR   sieve_loop

mark_multiples:
    MULT R4, R7, R3           # j = 2 * i
mark_mult_loop:
    JGT  R4, R1, after_mark   # if j > N skip inner loop
    ADD  R8, R2, R4           # addr = base + j
    SD   R6, R8               # memory[addr] = 1 (mark composite)
    ADD  R4, R4, R3           # j = j + i
    JR   mark_mult_loop

after_mark:
    ADDI R3, R3, 1            # i++
    JR   sieve_loop

sieve_done:
    # ------------- collect primes into output array at out_base
    LI   R13, 0               # out_index = 0
    LI   R3, 2                # i = 2

collect_loop:
    JGT  R3, R1, collect_done    # if i > N done collecting
    ADD  R8, R2, R3              # addr = base + i
    LD   R5, R8                  # R5 = memory[addr]
    JEQ  R5, R0, store_prime     # if memory[base+i] == 0 -> prime

    # not prime -> next i
    ADDI R3, R3, 1
    JR   collect_loop

store_prime:
    ADD  R14, R12, R13          # out_addr = out_base + out_index
    SD   R3, R14                # store i into output list
    ADDI R13, R13, 1           # out_index++
    ADDI R3, R3, 1
    JR   collect_loop

collect_done:
    END
