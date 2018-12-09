with open('input.txt') as input_file:
    numbers = list(map(int, input_file.read().strip().split(' ')))

def read_memory(numbers):
    num_children = numbers.pop(0)
    num_metadata = numbers.pop(0)
    sum_metadata = 0
    for _ in range(num_children):
        sum_metadata += read_memory(numbers)
    for _ in range(num_metadata):
        sum_metadata += numbers.pop(0)
    return sum_metadata

print(read_memory(numbers))
