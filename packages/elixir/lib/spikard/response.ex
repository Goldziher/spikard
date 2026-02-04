defmodule Spikard.Response do
  @moduledoc """
  Response builders for Spikard handlers.
  """

  @type t :: %{
          status: non_neg_integer(),
          headers: [{String.t(), String.t()}],
          body: binary() | nil
        }

  @doc """
  Creates a JSON response.

  ## Examples

      Spikard.Response.json(%{hello: "world"})
      #=> %{status: 200, headers: [{"content-type", "application/json"}], body: "{\"hello\":\"world\"}"}
  """
  @spec json(term(), keyword()) :: t()
  def json(data, opts \\ []) do
    status = Keyword.get(opts, :status, 200)
    body = Jason.encode!(data)

    %{
      status: status,
      headers: [{"content-type", "application/json"}],
      body: body
    }
  end

  @doc """
  Creates a plain text response.
  """
  @spec text(String.t(), keyword()) :: t()
  def text(content, opts \\ []) do
    status = Keyword.get(opts, :status, 200)

    %{
      status: status,
      headers: [{"content-type", "text/plain; charset=utf-8"}],
      body: content
    }
  end

  @doc """
  Creates a response with a specific status code and no body.
  """
  @spec status(non_neg_integer()) :: t()
  def status(code) do
    %{
      status: code,
      headers: [],
      body: nil
    }
  end
end
