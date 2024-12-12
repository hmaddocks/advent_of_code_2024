# frozen_string_literal: true

def parse_input(input)
  input.split.map(&:to_i)
end

def blink_recursive(value, remaining_blinks, cache)
  return 1 if remaining_blinks.zero?

  cache_key = [value, remaining_blinks]
  return cache[cache_key] if cache.key?(cache_key)

  result = if value.zero?
             blink_recursive(1, remaining_blinks - 1, cache)
           else
             value_str = value.to_s
             if value_str.length.even?
               mid = value_str.length / 2
               first_half = value_str[0...mid].to_i
               second_half = value_str[mid..].to_i
               blink_recursive(first_half, remaining_blinks - 1, cache) +
                 blink_recursive(second_half, remaining_blinks - 1, cache)
             else
               blink_recursive(2024 * value, remaining_blinks - 1, cache)
             end
           end

  cache[cache_key] = result
  result
end

def blink(stones, blinks)
  cache = {}
  stones.reduce(0) { |acc, x| acc + blink_recursive(x, blinks, cache) }
end

def part2(input, blinks)
  numbers = parse_input(input)
  blink(numbers, blinks)
end

if __FILE__ == $PROGRAM_NAME
  input = File.read('input.txt')
  puts part2(input, 75)
end
