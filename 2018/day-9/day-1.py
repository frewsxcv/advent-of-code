import collections

# 430 players; last marble is worth 71588 points
num_players = 430
last_marble = 71_588

circle = [0]
curr_marble_idx = 1
curr_player = 0
scores = collections.defaultdict(int)

for next_marble_value in range(1, last_marble + 1):
    if next_marble_value % 23 == 0:
        scores[curr_player] += next_marble_value
        curr_marble_idx = (curr_marble_idx - 7) % len(circle)
        scores[curr_player] += circle.pop(curr_marble_idx)
    else:
        next_marble_idx = (curr_marble_idx + 1) % len(circle) + 1
        circle.insert(next_marble_idx, next_marble_value)
        curr_marble_idx = next_marble_idx
    curr_player = (curr_player + 1) % num_players

print(max(scores.values()))