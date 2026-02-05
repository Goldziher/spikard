defmodule Spikard.Error do
  @moduledoc """
  Exception raised when Spikard operations fail.
  """

  defexception [:reason, :message]

  @type reason ::
          :not_implemented
          | :server_error
          | :invalid_config
          | :route_not_found
          | :handler_error
          | {:nif_error, term()}

  @type t :: %__MODULE__{
          reason: reason(),
          message: String.t()
        }

  @impl true
  @spec exception(atom()) :: t()
  def exception(reason) when is_atom(reason) do
    %__MODULE__{
      reason: reason,
      message: reason_to_message(reason)
    }
  end

  @spec exception({atom(), term()}) :: t()
  def exception({reason, details}) do
    %__MODULE__{
      reason: reason,
      message: "#{reason_to_message(reason)}: #{inspect(details)}"
    }
  end

  defp reason_to_message(:not_implemented), do: "Feature not yet implemented"
  defp reason_to_message(:server_error), do: "Server error occurred"
  defp reason_to_message(:invalid_config), do: "Invalid configuration"
  defp reason_to_message(:route_not_found), do: "Route not found"
  defp reason_to_message(:handler_error), do: "Handler execution failed"
  defp reason_to_message({:nif_error, _}), do: "NIF error"
  defp reason_to_message(other), do: "Unknown error: #{inspect(other)}"
end
