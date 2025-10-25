# ISA

## Arithmetic and Logic Instructions

| Instruction    | Syntax (Example)  | Meaning (Example) |
| -------------- | ----------------- | ----------------- |
| Addition       | `ADD R1, R2, R3`  | `R1 = R2 + R3`    |
| Subtraction    | `SUB R1, R2, R3`  | `R1 = R2 - R3`    |
| Multiplication | `MULT R1, R2, R3` | `R1 = R2 * R3`    |
| Immediate Add. | `ADDI R1, R2, 4`  | `R1 = R2 + 4`     |
| Immediate Sub. | `SUBI R1, R2, 5`  | `R1 = R2 - 5`     |
| Bitwise OR     | `OR R1, R2, R3`   | `R1 = R2 or R3`   |
| Bitwise AND    | `AND R1, R2, R3`  | `R1 = R2 and R3`  |
| Bitwise NOT    | `NOT R1, R2`      | `R1 = not(R2)`    |

## Data Transfer Instructions

| Instruction    | Syntax (Example) | Meaning (Example) |
| -------------- | ---------------- | ----------------- |
| Load immediate | `LI R1, 6`       | `R1 = 6`          |
| Load data      | `LD R1, R2`      | `R1 = memory(R2)` |
| Store data     | `SD R1, R2`      | `memory(R2) = R1` |

## Control and Flow Instructions

| Instruction          | Syntax (Example)  | Meaning (Example)             |
| -------------------- | ----------------- | ----------------------------- |
| Jump                 | `JR 7`            | `goto inst. 7`                |
| Jump if equal        | `JEQ R2, R3, 8`   | `if (R2 == R3) goto inst. 8`  |
| Jump if less than    | `JLTV R2, 19, 9`  | `if (R2 < 19) goto inst. 9`   |
| Jump if greater than | `JGT R2, R3, 10`  | `if (R2 > R3) goto inst. 10`  |
| Jump if eq. to value | `JETV R2, 10, 15` | `if (R2 == 10) goto inst. 15` |
| No operation         | `NOP`             | do nothing                    |
| End execution        | `END`             | terminates execution          |

## Labels

Labels are defined by placing a name followed by a colon at the beginning of a line. They serve as targets for jump instructions.
When parsed into machine code, labels are replaced with the corresponding instruction address, which is determined by the position of the label in the source code.
Example:

```
JEQ R1, R2 end
start:          # This is a label
    ADD R1, R2, R3
    JEQ R1, R4, end
    SUB R1, R1, R5

end:            # Another label
    END
```

# Machine Code Types

Below are the different formats of machine code instructions, showing how bits are allocated for each field.

| **Type** | **Format (Bit Allocation)**                                     |
| :------- | :-------------------------------------------------------------- |
| **R2**   | `OPCODE(5)` · `REGISTER(5)` · `REGISTER(5)`                     |
| **R3**   | `OPCODE(5)` · `REGISTER(5)` · `REGISTER(5)` · `REGISTER(5)`     |
| **RI**   | `OPCODE(5)` · `REGISTER(5)` · `IMMEDIATE(22)`                   |
| **RRI**  | `OPCODE(5)` · `REGISTER(5)` · `REGISTER(5)` · `IMMEDIATE(17)`   |
| **RII**  | `OPCODE(5)` · `REGISTER(5)` · `IMMEDIATE(11)` · `IMMEDIATE(11)` |
| **I**    | `OPCODE(5)` · `IMMEDIATE(27)`                                   |

> 💡 _All bit widths are shown in parentheses. "IMMEDIATE" fields represent literal constant values encoded directly in the instruction._
