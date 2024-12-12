defmodule Part2Test do
  use ExUnit.Case

  test "part2, 1" do
    input = "0 1 10 99 999"

    assert Part2.part2(input, 1) == 7
  end

  test "part2, 2" do
    input = "125 17"

    assert Part2.part2(input, 6) == 22
  end

  test "part2, 3" do
    input = "8435 234 928434 14 0 7 92446 8992692"

    assert Part2.part2(input, 25) == 182_081
  end
end
