import itertools

with open('input.txt') as f:
    lines = f.read().splitlines()

for box_id_1, box_id_2 in itertools.product(lines, repeat=2):
    zipped = list(zip(box_id_1, box_id_2))
    if sum(x != y for x, y in zipped) == 1:
        print(''.join(x for x, y in zipped if x == y))
        exit()
