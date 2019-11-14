## Abbreviations
accumulator  Register A
addr         16-bit address quantity
data         8-bit data quantity
data 16      16-bit data quantity
byte 2       The second byte of the instruction
byte 3       The third byte of the instruction
port 8-bit   address of an I/O device
r,rl,r2      One of the registers A,B,C,D,E,H,L
DDD,SSS      The bit pattern designating one of the registers A,B,C,D,E,H,L (DDD=destination, SSS= source)

DOD or SSS     REGISTER NAME
111            A
000            B
001            C

RP          REGISTER PAIR
00          B-C
01          D-E
10          H-L
11          SP

- Should I store the opcodes as structs? or maybe hashmaps?
- I should store how many operands, if any, an instruction takes