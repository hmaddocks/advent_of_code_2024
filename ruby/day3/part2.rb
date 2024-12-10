# frozen_string_literal: true

def part2(input)
  input.scan(/(do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\))/).reduce([true, 0]) do |(enabled, sum), command|
    case command[0]
    when 'do()'
      [true, sum]
    when "don't()"
      [false, sum]
    else
      [enabled, sum + (enabled ? command[1].to_i * command[2].to_i : 0)]
    end
  end.last
end

if __FILE__ == $PROGRAM_NAME
  input = File.read('input.txt')
  puts "Result: #{part2(input)}"
end
