defmodule ShuiBoxTest do
  use ExUnit.Case
  doctest Shui.Box
  alias Shui.Box
  alias Shui.Viewer
  alias Shui.Color
  alias Shui.Position

  test "color" do
    recorder = Viewer.start_proxy(self)
    Box.color(Color.color(0.1, 0.2, 0.3)) |> Box.present(recorder, :d)
    receive do
      {:patch, _id, patch} ->
        assert patch == %{color: Color.color(0.1, 0.2, 0.3), position: Position.full()}
      other -> IO.inspect(other)
      after 500 -> flunk("timeout")
    end
  end

  test "pad_r" do
    recorder = Viewer.start_proxy(self)
    Box.color(Color.color(0.1, 0.2, 0.3)) |> Box.pad_r(0.1) |> Box.present(recorder, :d)
    receive do
      {:patch, _id, patch} ->
        assert patch == %{color: Color.color(0.1, 0.2, 0.3), position: Position.position(-1,-1,0.8,1,0)}
      other -> IO.inspect(other)
      after 250 -> flunk("timeout")
    end
    receive do
      {:patch, _id, _patch} ->
        flunk("too many patches")
      other -> IO.inspect(other)
      after 250 -> :expected
    end
  end

  test "stack_n" do
    recorder = Viewer.start_proxy(self)
    distance = 0.1
    f_color = Color.color(0.1, 0.2, 0.3)
    n_color = Color.color(0.4,0.5,0.6)
    f_position = Position.full()
    n_position = f_position |> Position.add_distance(distance)

    Box.color(f_color) |> Box.stack_n(Box.color(n_color), distance) |> Box.present(recorder, :d)
    receive do
      {:patch, _id, patch} ->
        assert patch == %{color: f_color, position: f_position}
      other -> IO.inspect(other)
      after 250 -> flunk("timeout")
    end
    receive do
      {:patch, _id, patch} ->
        assert patch == %{color: n_color, position: n_position}
      other -> IO.inspect(other)
      after 250 -> flunk("timeout")
    end
  end

end
