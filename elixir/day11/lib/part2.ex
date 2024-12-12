defmodule Part2 do
  def parse_input(input) do
    input
    |> String.split()
    |> Enum.map(&String.to_integer/1)
  end

  def blink(stones, blinks) do
    cache = :ets.new(:blink_cache, [:set])

    result =
      Enum.reduce(stones, 0, fn x, acc ->
        acc + blink_recursive(x, blinks, cache)
      end)

    :ets.delete(cache)
    result
  end

  defp blink_recursive(_, 0, _cache), do: 1

  defp blink_recursive(value, remaining_blinks, cache) do
    cache_key = {value, remaining_blinks}

    case :ets.lookup(cache, cache_key) do
      [{^cache_key, result}] ->
        result

      [] ->
        result = calculate_blink(value, remaining_blinks, cache)
        :ets.insert(cache, {cache_key, result})
        result
    end
  end

  defp calculate_blink(0, remaining_blinks, cache) do
    blink_recursive(1, remaining_blinks - 1, cache)
  end

  defp calculate_blink(value, remaining_blinks, cache) do
    value_str = Integer.to_string(value)

    if rem(String.length(value_str), 2) == 0 do
      mid = div(String.length(value_str), 2)
      {first_half, second_half} = String.split_at(value_str, mid)
      first_num = String.to_integer(first_half)
      second_num = String.to_integer(second_half)

      blink_recursive(first_num, remaining_blinks - 1, cache) +
        blink_recursive(second_num, remaining_blinks - 1, cache)
    else
      blink_recursive(2024 * value, remaining_blinks - 1, cache)
    end
  end

  def part2(input, blinks) do
    input
    |> parse_input()
    |> blink(blinks)
  end
end

File.read!("input.txt")
|> Part2.part2(75)
|> IO.inspect(label: "part2")
