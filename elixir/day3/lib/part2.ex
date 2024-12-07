defmodule Part2 do
  def extract_and_sum_multiplications(input) do
    # First find all instructions in order
    instructions =
      Regex.scan(~r/(?:do\(\)|don't\(\)|mul\(\d+,\d+\))/, input)
      |> List.flatten()

    # Process each instruction in order
    {_enabled, sum} =
      Enum.reduce(instructions, {true, 0}, fn
        "do()", {_enabled, sum} ->
          {true, sum}

        "don't()", {_enabled, sum} ->
          {false, sum}

        "mul(" <> rest, {enabled, sum} ->
          if enabled do
            case Regex.run(~r/(\d+),(\d+)\)/, rest, capture: :all_but_first) do
              [x, y] -> {enabled, sum + String.to_integer(x) * String.to_integer(y)}
              _ -> {enabled, sum}
            end
          else
            {enabled, sum}
          end

        _, acc ->
          acc
      end)

    sum
  end

  def part2(input) do
    extract_and_sum_multiplications(input)
  end
end

File.read!(~c"input.txt")
|> Part2.part2()
|> IO.inspect(label: "part2")
