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
      {:patch, _id, patch} ->
        flunk("too many patches")
      other -> IO.inspect(other)
      after 250 -> :expected
    end
  end
end
