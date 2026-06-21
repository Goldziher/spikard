# frozen_string_literal: true

module Spikard
  module Errors
class Error < StandardError
  attr_reader :status_code, :problem_details_type

  def initialize(message = nil, status_code: nil, problem_details_type: nil)
    super(message)
    @status_code = status_code
    @problem_details_type = problem_details_type
  end

  def to_problem_details
    {
      type: @problem_details_type || "about:blank",
      title: self.class.name.split("::").last,
      status: @status_code || 500,
      detail: message
    }
  end
end
class NotFoundError < Error
    # Raised when the requested resource does not exist.
  def initialize(message = nil)
    super(message, status_code: 404, problem_details_type: nil)
  end
end
class ValidationError < Error
    # Raised when input validation fails. Carries a list of field errors per RFC 9457.
  def initialize(message = nil)
    super(message, status_code: 422, problem_details_type: nil)
  end
end
class UnauthorizedError < Error
    # Raised when the request lacks valid authentication credentials.
  def initialize(message = nil)
    super(message, status_code: 401, problem_details_type: nil)
  end
end
class ForbiddenError < Error
    # Raised when the authenticated user lacks permission for the requested action.
  def initialize(message = nil)
    super(message, status_code: 403, problem_details_type: nil)
  end
end
class RateLimitedError < Error
    # Raised when the client exceeds the configured request rate limit.
  def initialize(message = nil)
    super(message, status_code: 429, problem_details_type: nil)
  end
end
class ConflictError < Error
    # Raised when the request conflicts with the current state of the resource.
  def initialize(message = nil)
    super(message, status_code: 409, problem_details_type: nil)
  end
end
class InternalError < Error
    # Raised when the server encounters an unexpected failure.
  def initialize(message = nil)
    super(message, status_code: 500, problem_details_type: nil)
  end
end
  end
end
