defmodule Shui.Messages do
  @moduledoc false

  use Protobuf, """
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
    }

    message BeginPatchResponse {
        required fixed32 patch = 1;
    }
  """

  def color(red, green, blue) do
    Shui.Messages.BeginPatch.Color.new(red: red, green: green, blue: blue)
  end

  def position(left, bottom, right, top) do
    Shui.Messages.BeginPatch.Position.new(left: left, bottom: bottom, right: right, top: top)
  end

  def begin_patch(color, position) do
    Shui.Messages.BeginPatch.new(color: color, position: position)
  end

  def begin_patch_encoded(color, position) do
    encoded = Shui.Messages.BeginPatch.encode(begin_patch(color,position))
    length = byte_size(encoded)
    <<length>> <> encoded
  end
end