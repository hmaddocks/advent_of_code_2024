defmodule Part2Test do
  use ExUnit.Case

  test "part1 calculates correct result" do
    input = """
    3   4
    4   3
    2   5
    1   3
    3   9
    3   3
    """

    assert Part2.part2(input) == 31
  end
end
