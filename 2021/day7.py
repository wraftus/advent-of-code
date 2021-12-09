import numpy as np

def puzzle1(crabs):
    cur_pos = round(sum(crabs)/len(crabs))
    while True:
        cur_dist = sum([abs(crab - cur_pos) for crab in crabs])
        down_dist = sum([abs(crab - (cur_pos - 1)) for crab in crabs])
        up_dist = sum([abs(crab - (cur_pos + 1)) for crab in crabs])
        
        if down_dist < cur_dist and up_dist < down_dist:
            raise Exception()
        if down_dist < cur_dist:
            cur_pos = cur_pos - 1
        elif up_dist < cur_dist:
            cur_pos = cur_pos + 1
        else:
            break
    
    print(cur_pos)
    return cur_dist

fuel_cost = lambda crab, cur_pos : sum(range(1, abs(crab-cur_pos) + 1))
def puzzle2(crabs):
    cur_pos = round(sum(crabs)/len(crabs))
    while True:
        cur_dist = sum([fuel_cost(crab, cur_pos) for crab in crabs])
        down_dist = sum([fuel_cost(crab, (cur_pos - 1)) for crab in crabs])
        up_dist = sum([fuel_cost(crab, (cur_pos + 1)) for crab in crabs])
        
        if down_dist < cur_dist and up_dist < down_dist:
            raise Exception()
        if down_dist < cur_dist:
            cur_pos = cur_pos - 1
        elif up_dist < cur_dist:
            cur_pos = cur_pos + 1
        else:
            break
    
    return cur_dist

if __name__ == "__main__":
    with open('data/day7.data', 'r') as data:
        data_rows = [line.strip() for line in data.readlines()]
    crabs = np.array([int(n.strip()) for n in data_rows[0].split(',')])

    print(puzzle1(crabs))
    print(puzzle2(crabs))
