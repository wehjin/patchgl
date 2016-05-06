defmodule ShuiViewerTest do
  use ExUnit.Case
  doctest Shui.Viewer

  alias Shui.Viewer
  alias Shui.Color
  alias Shui.Position

  test "recorder" do
    recorder = Viewer.start_recorder()
    color = Color.color(0.1,0.2,0.3)
    position = Position.position(1,2,3,4,5)
    Viewer.patch(recorder, color, position, :p1)
    Viewer.inspect(recorder, self)
    receive do
      {:patches, patches} ->
        Viewer.stop(recorder)
        assert patches == %{p1: %{color: color, position: position}}
    end
  end
end
