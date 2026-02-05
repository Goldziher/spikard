defmodule Spikard.UploadFile do
  @moduledoc """
  Represents an uploaded file in a multipart form request.

  This struct contains all metadata and content of a file that was uploaded
  via a multipart/form-data request.

  ## Fields

    * `:filename` - Original filename provided by the client (string or nil)
    * `:content_type` - MIME type of the file (e.g., "text/plain", "image/jpeg")
    * `:size` - File size in bytes
    * `:data` - File content as binary

  ## Examples

      # Access file in a handler
      handler = fn req ->
        files = Spikard.Request.files(req)
        case files do
          [file] ->
            %{
              status: 200,
              body: %{
                filename: file.filename,
                size: file.size,
                content_type: file.content_type
              }
            }
          [] ->
            %{status: 400, body: %{error: "No file uploaded"}}
        end
      end

      # Create manually for testing
      file = Spikard.UploadFile.new("test.txt", "text/plain", 11, "hello world")
  """

  @type t :: %__MODULE__{
          filename: String.t() | nil,
          content_type: String.t(),
          size: non_neg_integer(),
          data: binary()
        }

  defstruct [:filename, :content_type, :size, :data]

  @doc """
  Creates a new UploadFile struct.

  ## Parameters

    * `filename` - Original filename (string or nil)
    * `content_type` - MIME type of the file
    * `size` - Size in bytes
    * `data` - File content as binary

  ## Returns

    An UploadFile struct with the given values.

  ## Examples

      iex> file = Spikard.UploadFile.new("test.txt", "text/plain", 5, "hello")
      iex> file.filename
      "test.txt"
      iex> file.size
      5

      iex> image = Spikard.UploadFile.new("photo.jpg", "image/jpeg", 1024, <<0xFF, 0xD8>>)
      iex> image.content_type
      "image/jpeg"
  """
  @spec new(String.t() | nil, String.t(), non_neg_integer(), binary()) :: t()
  def new(filename, content_type, size, data) do
    %__MODULE__{
      filename: filename,
      content_type: content_type,
      size: size,
      data: data
    }
  end

  @doc """
  Checks if the file is a text file based on MIME type.

  ## Examples

      iex> file = Spikard.UploadFile.new("test.txt", "text/plain", 5, "hello")
      iex> Spikard.UploadFile.text?(file)
      true

      iex> file = Spikard.UploadFile.new("image.jpg", "image/jpeg", 100, <<0xFF>>)
      iex> Spikard.UploadFile.text?(file)
      false
  """
  @spec text?(t()) :: boolean()
  def text?(%__MODULE__{content_type: content_type}) do
    content_type
    |> String.downcase()
    |> then(fn ct ->
      String.starts_with?(ct, "text/") or
        ct == "application/json" or
        ct == "application/xml"
    end)
  end

  @doc """
  Checks if the file is an image based on MIME type.

  ## Examples

      iex> file = Spikard.UploadFile.new("photo.jpg", "image/jpeg", 100, <<0xFF>>)
      iex> Spikard.UploadFile.image?(file)
      true

      iex> file = Spikard.UploadFile.new("test.txt", "text/plain", 5, "hello")
      iex> Spikard.UploadFile.image?(file)
      false
  """
  @spec image?(t()) :: boolean()
  def image?(%__MODULE__{content_type: content_type}) do
    content_type
    |> String.downcase()
    |> String.starts_with?("image/")
  end

  @doc """
  Gets the file extension from the filename.

  Returns the extension without the dot, or nil if no filename or no extension.

  ## Examples

      iex> file = Spikard.UploadFile.new("document.pdf", "application/pdf", 100, "")
      iex> Spikard.UploadFile.extension(file)
      "pdf"

      iex> file = Spikard.UploadFile.new("archive.tar.gz", "application/gzip", 100, "")
      iex> Spikard.UploadFile.extension(file)
      "gz"

      iex> file = Spikard.UploadFile.new(nil, "text/plain", 5, "data")
      iex> Spikard.UploadFile.extension(file)
      nil
  """
  @spec extension(t()) :: String.t() | nil
  def extension(%__MODULE__{filename: nil}), do: nil

  def extension(%__MODULE__{filename: filename}) do
    case String.split(filename, ".") do
      [_] -> nil
      parts -> parts |> List.last() |> String.downcase()
    end
  end
end
