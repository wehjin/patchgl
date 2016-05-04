defmodule Shui.Viewer do
  alias Shui.Position

  def start() do
    pid = spawn(fn() -> open_window() end)
    {:root, pid, Position.full()}
  end

  def stop({:root, pid, _position}) do
    message = Shui.Messages.close_encoded()
    send(pid, {:command, message})
  end

  def patch({:root, pid, _position}, color, position, id) do
    %{:red=>red, :green=>green, :blue=>blue} = color
    %{:left=>left, :bottom=>bottom, :right=>right, :top=>top} = position
    color_message = Shui.Messages.color(red, green, blue)
    position_message = Shui.Messages.position(left, bottom, right, top)
    message = Shui.Messages.begin_patch_encoded(color_message, position_message, id)
    send(pid, {:command, message})
  end
  def patch({:child, parent, _position}, color, position, id) do
    patch(parent, color, position, id)
  end

  def unpatch({:root, pid, _position}, id) do
    send(pid, {:command, Shui.Messages.end_patch_encoded(id)})
  end
  def unpatch({:child, parent, _position}, id) do
    unpatch(parent, id)
  end

  def position({_, _, position}) do
    position
  end

  def reposition(viewer, position) do
    create_child(viewer, position)
  end

  defp create_child(parent, position) do
    {:child, parent, position}
  end

  defp open_window() do
    port = Port.open({:spawn, "patchgl"}, [:stream, :binary, :exit_status, :hide, :use_stdio, :stderr_to_stdout])
    loop_window(port)
  end

  defp loop_window(port) do
    receive do
      {:command, data} ->
          send(port, {self, {:command, data}})
          loop_window(port)
      {^port, {:data, data}} ->
          IO.puts(data)
          loop_window(port)
      :close ->
          IO.puts("Closing port")
          Port.close(port)
      {^port, {:exit_status, status}} ->
          status
      {^port, :closed} ->
          :closed
    end
  end
end