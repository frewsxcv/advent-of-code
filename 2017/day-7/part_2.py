from itertools import groupby
import igraph

def total_weight(vertex):
    weight = vertex['weight']
    for child in vertex.neighbors(mode=igraph.OUT):
        weight += total_weight(child)
    return weight

def find_imbalance(vertex, diff):
    neighbors = vertex.neighbors(mode=igraph.OUT)
    scores = list(map(lambda v: total_weight(v), neighbors))
    imbalance_score = [n for (n, i) in groupby(sorted(scores)) if len(list(i)) == 1]
    balance_score = [n for (n, i) in groupby(sorted(scores)) if len(list(i)) != 1]
    if not imbalance_score:
        print(vertex)
        return vertex['weight'] - diff
    return find_imbalance(
        neighbors[scores.index(imbalance_score[0])],
        imbalance_score[0] - balance_score[0],
    )

with open('input') as file_:
    input_ = file_.read().strip()

programs = []

for line in input_.splitlines():
    tokens = line.replace(',', '').replace(')', '').replace('(', '').split()
    if len(tokens) > 2:
        tokens.pop(2)  # throw away '->'
    name, weight, *above = tokens
    programs.append((name, int(weight), above))

graph = igraph.Graph(directed=True)

for (name, weight, _) in programs:
    graph.add_vertex(name=name, weight=weight)

for (name, _, above) in programs:
    for child_name in above:
        graph.add_edge(name, child_name)

root_index = graph.indegree().index(0)
root = graph.vs[root_index]

print(find_imbalance(root, 0))
