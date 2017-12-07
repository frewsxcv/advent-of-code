import igraph

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
print("part 1 solution: {}".format(graph.vs[root_index]))
