pub struct Opcode {
    pub hex: u8,
    pub mnemonic: &'static str,
    pub size: usize,
}

pub fn init_opcodes() -> Vec<Opcode> {
    vec![
        Opcode {
            hex: 0x00u8,
            mnemonic: "NOP",
            size: 1,
        },
        Opcode {
            hex: 0x01u8,
            mnemonic: "LXI B,D16",
            size: 3,
        },
        Opcode {
            hex: 0x02u8,
            mnemonic: "STAX B",
            size: 1,
        },
        Opcode {
            hex: 0x03u8,
            mnemonic: "INX B",
            size: 1,
        },
        Opcode {
            hex: 0x04u8,
            mnemonic: "INR B",
            size: 1,
        },
        Opcode {
            hex: 0x05u8,
            mnemonic: "DCR B",
            size: 1,
        },
        Opcode {
            hex: 0x06u8,
            mnemonic: "MVI B, D8",
            size: 2,
        },
        Opcode {
            hex: 0x07u8,
            mnemonic: "RLC", // Rotate left. The low order bit and the CY flag are both set to the value shifted out of the high order bit
            size: 1,
        },
        Opcode {
            hex: 0x08u8,
            mnemonic: "NOP",
            size: 1,
        },
        Opcode {
            hex: 0x09u8,
            mnemonic: "DAD B",
            size: 1,
        },
        Opcode {
            hex: 0x0Au8,
            mnemonic: "LDAX B",
            size: 1,
        },
        Opcode {
            hex: 0x0Bu8,
            mnemonic: "DCX B",
            size: 1,
        },
        Opcode {
            hex: 0x0Cu8,
            mnemonic: "INR C",
            size: 1,
        },
        Opcode {
            hex: 0x0Du8,
            mnemonic: "DCR C",
            size: 1,
        },
        Opcode {
            hex: 0x0Eu8,
            mnemonic: "MVI C,D8",
            size: 2,
        },
        Opcode {
            hex: 0x0Fu8,
            mnemonic: "RRC",
            size: 1,
        },
        Opcode {
            hex: 0x10u8,
            mnemonic: "NOP",
            size: 1,
        },
        Opcode {
            hex: 0x11u8,
            mnemonic: "LXI D,D16",
            size: 3,
        },
        Opcode {
            hex: 0x12u8,
            mnemonic: "STAX D",
            size: 1,
        },
        Opcode {
            hex: 0x13u8,
            mnemonic: "INX D",
            size: 1,
        },
        Opcode {
            hex: 0x14u8,
            mnemonic: "INR D",
            size: 1,
        },
        Opcode {
            hex: 0x15u8,
            mnemonic: "DCR D",
            size: 1,
        },
        Opcode {
            hex: 0x16u8,
            mnemonic: "MVI D, D8",
            size: 2,
        },
        Opcode {
            hex: 0x17u8,
            mnemonic: "RAL",
            size: 1,
        },
        Opcode {
            hex: 0x18u8,
            mnemonic: "NOP",
            size: 1,
        },
        Opcode {
            hex: 0x19u8,
            mnemonic: "DAD D",
            size: 1,
        },
        Opcode {
            hex: 0x1Au8,
            mnemonic: "LDAX D",
            size: 1,
        },
        Opcode {
            hex: 0x1Bu8,
            mnemonic: "DCX D",
            size: 1,
        },
        Opcode {
            hex: 0x1Cu8,
            mnemonic: "INR E",
            size: 1,
        },
        Opcode {
            hex: 0x1Du8,
            mnemonic: "DCR E",
            size: 1,
        },
        Opcode {
            hex: 0x1Eu8,
            mnemonic: "MVI E,D8",
            size: 2,
        },
        Opcode {
            hex: 0x1Fu8,
            mnemonic: "RAR",
            size: 1,
        },
        Opcode {
            hex: 0x20u8,
            mnemonic: "NOP",
            size: 1,
        },
        Opcode {
            hex: 0x21u8,
            mnemonic: "LXI H,D16",
            size: 3,
        },
        Opcode {
            hex: 0x22u8,
            mnemonic: "SHLD adr",
            size: 3,
        },
        Opcode {
            hex: 0x23u8,
            mnemonic: "INX H",
            size: 1,
        },
        Opcode {
            hex: 0x24u8,
            mnemonic: "INR H",
            size: 1,
        },
        Opcode {
            hex: 0x25u8,
            mnemonic: "DCR H",
            size: 1,
        },
        Opcode {
            hex: 0x26u8,
            mnemonic: "MVI H,D8",
            size: 2,
        },
        Opcode {
            hex: 0x27u8,
            mnemonic: "DAA",
            size: 1,
        },
        Opcode {
            hex: 0x28u8,
            mnemonic: "NOP",
            size: 1,
        },
        Opcode {
            hex: 0x29u8,
            mnemonic: "DAD H",
            size: 1,
        },
        Opcode {
            hex: 0x2Au8,
            mnemonic: "LHLD adr",
            size: 3,
        },
        Opcode {
            hex: 0x2Bu8,
            mnemonic: "DCX H", // HL = HL-1
            size: 1,
        },
        Opcode {
            hex: 0x2Cu8,
            mnemonic: "INR L", // L <- L+1
            size: 1,
        },
        Opcode {
            hex: 0x2Du8,
            mnemonic: "DCR L", // L <- L-1
            size: 1,
        },
        Opcode {
            hex: 0x2Eu8,
            mnemonic: "MVI L", // L <- byte 2
            size: 2,
        },
        Opcode {
            hex: 0x2Fu8,
            mnemonic: "CMA", // A <- !A
            size: 1,
        },
        Opcode {
            hex: 0x30u8,
            mnemonic: "NOP",
            size: 1,
        },
        Opcode {
            hex: 0x31u8,
            mnemonic: "LXI SP, D16", // SP.hi <- byte 3, SP.lo <- byte 2
            size: 3,
        },
        Opcode {
            hex: 0x32u8,
            mnemonic: "STA adr", // (adr) <- A
            size: 3,
        },
        Opcode {
            hex: 0x33u8,
            mnemonic: "INX SP", // SP = SP + 1
            size: 1,
        },
        Opcode {
            hex: 0x34u8,
            mnemonic: "INR M", // (HL) <- (HL)+1
            size: 1,
        },
        Opcode {
            hex: 0x35u8,
            mnemonic: "DCR M", // (HL) <- (HL)-1
            size: 1,
        },
        Opcode {
            hex: 0x36u8,
            mnemonic: "MVI M,D8", // (HL) <- byte 2
            size: 2,
        },
        Opcode {
            hex: 0x37u8,
            mnemonic: "STC", // CY = 1
            size: 1,
        },
        Opcode {
            hex: 0x38u8,
            mnemonic: "NOP",
            size: 1,
        },
        Opcode {
            hex: 0x39u8,
            mnemonic: "DAD SP", // HL = HL + SP
            size: 1,
        },
        Opcode {
            hex: 0x3Au8,
            mnemonic: "LDA adr", // A <- (adr)
            size: 3,
        },
        Opcode {
            hex: 0x3Bu8,
            mnemonic: "DCX SP", // SP = SP-1
            size: 1,
        },
        Opcode {
            hex: 0x3Cu8,
            mnemonic: "INR A", // A <- A+1
            size: 1,
        },
        Opcode {
            hex: 0x3Du8,
            mnemonic: "DCR A", // A <- A-1
            size: 1,
        },
        Opcode {
            hex: 0x3Eu8,
            mnemonic: "MVI A,D8", // A <- byte 2
            size: 2,
        },
        Opcode {
            hex: 0x3Fu8,
            mnemonic: "CMC", // CY=!CY
            size: 1,
        },
        Opcode {
            hex: 0x40u8,
            mnemonic: "MOV B,B", // B <- B
            size: 1,
        },
        Opcode {
            hex: 0x41u8,
            mnemonic: "MOV B,C	1", // B <- C
            size: 1,
        },
        Opcode {
            hex: 0x42u8,
            mnemonic: "MOV B,D", // B <- C
            size: 1,
        },
        Opcode {
            hex: 0x43u8,
            mnemonic: "MOV B,E", // B <- E
            size: 1,
        },
        Opcode {
            hex: 0x44u8,
            mnemonic: "MOV B,H", // B <- H
            size: 1,
        },
        Opcode {
            hex: 0x45u8,
            mnemonic: "MOV B,L", // B <- L
            size: 1,
        },
        Opcode {
            hex: 0x46u8,
            mnemonic: "MOV B,M", // B <- (HL)
            size: 1,
        },
        Opcode {
            hex: 0x47u8,
            mnemonic: "MOV B,A", // B <- A
            size: 1,
        },
        Opcode {
            hex: 0x48u8,
            mnemonic: "MOV C,B", // C <- B
            size: 1,
        },
        Opcode {
            hex: 0x49u8,
            mnemonic: "MOV C,C", // C <- C
            size: 1,
        },
        Opcode {
            hex: 0x4Au8,
            mnemonic: "MOV C,D", // C <- D
            size: 1,
        },
        Opcode {
            hex: 0x4Bu8,
            mnemonic: "MOV C,E", // C <- E
            size: 1,
        },
        Opcode {
            hex: 0x4Cu8,
            mnemonic: "MOV C,H", // C <- H
            size: 1,
        },
        Opcode {
            hex: 0x4Du8,
            mnemonic: "MOV C,L", // C <- L
            size: 1,
        },
        Opcode {
            hex: 0x4Eu8,
            mnemonic: "MOV C,M", // C <- (HL)
            size: 1,
        },
        Opcode {
            hex: 0x4Fu8,
            mnemonic: "MOV C,A", // C <- A
            size: 1,
        },
        Opcode {
            hex: 0x50u8,
            mnemonic: "MOV D,B", // D <- B
            size: 1,
        },
        Opcode {
            hex: 0x51u8,
            mnemonic: "MOV D,C", // D <- C
            size: 1,
        },
        Opcode {
            hex: 0x52u8,
            mnemonic: "MOV D,D", // D <- D
            size: 1,
        },
        Opcode {
            hex: 0x53u8,
            mnemonic: "MOV D,E", // D <- E
            size: 1,
        },
        Opcode {
            hex: 0x54u8,
            mnemonic: "MOV D,H", // D <- H
            size: 1,
        },
        Opcode {
            hex: 0x55u8,
            mnemonic: "MOV D,L", // D <- L
            size: 1,
        },
        Opcode {
            hex: 0x56u8,
            mnemonic: "MOV D,M", // D <- (HL)
            size: 1,
        },
        Opcode {
            hex: 0x57u8,
            mnemonic: "MOV D,A", // D <- A
            size: 1,
        },
        Opcode {
            hex: 0x58u8,
            mnemonic: "MOV E,B", // E <- B
            size: 1,
        },
        Opcode {
            hex: 0x59u8,
            mnemonic: "MOV E,C", // E <- C
            size: 1,
        },
        Opcode {
            hex: 0x5Au8,
            mnemonic: "MOV E,D", // E <- D
            size: 1,
        },
        Opcode {
            hex: 0x5Bu8,
            mnemonic: "MOV E,E", // E <- E
            size: 1,
        },
        Opcode {
            hex: 0x5Cu8,
            mnemonic: "MOV E,H", // E <- H
            size: 1,
        },
        Opcode {
            hex: 0x5Du8,
            mnemonic: "MOV E,L", // E <- L
            size: 1,
        },
        Opcode {
            hex: 0x5Eu8,
            mnemonic: "MOV E,M", // E <- (HL)
            size: 1,
        },
        Opcode {
            hex: 0x5Fu8,
            mnemonic: "MOV E,A", // E <- A
            size: 1,
        },
        Opcode {
            hex: 0x60u8,
            mnemonic: "MOV H,B", // H <- B
            size: 1,
        },
        Opcode {
            hex: 0x61u8,
            mnemonic: "MOV H,C", // H <- C
            size: 1,
        },
        Opcode {
            hex: 0x62u8,
            mnemonic: "MOV H,D", // H <- D
            size: 1,
        },
        Opcode {
            hex: 0x63u8,
            mnemonic: "MOV H,E", // H <- E
            size: 1,
        },
        Opcode {
            hex: 0x64u8,
            mnemonic: "MOV H,H", // H <- H
            size: 1,
        },
        Opcode {
            hex: 0x65u8,
            mnemonic: "MOV H,L", // H <- L
            size: 1,
        },
        Opcode {
            hex: 0x66u8,
            mnemonic: "MOV H,M", // H <- (HL)
            size: 1,
        },
        Opcode {
            hex: 0x67u8,
            mnemonic: "MOV H,A", // H <- A
            size: 1,
        },
        Opcode {
            hex: 0x68u8,
            mnemonic: "MOV L,B", // L <- B
            size: 1,
        },
        Opcode {
            hex: 0x69u8,
            mnemonic: "MOV L,C", // L <- C
            size: 1,
        },
        Opcode {
            hex: 0x6Au8,
            mnemonic: "MOV L,D", // L <- D
            size: 1,
        },
        Opcode {
            hex: 0x6Bu8,
            mnemonic: "MOV L,E", // L <- E
            size: 1,
        },
        Opcode {
            hex: 0x6Cu8,
            mnemonic: "MOV L,H", // L <- H
            size: 1,
        },
        Opcode {
            hex: 0x6Du8,
            mnemonic: "MOV L,L", // L <- L
            size: 1,
        },
        Opcode {
            hex: 0x6Eu8,
            mnemonic: "MOV L,M", // L <- (HL)
            size: 1,
        },
        Opcode {
            hex: 0x6Fu8,
            mnemonic: "MOV L,A", // L <- A
            size: 1,
        },
        Opcode {
            hex: 0x70u8,
            mnemonic: "MOV M,B", // (HL) <- B
            size: 1,
        },
        Opcode {
            hex: 0x71u8,
            mnemonic: "MOV M,C", // (HL) <- C
            size: 1,
        },
        Opcode {
            hex: 0x72u8,
            mnemonic: "MOV M,D", // (HL) <- D
            size: 1,
        },
        Opcode {
            hex: 0x73u8,
            mnemonic: "MOV M,E", // (HL) <- E
            size: 1,
        },
        Opcode {
            hex: 0x74u8,
            mnemonic: "MOV M,H", // (HL) <- H
            size: 1,
        },
        Opcode {
            hex: 0x75u8,
            mnemonic: "MOV M,L", // (HL) <- L
            size: 1,
        },
        Opcode {
            hex: 0x76u8,
            mnemonic: "HLT", // special
            size: 1,
        },
        Opcode {
            hex: 0x77u8,
            mnemonic: "MOV M,A", // (HL) <- A
            size: 1,
        },
        Opcode {
            hex: 0x78u8,
            mnemonic: "MOV A,B", // A <- B
            size: 1,
        },
        Opcode {
            hex: 0x79u8,
            mnemonic: "MOV A,C", // A <- C
            size: 1,
        },
        Opcode {
            hex: 0x7Au8,
            mnemonic: "MOV A,D", // A <- D
            size: 1,
        },
        Opcode {
            hex: 0x7Bu8,
            mnemonic: "MOV A,E", // A <- E
            size: 1,
        },
        Opcode {
            hex: 0x7Cu8,
            mnemonic: "MOV A,H", // A <- H
            size: 1,
        },
        Opcode {
            hex: 0x7Du8,
            mnemonic: "MOV A,L", // A <- L
            size: 1,
        },
        Opcode {
            hex: 0x7Eu8,
            mnemonic: "MOV A,M", // A <- (HL)
            size: 1,
        },
        Opcode {
            hex: 0x7Fu8,
            mnemonic: "MOV A,A", // A <- A
            size: 1,
        },
        Opcode {
            hex: 0x80u8,
            mnemonic: "ADD B", // A <- A + B
            size: 1,
        },
        Opcode {
            hex: 0x81u8,
            mnemonic: "ADD C", // A <- A + C
            size: 1,
        },
        Opcode {
            hex: 0x82u8,
            mnemonic: "ADD D", // A <- A + D
            size: 1,
        },
        Opcode {
            hex: 0x83u8,
            mnemonic: "ADD E", // A <- A + E
            size: 1,
        },
        Opcode {
            hex: 0x84u8,
            mnemonic: "ADD H", // A <- A + H
            size: 1,
        },
        Opcode {
            hex: 0x85u8,
            mnemonic: "ADD L", // A <- A + L
            size: 1,
        },
        Opcode {
            hex: 0x86u8,
            mnemonic: "ADD M", // A <- A + (HL)
            size: 1,
        },
        Opcode {
            hex: 0x87u8,
            mnemonic: "ADD A", // A <- A + A
            size: 1,
        },
        Opcode {
            hex: 0x88u8,
            mnemonic: "ADC B", // A <- A + B + CY
            size: 1,
        },
        Opcode {
            hex: 0x89u8,
            mnemonic: "ADC C", // A <- A + C + CY
            size: 1,
        },
        Opcode {
            hex: 0x8Au8,
            mnemonic: "ADC D", // A <- A + D + CY
            size: 1,
        },
        Opcode {
            hex: 0x8Bu8,
            mnemonic: "ADC E", // A <- A + E + CY
            size: 1,
        },
        Opcode {
            hex: 0x8Cu8,
            mnemonic: "ADC H", // A <- A + H + CY
            size: 1,
        },
        Opcode {
            hex: 0x8Du8,
            mnemonic: "ADC L", // A <- A + L + CY
            size: 1,
        },
        Opcode {
            hex: 0x8Eu8,
            mnemonic: "ADC M", // A <- A + (HL) + CY
            size: 1,
        },
        Opcode {
            hex: 0x8Fu8,
            mnemonic: "ADC A", // A <- A + A + CY
            size: 1,
        },
        Opcode {
            hex: 0x90u8,
            mnemonic: "SUB B", // A <- A - B
            size: 1,
        },
        Opcode {
            hex: 0x91u8,
            mnemonic: "SUB C", // A <- A - C
            size: 1,
        },
        Opcode {
            hex: 0x92u8,
            mnemonic: "SUB D", // A <- A - D
            size: 1,
        },
        Opcode {
            hex: 0x93u8,
            mnemonic: "SUB E", // A <- A - E
            size: 1,
        },
        Opcode {
            hex: 0x94u8,
            mnemonic: "SUB H", // A <- A - H
            size: 1,
        },
        Opcode {
            hex: 0x95u8,
            mnemonic: "SUB L", // A <- A - L
            size: 1,
        },
        Opcode {
            hex: 0x96u8,
            mnemonic: "SUB M", // A <- A - (HL)
            size: 1,
        },
        Opcode {
            hex: 0x97u8,
            mnemonic: "SUB A", // A <- A - A
            size: 1,
        },
        Opcode {
            hex: 0x98u8,
            mnemonic: "SBB B", // A <- A - B - CY
            size: 1,
        },
        Opcode {
            hex: 0x99u8,
            mnemonic: "SBB C", // A <- A - C - CY
            size: 1,
        },
        Opcode {
            hex: 0x9Au8,
            mnemonic: "SBB D", // A <- A - D - CY
            size: 1,
        },
        Opcode {
            hex: 0x9Bu8,
            mnemonic: "SBB E", // A <- A - E - CY
            size: 1,
        },
        Opcode {
            hex: 0x9Cu8,
            mnemonic: "SBB H", // A <- A - H - CY
            size: 1,
        },
        Opcode {
            hex: 0x9Du8,
            mnemonic: "SBB L", // A <- A - L - CY
            size: 1,
        },
        Opcode {
            hex: 0x9Eu8,
            mnemonic: "SBB M", // A <- A - (HL) - CY
            size: 1,
        },
        Opcode {
            hex: 0x9Fu8,
            mnemonic: "SBB A", // A <- A - A - CY
            size: 1,
        },
        Opcode {
            hex: 0xA0u8,
            mnemonic: "ANA B", // A <- A & B
            size: 1,
        },
        Opcode {
            hex: 0xA1u8,
            mnemonic: "ANA C", // A <- A & C
            size: 1,
        },
        Opcode {
            hex: 0xA2u8,
            mnemonic: "ANA D", // A <- A & D
            size: 1,
        },
        Opcode {
            hex: 0xA3u8,
            mnemonic: "ANA E", // A <- A & E
            size: 1,
        },
        Opcode {
            hex: 0xA4u8,
            mnemonic: "ANA H", // A <- A & H
            size: 1,
        },
        Opcode {
            hex: 0xA5u8,
            mnemonic: "ANA L", // A <- A & L
            size: 1,
        },
        Opcode {
            hex: 0xA6u8,
            mnemonic: "ANA M", // A <- A & (HL)
            size: 1,
        },
        Opcode {
            hex: 0xA7u8,
            mnemonic: "ANA A", // A <- A & A
            size: 1,
        },
        Opcode {
            hex: 0xA8u8,
            mnemonic: "XRA B", // A <- A ^ B
            size: 1,
        },
        Opcode {
            hex: 0xA9u8,
            mnemonic: "XRA C", // A <- A ^ C
            size: 1,
        },
        Opcode {
            hex: 0xAAu8,
            mnemonic: "XRA D", // A <- A ^ D
            size: 1,
        },
        Opcode {
            hex: 0xABu8,
            mnemonic: "XRA E", // A <- A ^ E
            size: 1,
        },
        Opcode {
            hex: 0xACu8,
            mnemonic: "XRA H", // A <- A ^ H
            size: 1,
        },
        Opcode {
            hex: 0xADu8,
            mnemonic: "XRA L", // A <- A ^ L
            size: 1,
        },
        Opcode {
            hex: 0xAEu8,
            mnemonic: "XRA M", // A <- A ^ (HL)
            size: 1,
        },
        Opcode {
            hex: 0xAFu8,
            mnemonic: "XRA A", // A <- A ^ A
            size: 1,
        },
        Opcode {
            hex: 0xB0u8,
            mnemonic: "ORA B", // A <- A | B
            size: 1,
        },
        Opcode {
            hex: 0xB1u8,
            mnemonic: "ORA C", // A <- A | C
            size: 1,
        },
        Opcode {
            hex: 0xB2u8,
            mnemonic: "ORA D", // A <- A | D
            size: 1,
        },
        Opcode {
            hex: 0xB3u8,
            mnemonic: "ORA E", // A <- A | E
            size: 1,
        },
        Opcode {
            hex: 0xB4u8,
            mnemonic: "ORA H", // A <- A | H
            size: 1,
        },
        Opcode {
            hex: 0xB5u8,
            mnemonic: "ORA L", // A <- A | L
            size: 1,
        },
        Opcode {
            hex: 0xB6u8,
            mnemonic: "ORA M", // A <- A | (HL)
            size: 1,
        },
        Opcode {
            hex: 0xB7u8,
            mnemonic: "ORA A", // A <- A | A
            size: 1,
        },
        Opcode {
            hex: 0xB8u8,
            mnemonic: "CMP B", // A <- A - B
            size: 1,
        },
        Opcode {
            hex: 0xB9u8,
            mnemonic: "CMP C", // A <- A - C
            size: 1,
        },
        Opcode {
            hex: 0xBAu8,
            mnemonic: "CMP D", // A <- A - D
            size: 1,
        },
        Opcode {
            hex: 0xBBu8,
            mnemonic: "CMP E", // A <- A - E
            size: 1,
        },
        Opcode {
            hex: 0xBCu8,
            mnemonic: "CMP H", // A <- A - H
            size: 1,
        },
        Opcode {
            hex: 0xBDu8,
            mnemonic: "CMP L", // A <- A - L
            size: 1,
        },
        Opcode {
            hex: 0xBEu8,
            mnemonic: "CMP M", // A <- A - (HL)
            size: 1,
        },
        Opcode {
            hex: 0xBFu8,
            mnemonic: "CMP A", // A <- A - A
            size: 1,
        },
        Opcode {
            hex: 0xC0u8,
            mnemonic: "RNZ", // if NZ, RET
            size: 1,
        },
        Opcode {
            hex: 0xC1u8,
            mnemonic: "POP B", // C <- (sp); B <- (sp+1); sp <- sp+2
            size: 1,
        },
        Opcode {
            hex: 0xC2u8,
            mnemonic: "JNZ adr", // if NZ, PC <- adr
            size: 3,
        },
        Opcode {
            hex: 0xC3u8,
            mnemonic: "JMP adr", // PC <= adr
            size: 3,
        },
        Opcode {
            hex: 0xC4u8,
            mnemonic: "CNZ adr", // if NZ, CALL adr
            size: 3,
        },
        Opcode {
            hex: 0xC5u8,
            mnemonic: "PUSH B adr", // (sp-2)<-C; (sp-1)<-B; sp <- sp - 2
            size: 3,
        },
        Opcode {
            hex: 0xC6u8,
            mnemonic: "ADI D8", // Z, S, P, CY, AC	A <- A + byte
            size: 2,
        },
        Opcode {
            hex: 0xC7u8,
            mnemonic: "RST 0", // CALL $0
            size: 1,
        },
        Opcode {
            hex: 0xC8u8,
            mnemonic: "RZ", // if Z, RET
            size: 1,
        },
        Opcode {
            hex: 0xC9u8,
            mnemonic: "RET", // PC.lo <- (sp); PC.hi<-(sp+1); SP <- SP+2
            size: 1,
        },
        Opcode {
            hex: 0xCAu8,
            mnemonic: "JZ adr", // if Z, PC <- adr
            size: 3,
        },
        Opcode {
            hex: 0xCBu8,
            mnemonic: "JMP adr", // PC <= adr - Alternative to 0xC3
            size: 3,
        },
        Opcode {
            hex: 0xCCu8,
            mnemonic: "CZ adr", // if Z, CALL adr
            size: 3,
        },
        Opcode {
            hex: 0xCDu8,
            mnemonic: "CALL adr", // (SP-1)<-PC.hi;(SP-2)<-PC.lo;SP<-SP-2;PC=adr
            size: 3,
        },
        Opcode {
            hex: 0xCEu8,
            mnemonic: "ACI D8", // Z, S, P, CY, AC	A <- A + data + CY
            size: 2,
        },
        Opcode {
            hex: 0xCFu8,
            mnemonic: "RST 1", // CALL $8
            size: 1,
        },
        Opcode {
            hex: 0xD0u8,
            mnemonic: "RNC", // if NCY, RET
            size: 1,
        },
        Opcode {
            hex: 0xD1u8,
            mnemonic: "POP D", // E <- (sp); D <- (sp+1); sp <- sp+2
            size: 1,
        },
        Opcode {
            hex: 0xD2u8,
            mnemonic: "JNC adr", // if NCY, PC<-adr
            size: 3,
        },
        Opcode {
            hex: 0xD3u8,
            mnemonic: "OUT D8", // special
            size: 2,
        },
        Opcode {
            hex: 0xD4u8,
            mnemonic: "CNC adr", // if NCY, CALL adr
            size: 3,
        },
        Opcode {
            hex: 0xD5u8,
            mnemonic: "PUSH D", // (sp-2)<-E; (sp-1)<-D; sp <- sp - 2
            size: 1,
        },
        Opcode {
            hex: 0xD6u8,
            mnemonic: "SUI D8", // Z, S, P, CY, AC	A <- A - data
            size: 2,
        },
        Opcode {
            hex: 0xD7u8,
            mnemonic: "RST 2", // CALL $10
            size: 1,
        },
        Opcode {
            hex: 0xD8u8,
            mnemonic: "RC", // if CY, RET
            size: 1,
        },
        Opcode {
            hex: 0xD9u8,
            mnemonic: "RET", // PC.lo <- (sp); PC.hi<-(sp+1); SP <- SP+2 - Alternative to 0xC9
            size: 1,
        },
        Opcode {
            hex: 0xDAu8,
            mnemonic: "JC adr", // if CY, PC<-adr
            size: 3,
        },
        Opcode {
            hex: 0xDBu8,
            mnemonic: "IN D8", // special
            size: 2,
        },
        Opcode {
            hex: 0xDCu8,
            mnemonic: "CC adr", // is CY, CALL adr
            size: 3,
        },
        Opcode {
            hex: 0xDDu8,
            mnemonic: "CALL adr", // (SP-1)<-PC.hi;(SP-2)<-PC.lo;SP<-SP-2;PC=adr - Alternative to 0xCD
            size: 3,
        },
        Opcode {
            hex: 0xDEu8,
            mnemonic: "SBI D8", // Z, S, P, CY, AC	A <- A - data - CY
            size: 2,
        },
        Opcode {
            hex: 0xDFu8,
            mnemonic: "RST", // CALL $18
            size: 3,
        },
        Opcode {
            hex: 0xE0u8,
            mnemonic: "RPO", // if PO, RET
            size: 1,
        },
        Opcode {
            hex: 0xE1u8,
            mnemonic: "POP H", // L <- (sp); H <- (sp+1); sp <- sp+2
            size: 1,
        },
        Opcode {
            hex: 0xE2u8,
            mnemonic: "JPO adr", // if PO, PC <- adr
            size: 3,
        },
        Opcode {
            hex: 0xE3u8,
            mnemonic: "XTHL", // L <-> (SP); H <-> (SP+1)
            size: 1,
        },
        Opcode {
            hex: 0xE4u8,
            mnemonic: "CPO adr", // if PO, CALL adr
            size: 3,
        },
        Opcode {
            hex: 0xE5u8,
            mnemonic: "PUSH H", // (sp-2)<-L; (sp-1)<-H; sp <- sp - 2
            size: 1,
        },
        Opcode {
            hex: 0xE6u8,
            mnemonic: "ANI D8", // Z, S, P, CY, AC	A <- A & data
            size: 2,
        },
        Opcode {
            hex: 0xE7u8,
            mnemonic: "RST 4", // CALL $20
            size: 1,
        },
        Opcode {
            hex: 0xE8u8,
            mnemonic: "RPE", // if PE, RET
            size: 1,
        },
        Opcode {
            hex: 0xE9u8,
            mnemonic: "PCHL", // PC.hi <- H; PC.lo <- L
            size: 1,
        },
        Opcode {
            hex: 0xEAu8,
            mnemonic: "JPE adr", // if PE, PC <- adr
            size: 3,
        },
        Opcode {
            hex: 0xEBu8,
            mnemonic: "XCHG", // H <-> D; L <-> E
            size: 1,
        },
        Opcode {
            hex: 0xECu8,
            mnemonic: "CPE adr", // if PE, CALL adr
            size: 1,
        },
        Opcode {
            hex: 0xEDu8,
            mnemonic: "CALL adr", // (SP-1)<-PC.hi;(SP-2)<-PC.lo;SP<-SP-2;PC=adr - Alternative to 0xCD
            size: 3,
        },
        Opcode {
            hex: 0xEEu8,
            mnemonic: "XRI D8", // Z, S, P, CY, AC	A <- A ^ data
            size: 2,
        },
        Opcode {
            hex: 0xEFu8,
            mnemonic: "RST 5", // CALL $28
            size: 1,
        },
        Opcode {
            hex: 0xF0u8,
            mnemonic: "RP", // if P, RET
            size: 1,
        },
        Opcode {
            hex: 0xF1u8,
            mnemonic: "POP PSW", // flags <- (sp); A <- (sp+1); sp <- sp+2
            size: 1,
        },
        Opcode {
            hex: 0xF2u8,
            mnemonic: "JP adr", // if P=1 PC <- adr
            size: 3,
        },
        Opcode {
            hex: 0xF3u8,
            mnemonic: "DI", // special
            size: 1,
        },
        Opcode {
            hex: 0xF4u8,
            mnemonic: "CP adr", // if P, PC <- adr
            size: 3,
        },
        Opcode {
            hex: 0xF5u8,
            mnemonic: "PUSH PSW", // (sp-2)<-flags; (sp-1)<-A; sp <- sp - 2
            size: 1,
        },
        Opcode {
            hex: 0xF6u8,
            mnemonic: "ORI D8", // Z, S, P, CY, AC	A <- A | data
            size: 2,
        },
        Opcode {
            hex: 0xF7u8,
            mnemonic: "RST 6", // CALL $30
            size: 1,
        },
        Opcode {
            hex: 0xF8u8,
            mnemonic: "RM", // if M, RET
            size: 1,
        },
        Opcode {
            hex: 0xF9u8,
            mnemonic: "SPHL", // SP=HL
            size: 1,
        },
        Opcode {
            hex: 0xFAu8,
            mnemonic: "JM adr", // if M, PC <- adr
            size: 3,
        },
        Opcode {
            hex: 0xFBu8,
            mnemonic: "EI", // special
            size: 1,
        },
        Opcode {
            hex: 0xFCu8,
            mnemonic: "CM adr", // if M, CALL adr
            size: 3,
        },
        Opcode {
            hex: 0xFDu8,
            mnemonic: "CALL adr", // (SP-1)<-PC.hi;(SP-2)<-PC.lo;SP<-SP-2;PC=adr - Alternative to 0xCD
            size: 3,
        },
        Opcode {
            hex: 0xFEu8,
            mnemonic: "CPI D8", // Z, S, P, CY, AC	A - data
            size: 2,
        },
        Opcode {
            hex: 0xFFu8,
            mnemonic: "RST 7", // CALL $38
            size: 1,
        },
    ]
}