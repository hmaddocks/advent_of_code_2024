# frozen_string_literal: true

def parse_input(input)
  left, right = input.split("\n")
                     .map { |line| line.split.map(&:to_i) }
                     .transpose
  [left.sort, right.sort]
end

def part2(input)
  left_list, right_list = parse_input(input)

  right_map = right_list.tally

  left_list.reduce(0) do |acc, left|
    count = right_map[left]
    count.nil? ? acc : acc + (left * count)
  end
end

if __FILE__ == $PROGRAM_NAME
  input = File.read('input.txt')
  p part2(input)
end
