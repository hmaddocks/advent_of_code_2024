defmodule Part1Test do
  use ExUnit.Case
  doctest Part1

  test "parse_line handles multiple spaces" do
    assert Part1.parse_line("3   4") == {3, 4}
    assert Part1.parse_line("42    17") == {42, 17}
  end

  test "part1 calculates correct result" do
    input = """
    3   4
    4   3
    2   5
    1   3
    3   9
    3   3
    """

    assert Part1.part1(input) == 11
  end
end
