import numpy as np

def puzzle1(fishes):
    for day in range(80):
        fishes = [fish - 1 for fish in fishes]
        for idx, _ in enumerate(fishes):
            if fishes[idx] == -1:
                fishes.append(8)
                fishes[idx] = 6
    
    return len(fishes)

def puzzle2(fishes):
    fish_counts = np.zeros(8 + 1)
    for fish in fishes: fish_counts[fish] += 1

    for day in range(256):
        zero_count = fish_counts[0]
        for i in range(8):
            fish_counts[i] = fish_counts[i+1]
        fish_counts[6] += zero_count
        fish_counts[8] = zero_count

    return np.sum(fish_counts)

if __name__ == "__main__":
    with open('data/day6.data', 'r') as data:
        data_rows = [line.strip() for line in data.readlines()]
        fishes = [int(n.strip()) for n in data_rows[0].split(',')]

    print(puzzle1(fishes))
    print(puzzle2(fishes))
