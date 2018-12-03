import collections

Claim = collections.namedtuple('Claim', ['id_', 'left_margin', 'top_margin', 'width', 'height'])

with open('input.txt') as f:
    input_contents = f.read()

claims = []
for line in input_contents.splitlines():
    line = line.strip().strip('#')
    id_, line = line.split(' @ ')
    left_margin, line = line.split(',')
    top_margin, line = line.split(': ')
    width, height = line.split('x')
    claims.append(
        Claim(
            id_=int(id_),
            left_margin=int(left_margin),
            top_margin=int(top_margin),
            width=int(width),
            height=int(height),
        )
    )

grid = collections.defaultdict(int)

for claim in claims:
    for i in range(claim.left_margin, claim.left_margin + claim.width):
        for j in range(claim.top_margin, claim.top_margin + claim.height):
            grid[(i, j)] += 1

print(sum(1 for v in grid.values() if v >= 2))