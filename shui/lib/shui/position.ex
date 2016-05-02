defmodule Shui.Position do

  def position(left, bottom, right, top) do
    %{:left => left, :bottom=>bottom, :right =>right, :top=>top}
  end

  def full() do
    position(-1.0, -1.0, 1.0, 1.0)
  end
end