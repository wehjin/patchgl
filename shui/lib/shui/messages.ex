defmodule Shui.Messages do
  @moduledoc false

  use Protobuf, """
  message Close {
  }

  message BeginPatch {
      message Position {
          required float left = 1;
          required float right = 2;
          required float top = 3;
          required float bottom = 4;
          optional float near = 5 [default = 1.0];
          optional float far = 6 [default = 1.0];
      }
      message Color {
          required float red = 1;
          required float green = 2;
          required float blue = 3;
          optional float alpha = 4 [default = 1.0];
      }

      required Position position = 1;
      required Color color = 2;
      optional fixed32 parent = 3 [default = 0];
      required fixed32 patch_id = 4;
  }

  message EndPatch {
      required fixed32 patch_id = 1;
  }

  message Command {
      oneof requests {
          Close close = 1;
          BeginPatch begin_patch = 2;
          EndPatch end_patch = 3;
      }
  }
  """

  def patch_id() do
    :random.uniform(2000000000) + 2
  end

  def color(red, green, blue) do
    Shui.Messages.BeginPatch.Color.new(red: red, green: green, blue: blue)
  end

  def position(left, bottom, right, top) do
    Shui.Messages.BeginPatch.Position.new(left: left, bottom: bottom, right: right, top: top)
  end

  def begin_patch_command(color, position, id) do
    begin_patch = Shui.Messages.BeginPatch.new(patch_id: id, color: color, position: position)
    Shui.Messages.Command.new(requests: {:begin_patch, begin_patch})
  end

  def end_patch_command(id) do
    end_patch = Shui.Messages.EndPatch.new(patch_id: id)
    Shui.Messages.Command.new(requests: {:end_patch, end_patch})
  end

  def packet(encoded) do
    length = byte_size(encoded)
    <<length>> <> encoded
  end

  def begin_patch_encoded(color, position, id) do
    encoded = Shui.Messages.Command.encode(begin_patch_command(color, position, id))
    packet(encoded)
  end

  def end_patch_encoded(id) do
    encoded = Shui.Messages.Command.encode(end_patch_command(id))
    packet(encoded)
  end
end