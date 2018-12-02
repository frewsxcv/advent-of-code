import collections

with open('input.txt') as f:
    lines = f.read().splitlines()

num_twos = 0
num_threes = 0

for box_id in lines:
    counter = collections.Counter(box_id)
    if 2 in counter.values():
        num_twos += 1
    if 3 in counter.values():
        num_threes += 1

print(num_twos * num_threes)
