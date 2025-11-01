#Constants
LI     R0, 0              # Black
LI     R1, 255            # White
LI    R2, 20             # image size
LI     R3, 19            # Loops and edges
LI     R4, 400               # Storage of new image

#Loop Variables
LI    R5, 0            #x = 0
xLoop:
LI    R6, 0            #y = 0
yLoop:
MULT     R7, R5, R2;        # R7 = current row/coloumn idk
ADD     R8, R6, R7;          # R8 = current pixel

JEQ     R5, R0, erode;        # Is it on a boarder
JEQ     R5, R3, erode;
JEQ     R6, R0, erode;
JEQ     R6, R3, erode;

LD         R9, R8;             # Is current pixel black
JEQ     R9, R0, erode;

#Is it's Neighbors Black
SUBI    R8, R8, 1;            #x-1 
LD         R9, R8;        
ADDI    R8, R8, 1;            #x neutral
JEQ        R9, R0, erode;

ADDI    R8, R8, 1;             #x+1
LD         R9, R8; 
SUBI    R8, R8, 1;          #x neutral
JEQ        R9, R0, erode;

SUBI    R8, R8, 20;            #y-1
LD         R9, R8; 
ADDI    R8, R8, 20;             #y neutral
JEQ     R9, R0, erode;

ADDI    R8, R8, 20;             #y+1
LD         R9, R8; 
SUBI    R8, R8, 20;             # neutral
JEQ     R9, R0, erode;

ADD     R8, R8, R4;         # Get correct index for new image   
SD         R1, R8;            # Set current pixel to 255 “SCPT255”
JR         loops

erode:
ADD     R8, R8, R4;         # Get correct index for new image
SD         R0, R8;         # Set current pixel to 0      “SCPT0”

loops:
ADDI    R6, R6, 1;            #Loops
JLT     R6, R2, yLoop;        #Loop y
ADDI    R5, R5, 1;  
JLT     R5, R2, xLoop;        #Loop x         
END;
