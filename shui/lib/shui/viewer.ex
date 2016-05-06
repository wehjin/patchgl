defmodule Shui.Viewer do
  alias Shui.Position
  alias Shui.Messages

  def start() do
    pid = spawn fn() ->
        loop_window(Port.open({:spawn, "patchgl"}, [:stream, :binary, :exit_status, :hide, :use_stdio, :stderr_to_stdout]))
    end
    {:root, pid, Position.full()}
  end
  def start_recorder() do
    pid = spawn fn() ->
      loop_recorder(%{})
    end
    {:recorder, pid, Position.full()}
  end
  def start_proxy(proxy_pid) do
    {:proxy, proxy_pid, Position.full()}
  end

  def stop({:root, pid, _position}) do
    message = Messages.close_encoded()
    send(pid, {:command, message})
  end
  def stop({:recorder, pid, _position}) do
    send(pid, :close)
  end

  def inspect({:recorder, pid, _position}, report_pid) do
    send(pid, {:report, report_pid})
  end

  def patch({:child, parent, _position}, color, position, id) do
    patch(parent, color, position, id)
  end
  def patch({:root, pid, _position}, color, position, id) do
    %{:red=>red, :green=>green, :blue=>blue} = color
    %{:left=>left, :bottom=>bottom, :right=>right, :top=>top} = position
    color_message = Messages.color(red, green, blue)
    position_message = Messages.position(left, bottom, right, top)
    message = Messages.begin_patch_encoded(color_message, position_message, id)
    send(pid, {:command, message})
  end
  def patch({:recorder, pid, _position}, color, position, id) do
    send(pid, {:patch, id, %{color: color, position: position}})
  end
  def patch({:proxy, proxy_pid, _position}, color, position, id) do
    send(proxy_pid, {:patch, id, %{color: color, position: position}})
  end

  def unpatch({:child, parent, _position}, id) do
    unpatch(parent, id)
  end
  def unpatch({:root, pid, _position}, id) do
    send(pid, {:command, Shui.Messages.end_patch_encoded(id)})
  end
  def unpatch({:recorder, pid, _position}, id) do
    send(pid, {:unpatch, id})
  end
  def unpatch({:proxy, proxy_pid, _position}, id) do
    send(proxy_pid, {:unpatch, id})
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

  defp loop_recorder(patches) do
    receive do
      {:patch, id, patch} ->
        loop_recorder(Map.put(patches, id, patch))
      {:unpatch, id} ->
        loop_recorder(Map.delete(id))
      {:report, pid} ->
        send(pid, {:patches, patches})
        loop_recorder(patches)
      :close -> :close
    end
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