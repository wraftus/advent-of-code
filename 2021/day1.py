# puzzle 1: number of increasing measurements
def puzzle_1(depths):
    inc_count = 0
    prev_depth = depths[0]
    for depth in depths[1:]:
        if depth > prev_depth:
            inc_count += 1
        prev_depth = depth
    return inc_count

# puzzle 2: 
def puzzle_2(depths):
    base_idx = 0
    windowed_depths = []
    while base_idx < len(depths):
        for offset in range(4):
            start_idx = min(base_idx + offset, len(depths))
            end_idx = min(base_idx + offset + 3, len(depths))
            windowed_depths.append(sum(depths[start_idx:end_idx]))
        base_idx += 4
    return puzzle_1(windowed_depths)

if __name__ == "__main__":
    with open('data/day1.data', 'r') as data:
        depths = [int(line.strip()) for line in data.readlines()]

    print(puzzle_1(depths))
    print(puzzle_2(depths))