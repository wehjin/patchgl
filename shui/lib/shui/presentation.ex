defmodule Shui.Presentation do

  def dismiss({pid} = presentation), do: send(pid, :dismiss)

end