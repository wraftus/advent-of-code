
def puzzle1(nodes, edges):
    paths = 0
    search_states = [('start', ['start'], ['start'])]
    while len(search_states) > 0:
        curr_node, path, visited_small = search_states[0]
        for edge in edges[curr_node]:
            if edge in visited_small: continue
            if edge == 'end': paths += 1; continue

            new_path = path.copy(); new_path.append(edge)
            new_visited_small = visited_small.copy()
            if edge.islower(): new_visited_small.append(edge)
            search_states.append((edge, new_path, new_visited_small))
        search_states = search_states[1:]
    return paths

def puzzle2(nodes, edges):
    paths = 0
    search_states = [('start', [], None)]
    while len(search_states) > 0:
        curr_node, visited_small, double_small = search_states[0]
        for edge in edges[curr_node]:
            if (edge in visited_small) and (double_small is not None): continue
            if edge == 'start': continue
            if edge == 'end': paths += 1; continue

            new_visited_small = visited_small.copy()
            new_double_small = double_small
            if edge.islower(): 
                if edge in visited_small: new_double_small = edge
                else: new_visited_small.append(edge)
            search_states.append((edge, new_visited_small, new_double_small))
        search_states = search_states[1:]
    return paths


if __name__ == "__main__":
    with open('data/day12.data', 'r') as data:
        data_rows = [line.strip() for line in data.readlines()]
    
    nodes = set([])
    edges = {}
    for row in data_rows:
        left, right = row.split('-')
        nodes.update([left, right])
        
        if left not in edges: edges[left] = []
        edges[left].append(right)
        if right not in edges: edges[right] = []
        edges[right].append(left)

    print(puzzle1(nodes, edges))
    print(puzzle2(nodes, edges))