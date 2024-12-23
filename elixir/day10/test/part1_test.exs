defmodule Part1Test do
  use ExUnit.Case

  test "part1" do
    input = """
    89010123
    78121874
    87430965
    96549874
    45678903
    32019012
    01329801
    10456732
    """

    assert Part1.part1(input) == 36
  end
end
