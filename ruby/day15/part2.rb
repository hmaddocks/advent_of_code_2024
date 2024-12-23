# frozen_string_literal: true

def constuct_map(input)
  input.split("\n").map(&:chars).map do |char|
    case char
    when '.' then '..'
    when '#' then '##'
    when '@' then '@.'
    when 'O' then '[]'
    end
  end
end

def parse_moves(moves)
  moves.strip.delete("\n").chars.map do |char|
    case char
    when '<' then [-1, 0]
    when '>' then [1, 0]
    when '^' then [0, -1]
    when 'v' then [0, 1]
    end
  end
end

def find_start(map)
  map.each_with_index do |row, y|
    row.each_with_index do |cell, x|
      return [x, y] if cell == '@'
    end
  end
  nil
end

def move_robot(map, start, moves)
  moves.each_with_object(map) do |(dx, dy), current_map|
    current_pos = start.dup
    new_pos = [current_pos[0] + dx, current_pos[1] + dy]

    next current_map if current_map[new_pos[1]][new_pos[0]] == '#'

    # Find and move objects in the path
    objects_to_move = []
    check_pos = new_pos.dup

    # Collect objects in the chain
    while current_map[check_pos[1]][check_pos[0]] == 'O'
      objects_to_move << check_pos
      check_pos = [check_pos[0] + dx, check_pos[1] + dy]
    end

    # Skip if the end of the chain is blocked
    next current_map if current_map[check_pos[1]][check_pos[0]] == '#'

    # Move objects
    objects_to_move.reverse_each do |obj_pos|
      current_map[obj_pos[1]][obj_pos[0]] = '.'
      new_obj_pos = [obj_pos[0] + dx, obj_pos[1] + dy]
      current_map[new_obj_pos[1]][new_obj_pos[0]] = 'O'
    end

    # Update start position for the next iteration
    start[0] += dx
    start[1] += dy
  end
end

def find_box_positions(map)
  map.each_with_index.flat_map do |row, y|
    row.each_with_index.filter_map do |cell, x|
      [x, y] if cell == '['
    end
  end
end

def draw_the_map(map, robot_pos = nil, move = nil)
  move_dir = case move
             when [1, 0] then '>'
             when [-1, 0] then '<'
             when [0, 1] then 'v'
             when [0, -1] then '^'
             end
  puts "Move: #{move_dir}" if move_dir
  puts "Posiiton: #{robot_pos}" if robot_pos
  map_copy = map.map(&:dup)
  map_copy[robot_pos[1]][robot_pos[0]] = '@' if robot_pos
  map_copy.each do |row|
    puts row.join
  end
  puts
end

def part1(input)
  map_part, move_part = input.split("\n\n")
  map = parse_map(map_part)
  moves = parse_moves(move_part)
  start = find_start(map)

  move_robot(map, start, moves)
    # .tap { |new_map| draw_the_map(new_map, start, moves.first) }
    .then { |map| find_box_positions(map) }
    # .tap { |box_positions| p box_positions }
    .then do |box_positions|
    box_positions.reduce(0) do |acc, (x, y)|
      acc + (y * 100) + x
    end
  end
end

if __FILE__ == $PROGRAM_NAME
  input = File.read('input.txt')
  puts part1(input)
end
