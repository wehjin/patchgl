defmodule Shui.Presenter do
  @moduledoc false

  def start(on_present, viewer, director) do
    pid = spawn(fn() -> on_present.(viewer, director) end)
    {pid}
  end

  def dismiss({pid} = presenter), do: Shui.Presentation.dismiss({pid})

  def is_dismissed(presenter) do
    1
  end
end