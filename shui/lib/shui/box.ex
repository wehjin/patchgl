defmodule Shui.Box do
  alias Shui.Presenter, as: Presenter

  def box(on_present) do
    {on_present}
  end

  def present({on_present} = box, viewer, director) do
    presenter = Presenter.start(viewer, director)
    on_present.(presenter)
    presenter
  end

  def color_box(red, green, blue) do
    on_present = fn (presenter) -> 1 end
    box(on_present)
  end
end
