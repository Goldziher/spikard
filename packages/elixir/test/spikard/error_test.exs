defmodule Spikard.ErrorTest do
  use ExUnit.Case, async: true

  alias Spikard.Error

  describe "exception/1 with atom reasons" do
    test "handles :not_implemented" do
      exception = Error.exception(:not_implemented)
      assert %Error{} = exception
      assert exception.reason == :not_implemented
      assert exception.message == "Feature not yet implemented"
    end

    test "handles :server_error" do
      exception = Error.exception(:server_error)
      assert %Error{} = exception
      assert exception.reason == :server_error
      assert exception.message == "Server error occurred"
    end

    test "handles :invalid_config" do
      exception = Error.exception(:invalid_config)
      assert %Error{} = exception
      assert exception.reason == :invalid_config
      assert exception.message == "Invalid configuration"
    end

    test "handles :route_not_found" do
      exception = Error.exception(:route_not_found)
      assert %Error{} = exception
      assert exception.reason == :route_not_found
      assert exception.message == "Route not found"
    end

    test "handles :handler_error" do
      exception = Error.exception(:handler_error)
      assert %Error{} = exception
      assert exception.reason == :handler_error
      assert exception.message == "Handler execution failed"
    end

    test "handles unknown atom reason" do
      exception = Error.exception(:unknown_reason)
      assert %Error{} = exception
      assert exception.reason == :unknown_reason
      assert exception.message =~ "Unknown error"
      assert exception.message =~ "unknown_reason"
    end
  end

  describe "exception/1 with tuple reasons" do
    test "handles {reason, details} tuple" do
      exception = Error.exception({:custom_error, "some details"})
      assert %Error{} = exception
      assert exception.reason == :custom_error
      assert exception.message =~ "Unknown error"
      assert exception.message =~ "some details"
    end

    test "handles {:nif_error, details} - falls back to unknown" do
      # The reason_to_message pattern matches on {:nif_error, _} but
      # exception/1 with tuple destructures {reason, details} separately
      exception = Error.exception({:nif_error, "socket bind failed"})
      assert %Error{} = exception
      assert exception.reason == :nif_error
      # The message is built with reason_to_message(:nif_error) which returns "Unknown error: :nif_error"
      assert exception.message =~ "Unknown error"
    end
  end

  describe "message/1" do
    test "returns the exception message" do
      exception = Error.exception(:server_error)
      assert Error.message(exception) == "Server error occurred"
    end
  end

  describe "raise behavior" do
    test "can be raised with atom" do
      assert_raise Error, fn ->
        raise Error, :not_implemented
      end
    end

    test "can be raised with tuple" do
      assert_raise Error, fn ->
        raise Error, {:custom_error, "test error"}
      end
    end

    test "raised exception contains correct message" do
      exception =
        assert_raise Error, fn ->
          raise Error, :server_error
        end

      assert exception.message == "Server error occurred"
    end
  end
end
