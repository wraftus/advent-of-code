import numpy as np

def puzzle1(lines, max_x, max_y):
    lines = [(min(x1, x2), min(y1, y2), max(x1, x2), max(y1, y2)) for x1, y1, x2, y2 in lines]
    floor = np.zeros((max_x + 1, max_y + 1))
    for x1, y1, x2, y2 in lines:
        if x1 != x2 and y1 != y2:
            continue
        floor[x1:(x2+1), y1:(y2+1)] += 1
    
    return np.count_nonzero(floor.flatten() >= 2)

def puzzle1(lines, max_x, max_y):
    floor = np.zeros((max_x + 1, max_y + 1))
    for x1, y1, x2, y2 in lines:
        if x1 == x2:
            y1, y2 = min(y1, y2), max(y1, y2)
            floor[x1, y1:(y2+1)] += 1
        if y1 == y2:
            x1, x2 = min(x1, x2), max(x1, x2)
            floor[x1:(x2+1), y1] += 1
        if abs(x1 - x2) == abs(y1 - y2):
            xs = np.linspace(x1, x2, abs(x1 - x2) + 1, dtype='int')
            ys = np.linspace(y1, y2, abs(y1 - y2) + 1, dtype='int')
            for x, y in zip(xs, ys):
                floor[x, y] += 1
    
    return np.count_nonzero(floor.flatten() >= 2)

with open('data/day5.data', 'r') as data:
    data_rows = [line.strip() for line in data.readlines()]

    processed_rows = [[point.strip().split(",") for point in  row.split(' -> ')] for row in data_rows]
    lines = [(int(row[0][0]), int(row[0][1]), int(row[1][0]), int(row[1][1])) for row in processed_rows]

    max_x, max_y = 0, 0
    for x1, y1, x2, y2 in lines:
        max_x = x1 if x1 > max_x else max_x
        max_x = x2 if x2 > max_x else max_x
        max_y = y1 if y1 > max_y else max_y
        max_y = y2 if y2 > max_y else max_y

    print(puzzle1(lines, max_x, max_y))