import numpy as np

def puzzle1(points, folds):
    fold = folds[0]
    fold_axis, fold_idx = {'x':0, 'y':1}[fold.split('=')[0]], int(fold.split('=')[1])
    new_points = set([])
    for point in points:
        if point[fold_axis] < fold_idx:
            new_points.add(point)
        else:
            new_point = [0, 0]
            new_point[not fold_axis] = point[not fold_axis]
            new_point[fold_axis] = 2*fold_idx - point[fold_axis]
            new_points.add(tuple(new_point))
    return len(new_points)

def puzzle2(points, folds):
    for fold in folds:
        fold_axis, fold_idx = {'x':0, 'y':1}[fold.split('=')[0]], int(fold.split('=')[1])
        new_points = set([])
        for point in points:
            if point[fold_axis] < fold_idx:
                new_points.add(point)
            else:
                new_point = [0, 0]
                new_point[not fold_axis] = point[not fold_axis]
                new_point[fold_axis] = 2*fold_idx - point[fold_axis]
                new_points.add(tuple(new_point))
        points = new_points
    
    max_x, max_y = 0, 0
    for point in points:
        x,y = point
        max_x, max_y = max(x, max_x), max(y, max_y)
    
    paper = np.zeros((max_x + 1, max_y + 1))
    for point in points:
        paper[point] = 1

    return paper

if __name__ == "__main__":
    with open('data/day13.data', 'r') as data:
        data_rows = [line.strip() for line in data.readlines()]

    for break_idx, row in enumerate(data_rows):
        if not row: break
    
    points = [(int(row.split(',')[0]), int(row.split(',')[1])) for row in data_rows[0:break_idx]]
    points = set(points)
    folds = [row.replace('fold along ', '') for row in data_rows[break_idx+1:]]

    print(puzzle1(points, folds))
    print(puzzle2(points, folds))