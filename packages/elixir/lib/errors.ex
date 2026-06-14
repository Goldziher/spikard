defmodule Spikard.Errors do
  @moduledoc """
  Generated exception types.
  """

  defmodule NotFoundError do
    @moduledoc "Raised when the requested resource does not exist."
    defexception [:message, :status_code, :problem_details]

    def new(message, status_code \\ 404, problem_details \\ nil) do
      %__MODULE__{
        message: message,
        status_code: status_code,
        problem_details: problem_details
      }
    end
  end

  defmodule ValidationError do
    @moduledoc "Raised when input validation fails. Carries a list of field errors per RFC 9457."
    defexception [:message, :status_code, :problem_details]

    def new(message, status_code \\ 422, problem_details \\ nil) do
      %__MODULE__{
        message: message,
        status_code: status_code,
        problem_details: problem_details
      }
    end
  end

  defmodule UnauthorizedError do
    @moduledoc "Raised when the request lacks valid authentication credentials."
    defexception [:message, :status_code, :problem_details]

    def new(message, status_code \\ 401, problem_details \\ nil) do
      %__MODULE__{
        message: message,
        status_code: status_code,
        problem_details: problem_details
      }
    end
  end

  defmodule ForbiddenError do
    @moduledoc "Raised when the authenticated user lacks permission for the requested action."
    defexception [:message, :status_code, :problem_details]

    def new(message, status_code \\ 403, problem_details \\ nil) do
      %__MODULE__{
        message: message,
        status_code: status_code,
        problem_details: problem_details
      }
    end
  end

  defmodule RateLimitedError do
    @moduledoc "Raised when the client exceeds the configured request rate limit."
    defexception [:message, :status_code, :problem_details]

    def new(message, status_code \\ 429, problem_details \\ nil) do
      %__MODULE__{
        message: message,
        status_code: status_code,
        problem_details: problem_details
      }
    end
  end

  defmodule ConflictError do
    @moduledoc "Raised when the request conflicts with the current state of the resource."
    defexception [:message, :status_code, :problem_details]

    def new(message, status_code \\ 409, problem_details \\ nil) do
      %__MODULE__{
        message: message,
        status_code: status_code,
        problem_details: problem_details
      }
    end
  end

  defmodule InternalError do
    @moduledoc "Raised when the server encounters an unexpected failure."
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
