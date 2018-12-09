with open('input.txt') as input_file:
    input_chars = list(input_file.read().strip())

def react(input_chars):
    stack = []
    for char in new_input:
        if len(stack) > 0 and char != stack[-1] and char.upper() == stack[-1].upper():
            stack.pop()
            continue
        stack.append(char)
    return len(stack)

lowest = 100000000
for char_to_remove in range(ord('a'), ord('z') + 1):
    char_to_remove = chr(char_to_remove)
    new_input = [c for c in input_chars[:] if c != char_to_remove and c != char_to_remove.upper()]
    length = react(new_input)
    if length < lowest:
        lowest = length

print(lowest)