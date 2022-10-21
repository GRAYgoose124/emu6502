from num_tables import *


def is_num(n: str):
    if "0x" in n:
        return "hex"
    if "0b" in n:
        return "bin"
    if n.isdigit():
        return "int"
    return False


def num_conversion(n: str, sep=" "):
    t = is_num(n)
    
    if t == "hex":
        value = int(n[2:], 16)
    elif t == "bin":
        value = int(n[2:], 2)
    elif t == "int":
        value = int(n)
    else:
        return

    return value


def format_conversion(n: str, sep=" "):
    value = num_conversion(n, sep=sep)
    if value is None:
        return 

    return format_number(value)
        

def format_number(value, sep=" "):
    return f"{d(value)}{sep}{h(value)}{sep}{b(value)}"
