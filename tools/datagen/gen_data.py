import json


N_VALID = 151
assert N_VALID == 151

N_VALID_NO_DUPES = 56
assert N_VALID_NO_DUPES == 56

valid_opcodes = [
    0x00, 0x01, 0x05, 0x06, 0x08, 0x09, 0x0A, 0x0D, 0x0E, 0x10, 0x11, 0x15, 0x16, 0x18, 0x19,
    0x1D, 0x1E, 0x20, 0x21, 0x24, 0x25, 0x26, 0x28, 0x29, 0x2A, 0x2C, 0x2D, 0x2E, 0x30, 0x31,
    0x35, 0x36, 0x38, 0x39, 0x3D, 0x3E, 0x40, 0x41, 0x45, 0x46, 0x48, 0x49, 0x4A, 0x4C, 0x4D,
    0x4E, 0x50, 0x51, 0x55, 0x56, 0x58, 0x59, 0x5D, 0x5E, 0x60, 0x61, 0x65, 0x66, 0x68, 0x69,
    0x6A, 0x6C, 0x6D, 0x6E, 0x70, 0x71, 0x75, 0x76, 0x78, 0x79, 0x7D, 0x7E, 0x81, 0x84, 0x85,
    0x86, 0x88, 0x8A, 0x8C, 0x8D, 0x8E, 0x90, 0x91, 0x94, 0x95, 0x96, 0x98, 0x99, 0x9A, 0x9D,
    0xA0, 0xA1, 0xA2, 0xA4, 0xA5, 0xA6, 0xA8, 0xA9, 0xAA, 0xAC, 0xAD, 0xAE, 0xB0, 0xB1, 0xB4,
    0xB5, 0xB6, 0xB8, 0xB9, 0xBA, 0xBC, 0xBD, 0xBE, 0xC0, 0xC1, 0xC4, 0xC5, 0xC6, 0xC8, 0xC9,
    0xCA, 0xCC, 0xCD, 0xCE, 0xD0, 0xD1, 0xD5, 0xD6, 0xD8, 0xD9, 0xDD, 0xDE, 0xE0, 0xE1, 0xE4,
    0xE5, 0xE6, 0xE8, 0xE9, 0xEA, 0xEC, 0xED, 0xEE, 0xF0, 0xF1, 0xF5, 0xF6, 0xF8, 0xF9, 0xFD,
    0xFE
]
assert len(valid_opcodes) == N_VALID

