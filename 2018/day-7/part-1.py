import networkx

with open('input.txt') as input_file:
    input_lines = input_file.readlines()

graph = networkx.DiGraph()

for line in input_lines:
    from_ = line[len('Step ')]
    to = line[-len(' can begin.') - 2]
    graph.add_edge(from_, to)

while len(graph.nodes) > 0:
    next_nodes = (n for n in graph if len(list(graph.predecessors(n))) == 0)
    next_node = sorted(next_nodes)[0] # alphabetic sort
    graph.remove_node(next_node)
    print(next_node, end='')
