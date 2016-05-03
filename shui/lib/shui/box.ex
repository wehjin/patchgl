defmodule Shui.Box do
  alias Shui.Presenter, as: Presenter
  alias Shui.Viewer, as: Viewer
  alias Shui.Color, as: Color

  def box(on_present) do
    {on_present}
  end

  def present({on_present} = box, viewer, director) do
    Presenter.start(on_present, viewer, director)
  end

  def color_box(red, green, blue) do
    color = Color.color(red, green, blue)
    box(fn(viewer, director) ->
      position = Viewer.position(viewer)
      id = Shui.Messages.patch_id()
      Viewer.patch(viewer, color, position, id)
      receive do
        :dismiss ->
          Viewer.unpatch(viewer, id)
      end
    end)
  end
  def color_box(), do: color_box(:random.uniform(), :random.uniform(), :random.uniform())

end
