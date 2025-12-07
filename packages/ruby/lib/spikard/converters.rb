# frozen_string_literal: true

module Spikard
  # Conversion helpers between native Rust values and Ruby types.
  module Converters
    module_function

    # No-op conversion now that Rust materialises UploadFile.
    def convert_handler_body(body)
      body
    end
  end
end
