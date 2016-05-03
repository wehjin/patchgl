defmodule Shui.Presenter do
  @moduledoc false

  def start(on_present, viewer, director) do
    pid = spawn(fn() ->
      :random.seed(:erlang.phash2([node()]),
                  :erlang.monotonic_time(),
                  :erlang.unique_integer())
       on_present.(viewer, director)
    end)
    {pid}
  end

  def dismiss({pid} = presenter), do: Shui.Presentation.dismiss({pid})

  def is_dismissed(presenter) do
    :not_implemented
  end
end