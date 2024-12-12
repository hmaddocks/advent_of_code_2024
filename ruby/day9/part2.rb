# frozen_string_literal: true

def parse_input(input)
  input.each_char.map(&:to_i)
end

def initialize_disk(numbers)
  disk = []
  numbers.each_slice(2).with_index do |(length, spaces), file_id|
    disk.concat([file_id] * length)
    disk.concat([nil] * spaces.to_i)
  end
  disk
end

def find_files(disk)
  files = []
  current = { id: nil, start: 0 }

  disk.each_with_index do |block, idx|
    next unless current[:id] != block

    files << [current[:id], current[:start], idx - current[:start]] if current[:id]
    current = { id: block, start: idx }
  end
  files << [current[:id], current[:start], disk.length - current[:start]] if current[:id]

  files.sort_by! { |id, _, _| -id }
end

def find_best_position(disk, start_pos, size)
  window_sum = disk[0...size].count(nil)
  current_sum = window_sum

  (1..start_pos - size).each do |i|
    current_sum -= 1 if disk[i - 1].nil?
    current_sum += 1 if disk[i + size - 1].nil?

    return i if current_sum == size
  end
  nil
end

def move_file(disk, file_id, start_pos, best_start, size)
  size.times { |i| disk[start_pos + i] = nil }
  size.times { |i| disk[best_start + i] = file_id }
end

def calculate_score(disk)
  disk.map.with_index { |block, idx| block ? idx * block : 0 }.sum
end

def compact_disk(numbers)
  disk = initialize_disk(numbers)
  files = find_files(disk)

  files.each do |file_id, start_pos, size|
    best_start = find_best_position(disk, start_pos, size)
    next unless best_start

    move_file(disk, file_id, start_pos, best_start, size)
  end

  calculate_score(disk)
end

def part2(input)
  numbers = parse_input(input)
  compact_disk(numbers)
end

if __FILE__ == $PROGRAM_NAME
  input = File.read('input.txt')
  puts "Result: #{part2(input)}"
end
