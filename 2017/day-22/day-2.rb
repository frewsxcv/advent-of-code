DIRS = [
    DIR_N = :north,
    DIR_E = :east,
    DIR_S = :south,
    DIR_W = :west,
]

# mutates `pos`
def move_forward!(pos, dir)
    if dir == DIR_N
        pos[1] -= 1
    elsif dir == DIR_S
        pos[1] += 1
    elsif dir == DIR_E
        pos[0] += 1
    elsif dir == DIR_W
        pos[0] -= 1
    else
        raise
    end
end

# mutates `grid`
def flip_cell!(grid, pos)
    grid[pos[1]][pos[0]] =
        case grid[pos[1]][pos[0]]
        when "#"
            "F"
        when "."
            "W"
        when "W"
            "#"
        when "F"
            "."
        else
            raise
        end
end

def determine_new_dir(prev_dir, grid, pos)
    case grid[pos[1]][pos[0]]
    when "#"
        # turn right
        DIRS.cycle(2).to_a[DIRS.index(prev_dir) + 1]
    when "W"
        # no turn
        prev_dir
    when "F"
        DIRS.cycle(2).to_a[DIRS.index(prev_dir) + 2]
    when "."
        # turn left
        DIRS.reverse.cycle(2).to_a[DIRS.reverse.index(prev_dir) + 1]
    else
        raise
    end
end

# mutates `grid`, `pos`
def do_work!(grid, pos, dir)
    new_dir = determine_new_dir(dir, grid, pos)
    flip_cell!(grid, pos)
    move_forward!(pos, new_dir)
    [new_dir, grid[pos[1]][pos[0]] == "W"]
end

# mutates `grid`
def expand_grid!(grid, padding)
    new_size = grid[0].length + (padding * 2)
    grid.each do |row|
        (0...padding).each do
            row << '.'
            row.prepend('.')
        end
    end
    (0...padding).each do
        grid.insert(0, '.' * new_size)
        grid.push('.' * new_size)
    end
end

input_file = File.open('input').read.strip
grid = input_file.split("\n")
expand_grid!(grid, 2000)

curr_dir = DIR_N
curr_pos = Array.new(2, grid.length / 2)
count = 0

(0...10000000).each do
    curr_dir, became_infected = do_work!(grid, curr_pos, curr_dir)
    if became_infected
        count += 1
    end
end

puts count
