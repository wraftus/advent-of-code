import numpy as np
BOARD_LENGTH = 5

def puzzle1(boards, nums):
    done = False
    marked_boards = [np.zeros((BOARD_LENGTH, BOARD_LENGTH), dtype=bool) for _ in boards]
    for num in nums:
        for board_idx, board in enumerate(boards):
            marked_boards[board_idx] = np.logical_or(marked_boards[board_idx], board == num)
            for i in range(BOARD_LENGTH):
                if marked_boards[board_idx][i,:].all() or marked_boards[board_idx][:, i].all():
                    winning_idx = board_idx
                    winning_num = num
                    done = True
                    break
            if done:
                break
        if done:
            break
    
    unmarked_sum = 0
    for i in range(BOARD_LENGTH):
        for j in range(BOARD_LENGTH):
            unmarked_sum += boards[winning_idx][i,j] if not marked_boards[winning_idx][i,j] else 0
    return unmarked_sum * winning_num

def puzzle2(boards, nums):
    boards_won = [False for _ in boards]
    marked_boards = [np.zeros((BOARD_LENGTH, BOARD_LENGTH), dtype=bool) for _ in boards]
    for num in nums:
        for board_idx, board in enumerate(boards):
            if boards_won[board_idx]:
                continue
            
            marked_boards[board_idx] = np.logical_or(marked_boards[board_idx], board == num)
            for i in range(BOARD_LENGTH):
                if marked_boards[board_idx][i, :].all() or marked_boards[board_idx][:, i].all():
                    winning_idx = board_idx
                    winning_num = num
                    boards_won[board_idx] = True

    unmarked_sum = 0
    for i in range(BOARD_LENGTH):
        for j in range(BOARD_LENGTH):
            unmarked_sum += boards[winning_idx][i,j] if not marked_boards[winning_idx][i,j] else 0
    return unmarked_sum * winning_num

with open('data/day4.data', 'r') as data:
    data_rows = [line.strip() for line in data.readlines()]

    nums = [int(n) for n in data_rows[0].split(',')]
    row_idx = 2
    boards = []
    while row_idx < len(data_rows):
        board = []
        for i in range(5):
            board.append([int(n) for n in data_rows[row_idx + i].split()])
        boards.append(np.array(board))
        row_idx += 6

    print(puzzle1(boards, nums))
    print(puzzle2(boards, nums))          



