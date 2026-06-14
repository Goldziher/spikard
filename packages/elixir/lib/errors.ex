defmodule Spikard.Errors do
  @moduledoc """
  Generated exception types.
  """

  @doc "Raised when the requested resource does not exist."
  defmodule NotFoundError do
    defexception [:message, :status_code, :problem_details]

    def new(message, status_code \\ 404, problem_details \\ nil) do
      %__MODULE__{
        message: message,
        status_code: status_code,
        problem_details: problem_details
      }
    end
  end

  @doc "Raised when input validation fails. Carries a list of field errors per RFC 9457."
  defmodule ValidationError do
    defexception [:message, :status_code, :problem_details]

    def new(message, status_code \\ 422, problem_details \\ nil) do
      %__MODULE__{
        message: message,
        status_code: status_code,
        problem_details: problem_details
      }
    end
  end

  @doc "Raised when the request lacks valid authentication credentials."
  defmodule UnauthorizedError do
    defexception [:message, :status_code, :problem_details]

    def new(message, status_code \\ 401, problem_details \\ nil) do
      %__MODULE__{
        message: message,
        status_code: status_code,
        problem_details: problem_details
      }
    end
  end

  @doc "Raised when the authenticated user lacks permission for the requested action."
  defmodule ForbiddenError do
    defexception [:message, :status_code, :problem_details]

    def new(message, status_code \\ 403, problem_details \\ nil) do
      %__MODULE__{
        message: message,
        status_code: status_code,
        problem_details: problem_details
      }
    end
  end

  @doc "Raised when the client exceeds the configured request rate limit."
  defmodule RateLimitedError do
    defexception [:message, :status_code, :problem_details]

    def new(message, status_code \\ 429, problem_details \\ nil) do
      %__MODULE__{
        message: message,
        status_code: status_code,
        problem_details: problem_details
      }
    end
  end

  @doc "Raised when the request conflicts with the current state of the resource."
  defmodule ConflictError do
    defexception [:message, :status_code, :problem_details]

    def new(message, status_code \\ 409, problem_details \\ nil) do
      %__MODULE__{
        message: message,
        status_code: status_code,
        problem_details: problem_details
      }
    end
  end

  @doc "Raised when the server encounters an unexpected failure."
  defmodule InternalError do
    defexception [:message, :status_code, :problem_details]

    def new(message, status_code \\ 500, problem_details \\ nil) do
      %__MODULE__{
        message: message,
        status_code: status_code,
        problem_details: problem_details
      }
    end
  end
end
