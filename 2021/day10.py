
close_to_open_map = {')':'(', ']':'[', '}':'{', '>':'<'}
close_to_cost_map = {')':3, ']':57, '}':1197, '>':25137}
def puzzle1(data_rows):
    cost = 0
    for row in data_rows:
        brackets = []
        for bracket in row:
            if bracket in ['(', '[', '{', '<']:
                brackets.append(bracket)
            else:
                open_bracket = close_to_open_map[bracket]
                if open_bracket != brackets[-1]:
                    cost += close_to_cost_map[bracket]
                    break
                brackets = brackets[0:-1]
    return cost

open_to_close_map = {'(':')', '[':']', '{':'}', '<':'>'}
close_to_score_map = {')':1, ']':2, '}':3, '>':4}
def puzzle2(data_rows):
    good_rows = [True for row in data_rows]
    for idx, row in enumerate(data_rows):
        brackets = []
        for bracket in row:
            if bracket in ['(', '[', '{', '<']:
                brackets.append(bracket)
            else:
                open_bracket = close_to_open_map[bracket]
                if open_bracket != brackets[-1]:
                    good_rows[idx] = False
                    break
                brackets = brackets[0:-1]
    
    data_rows = [data_rows[idx] for idx, good in enumerate(good_rows) if good]
    scores = []
    for row in data_rows:
        brackets = []
        for bracket in row:
            if bracket in ['(', '[', '{', '<']:
                brackets.append(bracket)
            else:
                brackets = brackets[0:-1]

        score = 0
        brackets.reverse()
        for bracket in brackets:
            score *= 5
            close_bracket = open_to_close_map[bracket]
            score += close_to_score_map[close_bracket]
        scores.append(score)
    scores.sort()

    return scores[len(scores)//2]

if __name__ == "__main__":
    with open('data/day10.data', 'r') as data:
        data_rows = [line.strip() for line in data.readlines()]
    
    print(puzzle1(data_rows))
    print(puzzle2(data_rows))