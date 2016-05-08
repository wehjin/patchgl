defmodule Shui.Position do

  def position(left, bottom, right, top) do
    position(left, bottom, right, top, 0)
  end
  def position(left, bottom, right, top, near) do
    %{:left => left, :bottom=>bottom, :right =>right, :top=>top, :near=>near}
  end

  def full() do
    position(-1.0, -1.0, 1.0, 1.0, 0)
  end

  def split_horizontal(position) do
    split_horizontal(position, 0.5)
  end
  def split_horizontal(%{:left => left, :bottom=>bottom, :right =>right, :top=>top, :near=>near}, degree) do
    split = left + (right - left) * degree
    {position(left, bottom, split, top, near), position(split, bottom, right, top, near)}
  end

  def add_distance(position, addition) do
    %{position | :near => position[:near] + addition}
  end
end