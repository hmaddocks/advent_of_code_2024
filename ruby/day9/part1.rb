# frozen_string_literal: true

def parse_input(input)
  input.chars.map(&:to_i)
end

def initialize_disk(sequence)
  disk = []
  sequence.each_slice(2).with_index do |(count, spaces), index|
    disk.concat([index] * count)
    disk.concat([nil] * spaces) if spaces
  end
  disk
end

def find_empty_slots(disk)
  disk.each_index.select { |i| disk[i].nil? }.to_set
end

def move_files_left(disk, empty_slots)
  (disk.size - 1).downto(0) do |current_pos|
    next if disk[current_pos].nil?

    target_pos = empty_slots.find { |pos| pos < current_pos }
    next unless target_pos

    empty_slots.delete(target_pos)
    empty_slots.add(current_pos)
    disk[target_pos] = disk[current_pos]
    disk[current_pos] = nil
  end
  disk
end

def calculate_score(disk)
  disk.map.with_index { |file_id, index| file_id ? index * file_id : 0 }.sum
end

def optimize_disk_space(sequence)
  disk = initialize_disk(sequence)
  empty_slots = find_empty_slots(disk)
  optimized_disk = move_files_left(disk, empty_slots)
  calculate_score(optimized_disk)
end

def part1(input)
  optimize_disk_space(parse_input(input))
end

if __FILE__ == $PROGRAM_NAME
  input = File.read('input.txt')
  puts part1(input)
end
