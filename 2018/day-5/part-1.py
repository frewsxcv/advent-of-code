with open('input.txt') as input_file:
    input_chars = list(input_file.read().strip())

while True:
    for i, (char_a, char_b) in enumerate(zip(input_chars, input_chars[1:])):
        if char_a != char_b and char_a.upper() == char_b.upper():
            input_chars.pop(i) # remove char_a
            input_chars.pop(i) # remove char_b
            break
    else:
        print(len(input_chars))
        exit()