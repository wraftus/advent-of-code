import numpy as np

def puzzle1(octopi):
    flashes = 0
    for t in range(100):
        not_flashed = np.ones(octopi.shape, dtype=bool)
        octopi = octopi + 1*not_flashed
        while (flashed := octopi * not_flashed > 9).any():
            flashes += np.sum(flashed)
            not_flashed = np.logical_and(not_flashed, np.logical_not(flashed))
            for i in range(octopi.shape[0]):
                for j in range(octopi.shape[1]):
                    if not flashed[i, j]: continue
                    if i-1 >= 0 : octopi[i-1, j] += 1
                    if j-1 >= 0 : octopi[i, j-1] += 1
                    if i+1 < octopi.shape[0] : octopi[i+1, j] += 1
                    if j+1 < octopi.shape[1] : octopi[i, j+1] += 1

                    
                    if i-1 >= 0 and j-1 >= 0 : octopi[i-1, j-1] += 1
                    if i-1 >= 0 and j+1 < octopi.shape[1] : octopi[i-1, j+1] += 1
                    if j-1 >= 0 and i+1 < octopi.shape[0] : octopi[i+1, j-1] += 1
                    if i+1 < octopi.shape[0] and j+1 < octopi.shape[1] : octopi[i+1, j+1] += 1
        octopi = octopi * not_flashed
    return flashes
        
def puzzle2(octopi):
    t = 1
    while True:
        not_flashed = np.ones(octopi.shape, dtype=bool)
        octopi = octopi + 1*not_flashed
        while (flashed := octopi * not_flashed > 9).any():
            not_flashed = np.logical_and(not_flashed, np.logical_not(flashed))
            for i in range(octopi.shape[0]):
                for j in range(octopi.shape[1]):
                    if not flashed[i, j]: continue
                    if i-1 >= 0 : octopi[i-1, j] += 1
                    if j-1 >= 0 : octopi[i, j-1] += 1
                    if i+1 < octopi.shape[0] : octopi[i+1, j] += 1
                    if j+1 < octopi.shape[1] : octopi[i, j+1] += 1

                    
                    if i-1 >= 0 and j-1 >= 0 : octopi[i-1, j-1] += 1
                    if i-1 >= 0 and j+1 < octopi.shape[1] : octopi[i-1, j+1] += 1
                    if j-1 >= 0 and i+1 < octopi.shape[0] : octopi[i+1, j-1] += 1
                    if i+1 < octopi.shape[0] and j+1 < octopi.shape[1] : octopi[i+1, j+1] += 1
        octopi = octopi * not_flashed
        if  np.logical_not(not_flashed).all():
            return t
        t += 1

if __name__ == "__main__":
    with open('data/day11.data', 'r') as data:
        data_rows = [line.strip() for line in data.readlines()]
    octopi = np.array([[int(x) for x in row] for row in data_rows])

    print(puzzle1(octopi))
    print(puzzle2(octopi))