defmodule Shui.Box do
  alias Shui.Presenter
  alias Shui.Viewer
  alias Shui.Color
  alias Shui.Position
  alias Shui.Messages
  alias Shui.Presentation

  def create(on_present) do
    {:box, on_present}
  end

  def present({:box, on_present}, viewer, director) do
    Presenter.start(on_present, viewer, director)
  end

  def split_r(l_box, r_box) do
    split_r(l_box, r_box, 0.5)
  end
  def split_r(l_box, r_box, degree) do
    create fn viewer, director ->
      {l_position, r_position} = Viewer.position(viewer) |> Position.split_horizontal(1-degree)
      {l_viewer, r_viewer} = {Viewer.reposition(viewer, l_position), Viewer.reposition(viewer, r_position)}
      {l_presentation, r_presentation} = {
        l_box |> present(l_viewer, director),
        r_box |> present(r_viewer, director)
      }
      receive do
        :dismiss ->
          Presentation.dismiss(l_presentation)
          Presentation.dismiss(r_presentation)
      end
    end
  end

  def color_box(red, green, blue) do
    create fn(viewer, director) ->
      position = Viewer.position(viewer)
      id = Messages.patch_id()
      Viewer.patch(viewer, Color.color(red, green, blue), position, id)
      receive do
        :dismiss ->
          Viewer.unpatch(viewer, id)
      end
    end
  end
  def color_box(), do: color_box(:random.uniform(), :random.uniform(), :random.uniform())

end
