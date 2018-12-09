with open('input.txt') as input_file:
    numbers = list(map(int, input_file.read().strip().split(' ')))

def read_memory(numbers):
    num_children = numbers.pop(0)
    num_metadata = numbers.pop(0)

    child_sums = [read_memory(numbers) for _ in range(num_children)]

    if num_children == 0:
        return sum(numbers.pop(0) for _ in range(num_metadata))
    else:
        sum_metadata = 0
        for _ in range(num_metadata):
            metadata_value = numbers.pop(0)
            if metadata_value - 1 < num_children:
                sum_metadata += child_sums[metadata_value - 1]
        return sum_metadata

print(read_memory(numbers))
