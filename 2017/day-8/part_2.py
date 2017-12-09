from collections import defaultdict

# input_ = """
# b inc 5 if a > 1
# a inc 1 if b < 5
# c dec -10 if a >= 1
# c inc -20 if c == 10
# """

with open('input') as file_:
    input_ = file_.read()

regs = defaultdict(int)

def is_cond_true(cond_reg_value, cond_cmp, cond_amount):
    if cond_cmp == '>':
        return cond_reg_value > int(cond_amount)
    if cond_cmp == '<':
        return cond_reg_value < int(cond_amount)
    if cond_cmp == '>=':
        return cond_reg_value >= int(cond_amount)
    if cond_cmp == '<=':
        return cond_reg_value <= int(cond_amount)
    if cond_cmp == '==':
        return cond_reg_value == int(cond_amount)
    if cond_cmp == '!=':
        return cond_reg_value != int(cond_amount)

highest = 0

for line in input_.strip().splitlines():
    reg, op, amount, _, cond_reg, cond_cmp, cond_amount = line.split()

    if is_cond_true(regs[cond_reg], cond_cmp, cond_amount):
        regs[reg] += int(amount) * (1 if op == 'inc' else -1)
        highest = max(highest, regs[reg])

print(highest)
