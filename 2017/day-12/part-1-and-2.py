import igraph

with open('input') as file_:
    input_ = file_.read()

graph = igraph.Graph()

input_lines = input_.strip().splitlines()
for line in input_lines:
    node1, _ = line.split(' <-> ')
    graph.add_vertex(node1)

for line in input_.strip().splitlines():
    node1, rest = line.split(' <-> ')
    for node2 in rest.split(', '):
        graph.add_edge(node1, node2)

print("part 1", len(graph.clusters()[0]))
print("part 2", len(graph.clusters()))
