#Constants
LI     R0, 0              # Black
LI     R1, 255            # White
LI    R2, 20             # image size
LI     R3, 19            # Loops and edges
LI     R4, 400               # Storage of new image

#Loop Variables
LI    R5, 1            #x = 0
xLoop:
LI    R6, 1            #y = 0
MULT     R7, R5, R2;        # R7 = current row/coloumn idk
yLoop:
ADD     R8, R6, R7;          # R8 = current pixel

LD         R9, R8;             # Is current pixel black
JEQ     R9, R0, erode;

# JEQ     R5, R0, erode;        # Is it on a boarder
# JEQ     R5, R3, erode;
# JEQ     R6, R0, erode;
# JEQ     R6, R3, erode;

#Is it's Neighbors Black
SUBI    R9, R8, 1;            #x-1 
LD         R10, R9;
JEQ        R10 R0, erode;

ADDI    R9, R8, 1;             #x+1
LD         R10, R9; 
JEQ        R10, R0, erode;

SUBI    R9, R8, 20;            #y-1
LD         R10, R9; 
JEQ     R10, R0, erode;

ADDI    R9, R8, 20;             #y+1
LD         R10, R9; 
JEQ     R10, R0, erode;

ADD     R8, R8, R4;         # Get correct index for new image
SD         R1, R8;            # Set current pixel to 255 “SCPT255”
JR         loops

erode:
loops:
ADDI    R6, R6, 1;            #Loops
JLT     R6, R3, yLoop;        #Loop y
ADDI    R5, R5, 1;
JLT     R5, R3, xLoop;        #Loop x
END;
