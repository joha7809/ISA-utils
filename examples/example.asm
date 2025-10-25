
# Example Assembly Program
# This demonstrates the ISA encoder capabilities

# Initialize registers
LI R1, 10
LI R2, 20
LI R3, 0

# Jump to later defined label
JR loop_start

# Arithmetic operations
ADD R3, R1, R2          # R3 = R1 + R2 = 30
SUB R4, R3, R1          # R4 = R3 - R1 = 20
MULT R5, R2, R2         # R5 = R2 * R2 = 400

# Immediate arithmetic  
ADDI R6, R3, 5          # R6 = R3 + 5
SUBI R7, R6, 10         # R7 = R6 - 10

# Logical operations
AND R8, R1, R2
OR R9, R1, R2
NOT R10, R1

# Memory operations
SD R3, R11              # Store R3 to address in R11
LD R12, R11             # Load from address in R11 to R12

# Control flow with labels
loop_start:
    ADDI R1, R1, 1
    JLTV R1, 100, 16     # Jump to instruction 16 if R1 < 100
    
# Conditional jumps  
JEQ R1, R2, 10          # Jump to instruction 10 if R1 == R2
JGT R1, R2, 20       # Jump to instruction 20 if R1 > R2
JETV R1, 42, 5          # Jump to instruction 5 if R1 == 42

# Jump register
JR 25

end_label:
    NOP
    END



