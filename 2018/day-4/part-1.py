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

# find sleepiest guard

curr_guard_id = None  # current guard id
prev_timestamp = None  # previous timestamp
time_asleep = collections.defaultdict(timedelta)  # guard id -> timedelta
for timestamp, command in records:
    if command.startswith("Guard"):
        curr_guard_id = int(command.lstrip("Guard #").rstrip(" begins shift"))
    elif command == "wakes up":
        time_asleep[curr_guard_id] += (timestamp - prev_timestamp)
    prev_timestamp = timestamp
sleepiest_guard_id = sorted(time_asleep.items(), key=lambda n: n[1])[-1][0]

# find sleepiest minute

sleep_histogram = collections.defaultdict(int)  # minute -> occurrences asleep
for timestamp, command in records:
    if command.startswith("Guard"):
        curr_guard_id = int(command.lstrip("Guard #").rstrip(" begins shift"))
    elif curr_guard_id == sleepiest_guard_id and command == "wakes up":
        for minute in range(prev_timestamp.minute, timestamp.minute):
            sleep_histogram[minute] += 1
    prev_timestamp = timestamp
sleepiest_minute = sorted(sleep_histogram.items(), key=lambda n: n[1])[-1][0]

print(sleepiest_guard_id * sleepiest_minute)
