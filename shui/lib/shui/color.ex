defmodule Shui.Color do

  def color(red, green, blue) do
    %{:red=>red, :green=>green, :blue=>blue}
  end

  def black() do
    color(0.0, 0.0, 0.0)
  end

  def white() do
    color(0.0, 0.0, 0.0)
  end

  def red() do
    color(1.0, 0.0, 0.0)
  end

  def green() do
    color(0.0, 1.0, 0.0)
  end

  def blue() do
    color(0.0, 0.0, 1.0)
  end

end