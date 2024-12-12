defmodule Part1 do
  def parse_input(input) do
    input
    |> String.split()
    |> Enum.map(&String.to_integer/1)
  end

  def process_stone(stone, 0), do: [stone]

  def process_stone(0, blinks), do: process_stone(1, blinks - 1)

  def process_stone(stone, blinks) do
    digits = Integer.to_string(abs(stone))
    process_stone_digits(stone, digits, blinks)
  end

  defp process_stone_digits(stone, digits, blinks) do
    case rem(String.length(digits), 2) do
      0 ->
        mid = div(String.length(digits), 2)
        {first_half, second_half} = String.split_at(digits, mid)

        [first_half, second_half]
        |> Enum.map(&String.to_integer/1)
        |> Enum.flat_map(&process_stone(&1, blinks - 1))

      _ ->
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
