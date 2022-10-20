import columnize

h = lambda x: f"0x{x:02X}"
b = lambda x: f"0b{x:b}"
d = lambda x: f"{x:d}"

bx = [ (b(i), h(i)) for i in range(0xFF)]
bd = [ (b(i), d(i)) for i in range(0xFF)]
dx = [ (d(i), h(i)) for i in range(0xFF)]
db = [ (d(i), b(i)) for i in range(0xFF)]
xb = [ (h(i), b(i)) for i in range(0xFF)]
xd = [ (h(i), d(i)) for i in range(0xFF)]


gh = lambda i: (xb[i], xd[i])
ghs = [gh(i) for i in range(0xFF)]

gb = lambda i: (bx[i], bd[i])
gbs = [gb(i) for i in range(0xFF)]

gd = lambda i: (dx[i], db[i])  
gds = [gd(i) for i in range(0xFF)]

gih = lambda i: (bx[i], dx[i])
gihs = [gih(i) for i in range(0xFF)]

gib = lambda i: (xb[i], db[i])
gibs = [gib(i) for i in range(0xFF)]

gid = lambda i: (xd[i], bd[i])
gids = [gid(i) for i in range(0xFF)]

tables = [("h", ghs), ("b", gbs), ("d", gds), ("ih", gihs), ("ib", gibs), ("id", gids)]
table_names = [("h", "Hex to Binary, Decimal"), ("b", "Binary to Hex, Decimal"), ("d", "Decimal to Hex, Binary"), ("ih", "Hex to Binary, Decimal"), ("ib", "Binary to Hex, Decimal"), ("id", "Decimal to Hex, Binary")]


def format_table(table):
    s = ""
    for i in table:
        s += f"{i[0][0]:4} {i[0][1]:<3} {i[1][0]:<8}\n"
    return s

def print_table(table):
    print(format_table(table))


def select_table():
    print("Select a table to print:")
    for i in table_names:
        print(f"\t{i[0]}: {i[1]}")

    try:
        selection = input("choice: ")
    except KeyboardInterrupt:
        exit()

    match selection:
        case "h":
            print("Hexadecimal to Binary and Decimal")
            print_table(ghs)
        case "b":
            print("Binary to Hexadecimal and Decimal")
            print_table(gbs)
        case "d":
            print("Decimal to Hexadecimal and Binary")
            print_table(gds)
        case "ih":
            print("Hexadecimal to Binary and Decimal")
            print_table(gihs)
        case "ib":
            print("Binary to Hexadecimal and Decimal")
            print_table(gibs)
        case "id":
            print("Decimal to Hexadecimal and Binary")
            print_table(gids)
        case _:
            print("Invalid selection")


def selection_loop():
    while True:
        select_table()
        print()


def columnize_test():
    print(columnize.columnize(list(range(100)), displaywidth=16))

if __name__ == "__main__":
    selection_loop()