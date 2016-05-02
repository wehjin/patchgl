defmodule Shui.Viewer do

  def start() do
    pid = spawn(fn() -> open_window() end)
    position = Shui.Position.full()
    {pid, :root, position}
  end

  def patch({pid, :root, _} = viewer, color, position, id) do
    %{:red=>red, :green=>green, :blue=>blue} = color
    %{:left=>left, :bottom=>bottom, :right=>right, :top=>top} = position
    color_message = Shui.Messages.color(red, green, blue)
    position_message = Shui.Messages.position(left, bottom, right, top)
    message = Shui.Messages.begin_patch_encoded(color_message, position_message, id)
    send(pid, {:command, message})
  end

  def unpatch({pid, :root, _} = viewer, id) do
    send(pid, {:command, Shui.Messages.end_patch_encoded(id)})
  end

  def position({_, _, position}) do
    position
  end

  defp open_window() do
    port = Port.open({:spawn, "patchgl"}, [:stream, :binary, :exit_status, :hide, :use_stdio, :stderr_to_stdout])
    handle_output(port)
  end

  defp handle_output(port) do
    receive do
      {:command, data} ->
          send(port, {self, {:command, data}})
          handle_output(port)
      {^port, {:data, data}} ->
          IO.puts(data)
          handle_output(port)
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