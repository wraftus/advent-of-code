def puzzle1(init, rules):
    curr = init
    for step in range(10):
        new_string = []
        for idx in range(len(curr) - 1):
            new_string.append(curr[idx])
            if curr[idx:(idx+2)] in rules.keys():
                new_string.append(rules[curr[idx:(idx+2)]])
        new_string.append(curr[-1])
        curr = ''.join(new_string)

    (curr := [c for c in curr]).sort()
    max_count, min_count = 0, len(curr)
    prev_c, count = curr[0], -1
    for idx, c in enumerate(curr):
        count += 1
        if c == prev_c: continue
        if count > max_count: max_count = count
        if count < min_count: min_count = count
        count = 0
        prev_c = c
    
    return max_count - min_count

def puzzle2(init, rules):
    letter_counts = {}
    for l in init: 
        if l not in letter_counts: letter_counts[l] = 0
        letter_counts[l] += 1
    for _, mid in rules.items(): 
        if mid not in letter_counts: letter_counts[mid] = 0

    pair_counts = {}
    for pair in rules.keys(): pair_counts[pair] = 0
    for idx in range(len(init) - 1):
        if init[idx:(idx+2)] in rules.keys():
            pair_counts[init[idx:(idx+2)]] += 1
    
    for step in range(40):
        new_pair_counts = {}
        for pair in rules.keys(): new_pair_counts[pair] = 0
        for pair, count in pair_counts.items():
            middle = rules[pair]
            letter_counts[middle] += count

            new_pairs = [''.join([pair[0], middle]), ''.join([middle, pair[1]])]
            for new_pair in new_pairs: new_pair_counts[new_pair] += count
        pair_counts = new_pair_counts
    (counts := [count for _, count in letter_counts.items()]).sort()
    return counts[-1] - counts[0]

if __name__ == "__main__":
    with open('data/day14.data', 'r') as data:
        data_rows = [line.strip() for line in data.readlines()]
    
    init = data_rows[0]
    rules = {}
    for row in data_rows[2:]: rules[row.split(' -> ')[0]] = row.split(' -> ')[1]

    print(puzzle1(init, rules))
    print(puzzle2(init, rules))