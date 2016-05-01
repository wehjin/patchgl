defmodule Shui do

    def open() do
        port = Port.open({:spawn, "patchgl"}, [:stream, :binary, :exit_status, :hide, :use_stdio, :stderr_to_stdout])
        handle_output(port)
    end

    def handle_output(port) do
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