all_opcode_names = [["BRK", 0], ["ORA", 1], ["NOP", 2], ["NOP", 3], ["NOP", 4], ["ORA", 5], ["ASL", 6], ["NOP", 7], ["PHP", 8], ["ORA", 9], ["ASL", 10], ["NOP", 11], ["NOP", 12], ["ORA", 13], ["ASL", 14], ["NOP", 15], ["BPL", 16], ["ORA", 17], ["NOP", 18], ["NOP", 19], ["NOP", 20], ["ORA", 21], ["ASL", 22], ["NOP", 23], ["CLC", 24], ["ORA", 25], ["NOP", 26], ["NOP", 27], ["NOP", 28], ["ORA", 29], ["ASL", 30], ["NOP", 31], ["JSR", 32], ["AND", 33], ["NOP", 34], ["NOP", 35], ["BIT", 36], ["AND", 37], ["ROL", 38], ["NOP", 39], ["PLP", 40], ["AND", 41], ["ROL", 42], ["NOP", 43], ["BIT", 44], ["AND", 45], ["ROL", 46], ["NOP", 47], ["BMI", 48], ["AND", 49], ["NOP", 50], ["NOP", 51], ["NOP", 52], ["AND", 53], ["ROL", 54], ["NOP", 55], ["SEC", 56], ["AND", 57], ["NOP", 58], ["NOP", 59], ["NOP", 60], ["AND", 61], ["ROL", 62], ["NOP", 63], ["RTI", 64], ["EOR", 65], ["NOP", 66], ["NOP", 67], ["NOP", 68], ["EOR", 69], ["LSR", 70], ["NOP", 71], ["PHA", 72], ["EOR", 73], ["LSR", 74], ["NOP", 75], ["JMP", 76], ["EOR", 77], ["LSR", 78], ["NOP", 79], ["BVC", 80], ["EOR", 81], ["NOP", 82], ["NOP", 83], ["NOP", 84], ["EOR", 85], ["LSR", 86], ["NOP", 87], ["CLI", 88], ["EOR", 89], ["NOP", 90], ["NOP", 91], ["NOP", 92], ["EOR", 93], ["LSR", 94], ["NOP", 95], ["RTS", 96], ["ADC", 97], ["NOP", 98], ["NOP", 99], ["NOP", 100], ["ADC", 101], ["ROR", 102], ["NOP", 103], ["PLA", 104], ["ADC", 105], ["ROR", 106], ["NOP", 107], ["JMP", 108], ["ADC", 109], ["ROR", 110], ["NOP", 111], ["BVS", 112], ["ADC", 113], ["NOP", 114], ["NOP", 115], ["NOP", 116], ["ADC", 117], ["ROR", 118], ["NOP", 119], ["SEI", 120], ["ADC", 121], ["NOP", 122], ["NOP", 123], ["NOP", 124], ["ADC", 125], ["ROR", 126], ["NOP", 127], ["NOP", 128], ["STA", 129], ["NOP", 130], ["NOP", 131], ["STY", 132], ["STA", 133], ["STX", 134], ["NOP", 135], ["DEY", 136], ["NOP", 137], ["TXA", 138], ["NOP", 139], ["STY", 140], ["STA", 141], ["STX", 142], ["NOP", 143], ["BCC", 144], ["STA", 145], ["NOP", 146], ["NOP", 147], ["STY", 148], ["STA", 149], ["STX", 150], ["NOP", 151], ["TYA", 152], ["STA", 153], ["TXS", 154], ["NOP", 155], ["NOP", 156], ["STA", 157], ["NOP", 158], ["NOP", 159], ["LDY", 160], ["LDA", 161], ["LDX", 162], ["NOP", 163], ["LDY", 164], ["LDA", 165], ["LDX", 166], ["NOP", 167], ["TAY", 168], ["LDA", 169], ["TAX", 170], ["NOP", 171], ["LDY", 172], ["LDA", 173], ["LDX", 174], ["NOP", 175], ["BCS", 176], ["LDA", 177], ["NOP", 178], ["NOP", 179], ["LDY", 180], ["LDA", 181], ["LDX", 182], ["NOP", 183], ["CLV", 184], ["LDA", 185], ["TSX", 186], ["NOP", 187], ["LDY", 188], ["LDA", 189], ["LDX", 190], ["NOP", 191], ["CPY", 192], ["CMP", 193], ["NOP", 194], ["NOP", 195], ["CPY", 196], ["CMP", 197], ["DEC", 198], ["NOP", 199], ["INY", 200], ["CMP", 201], ["DEX", 202], ["NOP", 203], ["CPY", 204], ["CMP", 205], ["DEC", 206], ["NOP", 207], ["BNE", 208], ["CMP", 209], ["NOP", 210], ["NOP", 211], ["NOP", 212], ["CMP", 213], ["DEC", 214], ["NOP", 215], ["CLD", 216], ["CMP", 217], ["NOP", 218], ["NOP", 219], ["NOP", 220], ["CMP", 221], ["DEC", 222], ["NOP", 223], ["CPX", 224], ["SBC", 225], ["NOP", 226], ["NOP", 227], ["CPX", 228], ["SBC", 229], ["INC", 230], ["NOP", 231], ["INX", 232], ["SBC", 233], ["NOP", 234], ["NOP", 235], ["CPX", 236], ["SBC", 237], ["INC", 238], ["NOP", 239], ["BEQ", 240], ["SBC", 241], ["NOP", 242], ["NOP", 243], ["NOP", 244], ["SBC", 245], ["INC", 246], ["NOP", 247], ["SED", 248], ["SBC", 249], ["NOP", 250], ["NOP", 251], ["NOP", 252], ["SBC", 253], ["INC", 254], ["NOP", 255]]
assert len(all_opcode_names) == 256
valid_modes = ["Implied", "IndirectX", "ZeroPage", "ZeroPage", "Implied", "Immediate", "Accumulator", "Absolute", "Absolute", "Relative", "IndirectY", "ZeroPageX", "ZeroPageX", "Implied", "AbsoluteY", "AbsoluteX", "AbsoluteX", "Absolute", "IndirectX", "ZeroPage", "ZeroPage", "ZeroPage", "Implied", "Immediate", "Accumulator", "Absolute", "Absolute", "Absolute", "Relative", "IndirectY", "ZeroPageX", "ZeroPageX", "Implied", "AbsoluteY", "AbsoluteX", "AbsoluteX", "Implied", "IndirectX", "ZeroPage", "ZeroPage", "Implied", "Immediate", "Accumulator", "Absolute", "Absolute", "Absolute", "Relative", "IndirectY", "ZeroPageX", "ZeroPageX", "Implied", "AbsoluteY", "AbsoluteX", "AbsoluteX", "Implied", "IndirectX", "ZeroPage", "ZeroPage", "Implied", "Immediate", "Accumulator", "Indirect", "Absolute", "Absolute", "Relative", "IndirectY", "ZeroPageX", "ZeroPageX", "Implied", "AbsoluteY", "AbsoluteX", "AbsoluteX", "IndirectX", "ZeroPage", "ZeroPage", "ZeroPage", "Implied", "Implied", "Absolute", "Absolute", "Absolute", "Relative", "IndirectY", "ZeroPageX", "ZeroPageX", "ZeroPageY", "Implied", "AbsoluteY", "Implied", "AbsoluteX", "Immediate", "IndirectX", "Immediate", "ZeroPage", "ZeroPage", "ZeroPage", "Implied", "Immediate", "Implied", "Absolute", "Absolute", "Absolute", "Relative", "IndirectY", "ZeroPageX", "ZeroPageX", "ZeroPageY", "Implied", "AbsoluteY", "Implied", "AbsoluteX", "AbsoluteX", "AbsoluteY", "Immediate", "IndirectX", "ZeroPage", "ZeroPage", "ZeroPage", "Implied", "Immediate", "Implied", "Absolute", "Absolute", "Absolute", "Relative", "IndirectY", "ZeroPageX", "ZeroPageX", "Implied", "AbsoluteY", "AbsoluteX", "AbsoluteX", "Immediate", "IndirectX", "ZeroPage", "ZeroPage", "ZeroPage", "Implied", "Immediate", "Implied", "Absolute", "Absolute", "Absolute", "Relative", "IndirectY", "ZeroPageX", "ZeroPageX", "Implied", "AbsoluteY", "AbsoluteX", "AbsoluteX" ]
assert len(valid_modes) == N_VALID
all_valid_ops = {"BRK": [[0, "Implied"]], "ORA": [[1, "IndirectX"], [5, "ZeroPage"], [9, "Immediate"], [13, "Absolute"], [17, "IndirectY"], [21, "ZeroPageX"], [25, "AbsoluteY"], [29, "AbsoluteX"]], "ASL": [[6, "ZeroPage"], [10, "Accumulator"], [14, "Absolute"], [22, "ZeroPageX"], [30, "AbsoluteX"]], "PHP": [[8, "Implied"]], "BPL": [[16, "Relative"]], "CLC": [[24, "Implied"]], "JSR": [[32, "Absolute"]], "AND": [[33, "IndirectX"], [37, "ZeroPage"], [41, "Immediate"], [45, "Absolute"], [49, "IndirectY"], [53, "ZeroPageX"], [57, "AbsoluteY"], [61, "AbsoluteX"]], "BIT": [[36, "ZeroPage"], [44, "Absolute"]], "ROL": [[38, "ZeroPage"], [42, "Accumulator"], [46, "Absolute"], [54, "ZeroPageX"], [62, "AbsoluteX"]], "PLP": [[40, "Implied"]], "BMI": [[48, "Relative"]], "SEC": [[56, "Implied"]], "RTI": [[64, "Implied"]], "EOR": [[65, "IndirectX"], [69, "ZeroPage"], [73, "Immediate"], [77, "Absolute"], [81, "IndirectY"], [85, "ZeroPageX"], [89, "AbsoluteY"], [93, "AbsoluteX"]], "LSR": [[70, "ZeroPage"], [74, "Accumulator"], [78, "Absolute"], [86, "ZeroPageX"], [94, "AbsoluteX"]], "PHA": [[72, "Implied"]], "JMP": [[76, "Absolute"], [108, "Indirect"]], "BVC": [[80, "Relative"]], "CLI": [[88, "Implied"]], "RTS": [[96, "Implied"]], "ADC": [[97, "IndirectX"], [101, "ZeroPage"], [105, "Immediate"], [109, "Absolute"], [113, "IndirectY"], [117, "ZeroPageX"], [121, "AbsoluteY"], [125, "AbsoluteX"]], "ROR": [[102, "ZeroPage"], [106, "Accumulator"], [110, "Absolute"], [118, "ZeroPageX"], [126, "AbsoluteX"]], "PLA": [[104, "Implied"]], "BVS": [[112, "Relative"]], "SEI": [[120, "Implied"]], "STA": [[129, "IndirectX"], [133, "ZeroPage"], [141, "Absolute"], [145, "IndirectY"], [149, "ZeroPageX"], [153, "AbsoluteY"], [157, "AbsoluteX"]], "STY": [[132, "ZeroPage"], [140, "Absolute"], [148, "ZeroPageX"]], "STX": [[134, "ZeroPage"], [142, "Absolute"], [150, "ZeroPageY"]], "DEY": [[136, "Implied"]], "TXA": [[138, "Implied"]], "BCC": [[144, "Relative"]], "TYA": [[152, "Implied"]], "TXS": [[154, "Implied"]], "LDY": [[160, "Immediate"], [164, "ZeroPage"], [172, "Absolute"], [180, "ZeroPageX"], [188, "AbsoluteX"]], "LDA": [[161, "IndirectX"], [165, "ZeroPage"], [169, "Immediate"], [173, "Absolute"], [177, "IndirectY"], [181, "ZeroPageX"], [185, "AbsoluteY"], [189, "AbsoluteX"]], "LDX": [[162, "Immediate"], [166, "ZeroPage"], [174, "Absolute"], [182, "ZeroPageY"], [190, "AbsoluteY"]], "TAY": [[168, "Implied"]], "TAX": [[170, "Implied"]], "BCS": [[176, "Relative"]], "CLV": [[184, "Implied"]], "TSX": [[186, "Implied"]], "CPY": [[192, "Immediate"], [196, "ZeroPage"], [204, "Absolute"]], "CMP": [[193, "IndirectX"], [197, "ZeroPage"], [201, "Immediate"], [205, "Absolute"], [209, "IndirectY"], [213, "ZeroPageX"], [217, "AbsoluteY"], [221, "AbsoluteX"]], "DEC": [[198, "ZeroPage"], [206, "Absolute"], [214, "ZeroPageX"], [222, "AbsoluteX"]], "INY": [[200, "Implied"]], "DEX": [[202, "Implied"]], "BNE": [[208, "Relative"]], "CLD": [[216, "Implied"]], "CPX": [[224, "Immediate"], [228, "ZeroPage"], [236, "Absolute"]], "SBC": [[225, "IndirectX"], [229, "ZeroPage"], [233, "Immediate"], [237, "Absolute"], [241, "IndirectY"], [245, "ZeroPageX"], [249, "AbsoluteY"], [253, "AbsoluteX"]], "INC": [[230, "ZeroPage"], [238, "Absolute"], [246, "ZeroPageX"], [254, "AbsoluteX"]], "INX": [[232, "Implied"]], "NOP": [[234, "Implied"]], "BEQ": [[240, "Relative"]], "SED": [[248, "Implied"]]}
assert len(all_valid_ops) == N_VALID_NO_DUPES


def check_valid_names_only():
    ll = [[v for opv in v if opv[0] in valid_opcodes] for k, v in enumerate(all_valid_ops.items())]
    assert len(all_valid_ops.keys()) == len(ll)
        
                

def intersect_all_opcodes_with_valid():
    intersection = {}
    counter = 0

    for n, v in filter(lambda k: k[0] != "NOP" or k[1] == 0xEA, all_opcode_names):
        if v in valid_opcodes:
            new = (v, valid_modes[valid_opcodes.index(v)])
            if n in intersection:
                intersection[n].append(new)
            else:
                intersection[n] = [new]
            counter+=1

    assert len(intersection) == N_VALID_NO_DUPES
    with open("opcodes.json", "w") as f:
        json.dump(intersection, f)


def main():
    # check_valid_names_only()
    # intersect_all_opcodes_with_valid()
    pass

if __name__ == "__main__":
    main()