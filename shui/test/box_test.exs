defmodule ShuiBoxTest do
  use ExUnit.Case
  doctest Shui.Box
  alias Shui.Box
  alias Shui.Viewer
  alias Shui.Color
  alias Shui.Position

  test "color_box" do
    recorder = Viewer.start_proxy(self)
    box = Box.color_box(0.1, 0.2, 0.3)
    Box.present(box, recorder, :d)
    receive do
      {:patch, _id, patch} ->
        assert patch == %{color: Color.color(0.1, 0.2, 0.3), position: Position.full()}
      other -> IO.inspect(other)
      after 500 -> flunk("timeout")
    end
  end
end
