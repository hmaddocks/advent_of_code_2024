defmodule Part2Test do
  use ExUnit.Case

  test "part2 calculates correct result" do
    input = """
    7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9
    """

    assert Part2.part2(input) == 4
  end
end
