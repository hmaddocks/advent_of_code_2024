defmodule Part2Test do
  use ExUnit.Case

  test "part2" do
    input = """
    MMMSXXMASM
    MSAMXMSMSA
    AMXSXMAAMM
    MSAMASMSMX
    XMASAMXAMM
    XXAMMXXAMA
    SMSMSASXSS
    SAXAMASAAA
    MAMMMXMMMM
    MXMXAXMASX
    """

    assert Part2.part2(input) == 9
  end
end
