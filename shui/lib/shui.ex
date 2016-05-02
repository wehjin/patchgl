defmodule Shui do

  def go() do
    color = Shui.Color.red()
    position = Shui.Position.full()
    viewer = Shui.Viewer.start()
    Shui.Viewer.patch(viewer, color, position)
    viewer
  end
end
