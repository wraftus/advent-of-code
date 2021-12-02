def puzzle_1(instrs):
    x, y = 0, 0
    for instr in instrs:
        direc, val = instr.split(' ')
        if direc == 'forward':
            x += int(val)
        elif direc == 'down':
            y += int(val)
        else:
            y -= int(val)
    return x * y

def puzzle_2(instrs):
    aim, x, y = 0, 0, 0
    for instr in instrs:
        direc, val = instr.split(' ')
        if direc == 'forward':
            y += aim * int(val)
            x += int(val)
        elif direc == 'down':
            aim += int(val)
        else:
            aim -= int(val)
    return x * y

if __name__ == "__main__":
    with open('data/day2.data', 'r') as data:
        instrs = [line.strip() for line in data.readlines()]
    print(puzzle_1(instrs))
    print(puzzle_2(instrs))