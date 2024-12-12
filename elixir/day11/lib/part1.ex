defmodule Part1 do
  def parse_input(input) do
    input
    |> String.split()
    |> Enum.map(&String.to_integer/1)
  end

  def process_stone(stone, 0), do: [stone]

  def process_stone(0, blinks), do: process_stone(1, blinks - 1)

  def process_stone(stone, blinks) do
    stone = abs(stone)
    num_digits = trunc(:math.log10(stone)) + 1

    if rem(num_digits, 2) == 0 do
      power = trunc(:math.pow(10, div(num_digits, 2)))
      first_half = div(stone, power)
      second_half = rem(stone, power)

      [first_half, second_half]
      |> Enum.flat_map(&process_stone(&1, blinks - 1))
    else
      process_stone(stone * 2024, blinks - 1)
    end
  end

  def blink(stones, blinks) do
    stones
    |> Enum.flat_map(&process_stone(&1, blinks))
    |> length()
  end

  def part1(input, blinks) do
    input
    |> parse_input()
    |> blink(blinks)
  end
end

File.read!("input.txt")
|> Part1.part1(25)
|> IO.inspect(label: "part1")
