import numpy as np

def puzzle1(height_map):
    risk_sum = 0
    for x in range(height_map.shape[0]):
        for y in range(height_map.shape[1]):
            curr_height = height_map[x, y]
            if x - 1 >= 0 and curr_height >= height_map[x - 1, y] : continue
            if y - 1 >= 0 and curr_height >=  height_map[x, y - 1] : continue
            if x + 1 < height_map.shape[0] and curr_height >=  height_map[x + 1, y] : continue
            if y + 1 < height_map.shape[1] and curr_height >=  height_map[x, y + 1] : continue
            risk_sum += curr_height + 1
    return risk_sum

def puzzle2(height_map):
    minima = []
    for x in range(height_map.shape[0]):
        for y in range(height_map.shape[1]):
            curr_height = height_map[x, y]
            if x - 1 >= 0 and curr_height >= height_map[x - 1, y] : continue
            if y - 1 >= 0 and curr_height >=  height_map[x, y - 1] : continue
            if x + 1 < height_map.shape[0] and curr_height >=  height_map[x + 1, y] : continue
            if y + 1 < height_map.shape[1] and curr_height >=  height_map[x, y + 1] : continue
            minima.append((x, y))
    
    basin_sizes = []
    for x, y in minima:
        in_basin = np.zeros(height_map.shape, dtype=bool)
        to_explore = [(x, y)]
        for x, y in to_explore:
            curr_height = height_map[x, y]
            if curr_height == 9 or in_basin[x, y]: continue
            if x - 1 >= 0 and curr_height < height_map[x - 1, y] : to_explore.append((x - 1, y))
            if y - 1 >= 0 and curr_height <  height_map[x, y - 1] : to_explore.append((x, y - 1))
            if x + 1 < height_map.shape[0] and curr_height <  height_map[x + 1, y] : to_explore.append((x + 1, y))
            if y + 1 < height_map.shape[1] and curr_height <  height_map[x, y + 1] : to_explore.append((x, y + 1))
            in_basin[x, y] = True
        basin_sizes.append(sum(in_basin.flatten()))
    basin_sizes.sort(reverse=True)

    return np.prod([basin_sizes[i] for i in range(3)])

if __name__ == "__main__":
    with open('data/day9.data', 'r') as data:
        data_rows = [line.strip() for line in data.readlines()]
    height_map = np.array([[int(n) for n in row] for row in data_rows])
    
    print(puzzle1(height_map))
    print(puzzle2(height_map))