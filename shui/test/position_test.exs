defmodule PositionTest do
  use ExUnit.Case
  doctest Shui.Position

  alias Shui.Position

  test "position" do
    position = Position.position(1,2,3,4)
    assert position[:left] == 1
    assert position[:bottom] == 2
    assert position[:right] == 3
    assert position[:top] == 4
    assert position[:near] == 0
  end

  test "split_horizontal" do
    {l, r} = Position.split_horizontal(Position.position(0, 0, 100, 100, 50), 0.5)
    assert l[:left] == 0 && r[:left] == 50
    assert l[:right] == 50 && r[:right] == 100
    assert l[:bottom] == 0 && r[:bottom] == 0
    assert l[:top] == 100 && r[:top] == 100
    assert l[:near] == 50 && r[:near] == 50
  end
end
