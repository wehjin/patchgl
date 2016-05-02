defmodule Shui.Viewer do

  def start() do
    pid = spawn(fn() -> open_window() end)
    {pid, :root}
  end

  def patch({pid, :root} = viewer, color, position) do
    send_begin_patch(pid, color, position)
  end

  def send_begin_patch(pid, color, position) do
    %{:red=>red, :green=>green, :blue=>blue} = color
    %{:left=>left, :bottom=>bottom, :right=>right, :top=>top} = position
    color_message = Shui.Messages.color(red, green, blue)
    position_message = Shui.Messages.position(left, bottom, right, top)
    message = Shui.Messages.begin_patch_encoded(color_message, position_message)
    send(pid, {:begin_patch, message})
  end

  defp open_window() do
    port = Port.open({:spawn, "patchgl"}, [:stream, :binary, :exit_status, :hide, :use_stdio, :stderr_to_stdout])
    handle_output(port)
  end

  defp handle_output(port) do
    receive do
      {:begin_patch, data} ->
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