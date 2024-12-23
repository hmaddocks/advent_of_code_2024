# test/day7/part1_test.exs
defmodule Part2Test do
  use ExUnit.Case

  test "two numbers OK" do
    input = "190: 10 19"
    assert Part2.part2(input) == 190
  end

  test "two numbers not OK" do
    input = "83: 17 5"
    assert Part2.part2(input) == 0
  end

  test "three numbers OK" do
    input = "3267: 81 40 27"
    assert Part2.part2(input) == 3267
  end

  test "three numbers not OK" do
    input = "161011: 16 10 13"
    assert Part2.part2(input) == 0
  end

  test "four numbers OK" do
    input = "292: 11 6 16 20"
    assert Part2.part2(input) == 292
  end

  test "four numbers not OK" do
    input = "7290: 6 8 6 15"
    assert Part2.part2(input) == 0
  end

  test "two numbers concat OK" do
    input = "156: 15 6"
    assert Part2.part2(input) == 156
  end

  test "three numbers concat OK" do
    input = "192: 17 8 14"
    assert Part2.part2(input) == 192
  end

  test "four numbers concat OK" do
    input = "7290: 6 8 6 15"
    assert Part2.part2(input) == 7290
  end

  test "solve puzzle" do
    input = """
    190: 10 19
    3267: 81 40 27
    83: 17 5
    156: 15 6
    7290: 6 8 6 15
    161011: 16 10 13
    192: 17 8 14
    21037: 9 7 18 13
    292: 11 6 16 20
    """

    assert Part2.part2(input) == 11387
  end
end
