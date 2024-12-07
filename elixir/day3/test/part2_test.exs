# Test module
defmodule Part2Test do
  use ExUnit.Case

  test "part2" do
    input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
    assert Part2.part2(input) == 48
  end
end
