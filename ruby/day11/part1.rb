# frozen_string_literal: true

def parse_input(input)
  input.split.map(&:to_i)
end

def process_stone(stone, blinks)
  return [stone] if blinks.zero?

  case stone
  when 0 then process_stone(1, blinks - 1)
  else
    digits = stone.abs.to_s
    if digits.length.even?
      mid = digits.length / 2
      [
        process_stone(digits[0...mid].to_i, blinks - 1),
        process_stone(digits[mid..].to_i, blinks - 1)
      ].flatten
    else
      process_stone(stone * 2024, blinks - 1)
    end
  end
end

def blink(stones, blinks)
  stones.flat_map { |stone| process_stone(stone, blinks) }.size
end

def part1(input, blinks)
  numbers = parse_input(input)
  blink(numbers, blinks)
end

if __FILE__ == $PROGRAM_NAME
  input = File.read('input.txt')
  puts part1(input, 25)
end
