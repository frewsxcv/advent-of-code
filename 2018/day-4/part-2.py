from datetime import datetime, timedelta
import collections

with open("input.txt") as input_file:
    input_lines = input_file.readlines()

records = []  # [(timestamp, command), ...]
for line in input_lines:
    line = line.strip().lstrip("[")
    timestamp_string, command = line.split("] ")
    timestamp = datetime.strptime(timestamp_string, "%Y-%m-%d %H:%M")
    records.append((timestamp, command))
records.sort(key=lambda n: n[0])

# build up all histograms

sleep_histograms = collections.defaultdict(lambda: collections.defaultdict(int))
for timestamp, command in records:
    if command.startswith("Guard"):
        curr_guard_id = int(command.lstrip("Guard #").rstrip(" begins shift"))
    elif command == "wakes up":
        for minute in range(prev_timestamp.minute, timestamp.minute):
            sleep_histograms[curr_guard_id][minute] += 1
    prev_timestamp = timestamp

# find guard most frequently asleep on the same minute

highest_quantity = 0
highest_score = None  # guard id * minute
for (guard_id, histogram) in sleep_histograms.items():
    (minute, quantity) = sorted(histogram.items(), key=lambda n: n[1])[-1]
    if quantity > highest_quantity:
        highest_quantity = quantity
        highest_score = guard_id * minute

print(highest_score)
