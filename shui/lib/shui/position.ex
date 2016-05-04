defmodule Shui.Position do

  def position(left, bottom, right, top) do
    %{:left => left, :bottom=>bottom, :right =>right, :top=>top}
  end

  def full() do
    position(-1.0, -1.0, 1.0, 1.0)
  end

  def split_horizontal(position) do
    split_horizontal(position, 0.5)
  end
  def split_horizontal(%{:left => left, :bottom=>bottom, :right =>right, :top=>top}, degree) do
    split = left + (right - left) * degree
    {position(left, bottom, split, top), position(split, bottom, right, top)}
  end
end