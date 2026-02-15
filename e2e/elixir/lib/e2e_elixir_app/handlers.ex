defmodule E2EElixirApp.Handlers do
  @moduledoc """
  Generated handler functions from test fixtures.

  These handlers extract data from the Spikard.Request struct and return
  responses that match the fixture expectations. Most handlers echo back
  request data (headers, params, body) to verify correct parsing.
  """

  alias Spikard.{Request, Response}

  @doc """
  Build a response with the given content, status, and optional headers.
  """
  def build_response(content, status, headers \\ %{}) do
    %{
      status: status,
      headers: headers,
      body: content
    }
  end

  @doc """
  Get a header value from the request, case-insensitively.
  """
  def get_header(request, name) do
    name_lower = String.downcase(name)

    Enum.find_value(request.headers, fn {k, v} ->
      if String.downcase(k) == name_lower, do: v, else: nil
    end)
  end

  @doc """
  Get a query parameter from the request.
  """
  def get_query_param(request, name, default \\ nil) do
    Map.get(request.query_params || %{}, name, default)
  end

  @doc """
  Get a path parameter from the request.
  """
  def get_path_param(request, name, default \\ nil) do
    Map.get(request.path_params || %{}, name, default)
  end

  @doc """
  Handler for fixture: auth - API key authentication - invalid key
  """
  def handle_auth_api_key_authentication___invalid_key(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/unauthorized",
        "title" => "Invalid API key",
        "status" => 401,
        "detail" => "The provided API key is not valid"
      },
      401,
      %{}
    )
  end

  @doc """
  Handler for fixture: auth - API key authentication - missing header
  """
  def handle_auth_api_key_authentication___missing_header(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/unauthorized",
        "title" => "Missing API key",
        "status" => 401,
        "detail" => "Expected 'X-API-Key' header or 'api_key' query parameter with valid API key"
      },
      401,
      %{}
    )
  end

  @doc """
  Handler for fixture: auth - API key authentication - valid key
  """
  def handle_auth_api_key_authentication___valid_key(_request) do
    build_response(%{"message" => "Access granted", "data" => "sensitive information"}, 200, %{})
  end

  @doc """
  Handler for fixture: auth - API key in query parameter
  """
  def handle_auth_api_key_in_query_parameter(_request) do
    build_response(%{"message" => "Access granted", "data" => "sensitive information"}, 200, %{})
  end

  @doc """
  Handler for fixture: auth - API key rotation - old key still valid
  """
  def handle_auth_api_key_rotation___old_key_still_valid(_request) do
    build_response(%{"message" => "Access granted", "data" => "sensitive information"}, 200, %{
      "X-API-Key-Deprecated" => "true"
    })
  end

  @doc """
  Handler for fixture: auth - API key with custom header name
  """
  def handle_auth_api_key_with_custom_header_name(_request) do
    build_response(%{"message" => "Access granted", "data" => "sensitive information"}, 200, %{})
  end

  @doc """
  Handler for fixture: auth - Bearer token without prefix
  """
  def handle_auth_bearer_token_without_prefix(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/unauthorized",
        "title" => "Invalid Authorization header format",
        "status" => 401,
        "detail" => "Authorization header must use Bearer scheme: 'Bearer <token>'"
      },
      401,
      %{}
    )
  end

  @doc """
  Handler for fixture: auth - JWT authentication - expired token
  """
  def handle_auth_jwt_authentication___expired_token(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/unauthorized",
        "title" => "JWT validation failed",
        "status" => 401,
        "detail" => "Token has expired"
      },
      401,
      %{}
    )
  end

  @doc """
  Handler for fixture: auth - JWT authentication - invalid audience
  """
  def handle_auth_jwt_authentication___invalid_audience(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/unauthorized",
        "title" => "JWT validation failed",
        "status" => 401,
        "detail" => "Token audience is invalid"
      },
      401,
      %{}
    )
  end

  @doc """
  Handler for fixture: auth - JWT authentication - invalid signature
  """
  def handle_auth_jwt_authentication___invalid_signature(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/unauthorized",
        "title" => "JWT validation failed",
        "status" => 401,
        "detail" => "Token signature is invalid"
      },
      401,
      %{}
    )
  end

  @doc """
  Handler for fixture: auth - JWT authentication - missing Authorization header
  """
  def handle_auth_jwt_authentication___missing_authorization_header(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/unauthorized",
        "title" => "Missing or invalid Authorization header",
        "status" => 401,
        "detail" => "Expected 'Authorization: Bearer <token>'"
      },
      401,
      %{}
    )
  end

  @doc """
  Handler for fixture: auth - JWT authentication - valid token
  """
  def handle_auth_jwt_authentication___valid_token(_request) do
    build_response(%{"message" => "Access granted", "user_id" => "user123"}, 200, %{})
  end

  @doc """
  Handler for fixture: auth - JWT invalid issuer
  """
  def handle_auth_jwt_invalid_issuer(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/unauthorized",
        "title" => "JWT validation failed",
        "status" => 401,
        "detail" => "Token issuer is invalid, expected 'https://auth.example.com'"
      },
      401,
      %{}
    )
  end

  @doc """
  Handler for fixture: auth - JWT malformed token format
  """
  def handle_auth_jwt_malformed_token_format(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/unauthorized",
        "title" => "Malformed JWT token",
        "status" => 401,
        "detail" => "Malformed JWT token: expected 3 parts separated by dots, found 2"
      },
      401,
      %{}
    )
  end

  @doc """
  Handler for fixture: auth - JWT missing required custom claims
  """
  def handle_auth_jwt_missing_required_custom_claims(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/forbidden",
        "title" => "Forbidden",
        "status" => 403,
        "detail" => "Required claims 'role' and 'permissions' missing from JWT"
      },
      403,
      %{}
    )
  end

  @doc """
  Handler for fixture: auth - JWT not before claim in future
  """
  def handle_auth_jwt_not_before_claim_in_future(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/unauthorized",
        "title" => "JWT validation failed",
        "status" => 401,
        "detail" => "JWT not valid yet, not before claim is in the future"
      },
      401,
      %{}
    )
  end

  @doc """
  Handler for fixture: auth - JWT with multiple audiences
  """
  def handle_auth_jwt_with_multiple_audiences(_request) do
    build_response(%{"message" => "Access granted", "user_id" => "user123"}, 200, %{})
  end

  @doc """
  Handler for fixture: auth - Multiple authentication schemes - JWT precedence
  """
  def handle_auth_multiple_authentication_schemes___jwt_precedence(_request) do
    build_response(
      %{"message" => "Access granted", "user_id" => "user123", "auth_method" => "jwt"},
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: background - Background event logging
  """
  def handle_background_background_event_logging(_request) do
    build_response(nil, 202, %{"content-type" => "application/json"})
  end

  @doc """
  Handler for fixture: background - Background event logging - second payload
  """
  def handle_background_background_event_logging___second_payload(_request) do
    build_response(nil, 202, %{"content-type" => "application/json"})
  end

  @doc """
  Handler for fixture: body_limits - Body over limit returns 413
  """
  def handle_body_limits_body_over_limit_returns_413(_request) do
    build_response(nil, 413, %{})
  end

  @doc """
  Handler for fixture: body_limits - Body under limit succeeds
  """
  def handle_body_limits_body_under_limit_succeeds(_request) do
    build_response(%{"accepted" => true, "note" => "small"}, 200, %{})
  end

  @doc """
  Handler for fixture: compression - Compression - gzip applied
  """
  def handle_compression_compression___gzip_applied(_request) do
    build_response(
      %{
        "message" => "Compressed payload",
        "payload" =>
          "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
      },
      200,
      %{"content-encoding" => "gzip", "vary" => "Accept-Encoding"}
    )
  end

  @doc """
  Handler for fixture: compression - Compression - payload below min_size is not compressed
  """
  def handle_compression_compression___payload_below_min_size_is_not_compressed(_request) do
    build_response(%{"message" => "Small payload", "payload" => "tiny"}, 200, %{
      "content-encoding" => "<<absent>>"
    })
  end

  @doc """
  Handler for fixture: content_types - 13_json_with_charset_utf16
  """
  def handle_content_types_13_json_with_charset_utf16(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/unsupported-charset",
        "title" => "Unsupported Charset",
        "status" => 415,
        "detail" => "Unsupported charset 'utf-16' for JSON. Only UTF-8 is supported."
      },
      415,
      %{}
    )
  end

  @doc """
  Handler for fixture: content_types - 14_content_type_case_insensitive
  """
  def handle_content_types_14_content_type_case_insensitive(_request) do
    build_response(%{"name" => "test"}, 201, %{})
  end

  @doc """
  Handler for fixture: content_types - 15_multipart_boundary_required
  """
  def handle_content_types_15_multipart_boundary_required(_request) do
    build_response(%{"error" => "multipart/form-data requires 'boundary' parameter"}, 400, %{})
  end

  @doc """
  Handler for fixture: content_types - 16_text_plain_not_accepted
  """
  def handle_content_types_16_text_plain_not_accepted(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/unsupported-media-type",
        "title" => "Unsupported Media Type",
        "status" => 415,
        "detail" => "Unsupported media type"
      },
      415,
      %{}
    )
  end

  @doc """
  Handler for fixture: content_types - 17_vendor_json_accepted
  """
  def handle_content_types_17_vendor_json_accepted(_request) do
    build_response(%{"data" => "value"}, 201, %{})
  end

  @doc """
  Handler for fixture: content_types - 18_content_type_with_multiple_params
  """
  def handle_content_types_18_content_type_with_multiple_params(_request) do
    build_response(%{"value" => "test"}, 201, %{})
  end

  @doc """
  Handler for fixture: content_types - 19_missing_content_type_default_json
  """
  def handle_content_types_19_missing_content_type_default_json(_request) do
    build_response(%{"name" => "test"}, 201, %{})
  end

  @doc """
  Handler for fixture: content_types - 20_content_length_mismatch
  """
  def handle_content_types_20_content_length_mismatch(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/content-length-mismatch",
        "title" => "Content-Length header mismatch",
        "status" => 400,
        "detail" => "Content-Length header does not match actual body size"
      },
      400,
      %{}
    )
  end

  @doc """
  Handler for fixture: content_types - 415 Unsupported Media Type
  """
  def handle_content_types_415_unsupported_media_type(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/unsupported-media-type",
        "title" => "Unsupported Media Type",
        "status" => 415,
        "detail" => "Unsupported media type"
      },
      415,
      %{}
    )
  end

  @doc """
  Handler for fixture: content_types - Binary response - application/octet-stream
  """
  def handle_content_types_binary_response___application_octet_stream(_request) do
    build_response("binary_data_placeholder", 200, %{
      "content-type" => "application/octet-stream",
      "content-disposition" => "attachment; filename=file.bin"
    })
  end

  @doc """
  Handler for fixture: content_types - CSV response - text/csv
  """
  def handle_content_types_csv_response___text_csv(_request) do
    build_response("id,name,price\n1,Item A,10.0\n2,Item B,20.0", 200, %{
      "content-type" => "text/csv; charset=utf-8",
      "content-disposition" => "attachment; filename=data.csv"
    })
  end

  @doc """
  Handler for fixture: content_types - Content negotiation - Accept header
  """
  def handle_content_types_content_negotiation___accept_header(_request) do
    build_response(%{"id" => 1, "name" => "Item"}, 200, %{"content-type" => "application/json"})
  end

  @doc """
  Handler for fixture: content_types - HTML response - text/html
  """
  def handle_content_types_html_response___text_html(_request) do
    build_response("<html><body><h1>Hello</h1></body></html>", 200, %{
      "content-type" => "text/html; charset=utf-8"
    })
  end

  @doc """
  Handler for fixture: content_types - JPEG image response - image/jpeg
  """
  def handle_content_types_jpeg_image_response___image_jpeg(_request) do
    build_response("jpeg_binary_data", 200, %{"content-type" => "image/jpeg"})
  end

  @doc """
  Handler for fixture: content_types - JSON response - application/json
  """
  def handle_content_types_json_response___application_json(_request) do
    build_response(%{"name" => "Item", "price" => 42.0}, 200, %{
      "content-type" => "application/json"
    })
  end

  @doc """
  Handler for fixture: content_types - JSON with UTF-8 charset
  """
  def handle_content_types_json_with_utf_8_charset(_request) do
    build_response(%{"name" => "Café", "emoji" => "☕"}, 200, %{
      "content-type" => "application/json; charset=utf-8"
    })
  end

  @doc """
  Handler for fixture: content_types - PDF response - application/pdf
  """
  def handle_content_types_pdf_response___application_pdf(_request) do
    build_response("pdf_binary_data", 200, %{
      "content-type" => "application/pdf",
      "content-disposition" => "attachment; filename=document.pdf"
    })
  end

  @doc """
  Handler for fixture: content_types - PNG image response - image/png
  """
  def handle_content_types_png_image_response___image_png(_request) do
    build_response("png_binary_data", 200, %{"content-type" => "image/png"})
  end

  @doc """
  Handler for fixture: content_types - Plain text response - text/plain
  """
  def handle_content_types_plain_text_response___text_plain(_request) do
    build_response("Hello, World!", 200, %{"content-type" => "text/plain; charset=utf-8"})
  end

  @doc """
  Handler for fixture: content_types - XML response - application/xml
  """
  def handle_content_types_xml_response___application_xml(_request) do
    build_response(
      "<?xml version=\"1.0\"?><item><name>Item</name><price>42.0</price></item>",
      200,
      %{"content-type" => "application/xml"}
    )
  end

  @doc """
  Handler for fixture: cookies - 24_cookie_samesite_strict
  """
  def handle_cookies_24_cookie_samesite_strict(_request) do
    build_response(nil, 200, %{})
  end

  @doc """
  Handler for fixture: cookies - 25_cookie_samesite_lax
  """
  def handle_cookies_25_cookie_samesite_lax(_request) do
    build_response(nil, 200, %{})
  end

  @doc """
  Handler for fixture: cookies - 26_cookie_secure_flag
  """
  def handle_cookies_26_cookie_secure_flag(_request) do
    build_response(nil, 200, %{})
  end

  @doc """
  Handler for fixture: cookies - 27_cookie_httponly_flag
  """
  def handle_cookies_27_cookie_httponly_flag(_request) do
    build_response(nil, 200, %{})
  end

  @doc """
  Handler for fixture: cookies - APIKey cookie authentication - missing
  """
  def handle_cookies_apikey_cookie_authentication___missing(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "missing",
            "loc" => ["cookie", "key"],
            "msg" => "Field required",
            "input" => nil
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: cookies - APIKey cookie authentication - success
  """
  def handle_cookies_apikey_cookie_authentication___success(_request) do
    build_response(%{"username" => "secret"}, 200, %{})
  end

  @doc """
  Handler for fixture: cookies - Cookie regex pattern validation - fail
  """
  def handle_cookies_cookie_regex_pattern_validation___fail(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "string_pattern_mismatch",
            "loc" => ["cookie", "tracking_id"],
            "msg" => "String should match pattern '^[A-Z0-9]{8}$'",
            "input" => "invalid-format",
            "ctx" => %{"pattern" => "^[A-Z0-9]{8}$"}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: cookies - Cookie regex pattern validation - success
  """
  def handle_cookies_cookie_regex_pattern_validation___success(_request) do
    build_response(%{"tracking_id" => "ABC12345"}, 200, %{})
  end

  @doc """
  Handler for fixture: cookies - Cookie validation - max_length constraint fail
  """
  def handle_cookies_cookie_validation___max_length_constraint_fail(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "string_too_long",
            "loc" => ["cookie", "session_id"],
            "msg" => "String should have at most 20 characters",
            "input" => "this_cookie_value_is_way_too_long",
            "ctx" => %{"max_length" => 20}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: cookies - Cookie validation - min_length constraint success
  """
  def handle_cookies_cookie_validation___min_length_constraint_success(_request) do
    build_response(%{"token" => "abc"}, 200, %{})
  end

  @doc """
  Handler for fixture: cookies - Cookie validation - min_length failure
  """
  def handle_cookies_cookie_validation___min_length_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "string_too_short",
            "loc" => ["cookie", "tracking_id"],
            "msg" => "String should have at least 3 characters",
            "input" => ""
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: cookies - Multiple cookies - success
  """
  def handle_cookies_multiple_cookies___success(_request) do
    build_response(
      %{
        "session_id" => "session123",
        "fatebook_tracker" => "tracker456",
        "googall_tracker" => "ga789"
      },
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: cookies - Optional APIKey cookie - missing
  """
  def handle_cookies_optional_apikey_cookie___missing(_request) do
    build_response(%{"msg" => "Create an account first"}, 200, %{})
  end

  @doc """
  Handler for fixture: cookies - Optional cookie parameter - missing
  """
  def handle_cookies_optional_cookie_parameter___missing(_request) do
    build_response(%{"ads_id" => nil}, 200, %{})
  end

  @doc """
  Handler for fixture: cookies - Optional cookie parameter - success
  """
  def handle_cookies_optional_cookie_parameter___success(_request) do
    build_response(%{"ads_id" => "abc123"}, 200, %{})
  end

  @doc """
  Handler for fixture: cookies - Required cookie - missing
  """
  def handle_cookies_required_cookie___missing(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "missing",
            "loc" => ["cookie", "session_id"],
            "msg" => "Field required",
            "input" => ""
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: cookies - Response - delete cookie
  """
  def handle_cookies_response___delete_cookie(_request) do
    build_response(%{"message" => "Cookie deleted"}, 200, %{})
  end

  @doc """
  Handler for fixture: cookies - Response - multiple cookies
  """
  def handle_cookies_response___multiple_cookies(_request) do
    build_response(%{"message" => "Multiple cookies set"}, 200, %{})
  end

  @doc """
  Handler for fixture: cookies - Response - session cookie (no max_age)
  """
  def handle_cookies_response___session_cookie__no_max_age_(_request) do
    build_response(%{"message" => "Session cookie set"}, 200, %{})
  end

  @doc """
  Handler for fixture: cookies - Response cookie with SameSite=Lax
  """
  def handle_cookies_response_cookie_with_samesite_lax(_request) do
    build_response(%{"message" => "Cookie set with SameSite=Lax"}, 200, %{})
  end

  @doc """
  Handler for fixture: cookies - Response cookie with SameSite=None
  """
  def handle_cookies_response_cookie_with_samesite_none(_request) do
    build_response(%{"message" => "Cookie set with SameSite=None"}, 200, %{})
  end

  @doc """
  Handler for fixture: cookies - Response cookie with SameSite=Strict
  """
  def handle_cookies_response_cookie_with_samesite_strict(_request) do
    build_response(%{"message" => "Cookie set with SameSite=Strict"}, 200, %{})
  end

  @doc """
  Handler for fixture: cookies - Response cookie with attributes
  """
  def handle_cookies_response_cookie_with_attributes(_request) do
    build_response(%{"message" => "Cookie set"}, 200, %{})
  end

  @doc """
  Handler for fixture: cookies - Response cookie with domain attribute
  """
  def handle_cookies_response_cookie_with_domain_attribute(_request) do
    build_response(%{"message" => "Cookie set with domain"}, 200, %{})
  end

  @doc """
  Handler for fixture: cookies - Response cookie with path attribute
  """
  def handle_cookies_response_cookie_with_path_attribute(_request) do
    build_response(%{"message" => "Cookie set with path"}, 200, %{})
  end

  @doc """
  Handler for fixture: cookies - Response set cookie - basic
  """
  def handle_cookies_response_set_cookie___basic(_request) do
    build_response(%{"message" => "Come to the dark side, we have cookies"}, 200, %{})
  end

  @doc """
  Handler for fixture: cors - 06_cors_preflight_method_not_allowed
  """
  def handle_cors_06_cors_preflight_method_not_allowed(_request) do
    build_response(nil, 403, %{})
  end

  @doc """
  Handler for fixture: cors - 07_cors_preflight_header_not_allowed
  """
  def handle_cors_07_cors_preflight_header_not_allowed(_request) do
    build_response(nil, 403, %{})
  end

  @doc """
  Handler for fixture: cors - 08_cors_max_age
  """
  def handle_cors_08_cors_max_age(_request) do
    build_response(nil, 204, %{
      "Access-Control-Allow-Methods" => "POST",
      "Access-Control-Allow-Headers" => "Content-Type",
      "Access-Control-Max-Age" => "3600",
      "Access-Control-Allow-Origin" => "https://example.com"
    })
  end

  @doc """
  Handler for fixture: cors - 09_cors_expose_headers
  """
  def handle_cors_09_cors_expose_headers(_request) do
    build_response(nil, 200, %{
      "Access-Control-Allow-Origin" => "https://example.com",
      "X-Request-Id" => "abc123",
      "X-Total-Count" => "42",
      "Access-Control-Expose-Headers" => "X-Total-Count, X-Request-Id"
    })
  end

  @doc """
  Handler for fixture: cors - 10_cors_origin_null
  """
  def handle_cors_10_cors_origin_null(_request) do
    build_response(%{"error" => "Origin 'null' is not allowed"}, 403, %{})
  end

  @doc """
  Handler for fixture: cors - CORS Private Network Access
  """
  def handle_cors_cors_private_network_access(_request) do
    build_response(nil, 204, %{
      "Vary" => "Origin",
      "Access-Control-Allow-Private-Network" => "true",
      "Access-Control-Allow-Origin" => "https://public.example.com",
      "Access-Control-Allow-Methods" => "GET, POST"
    })
  end

  @doc """
  Handler for fixture: cors - CORS Vary header for proper caching
  """
  def handle_cors_cors_vary_header_for_proper_caching(_request) do
    build_response(%{"data" => "cacheable resource"}, 200, %{
      "Access-Control-Allow-Origin" => "https://app.example.com",
      "Cache-Control" => "public, max-age=3600",
      "Vary" => "Origin"
    })
  end

  @doc """
  Handler for fixture: cors - CORS multiple allowed origins
  """
  def handle_cors_cors_multiple_allowed_origins(_request) do
    build_response(%{"data" => "resource data"}, 200, %{
      "Vary" => "Origin",
      "Access-Control-Allow-Origin" => "https://admin.example.com"
    })
  end

  @doc """
  Handler for fixture: cors - CORS origin case sensitivity
  """
  def handle_cors_cors_origin_case_sensitivity(_request) do
    build_response(nil, 200, %{"Vary" => "Origin"})
  end

  @doc """
  Handler for fixture: cors - CORS preflight for DELETE method
  """
  def handle_cors_cors_preflight_for_delete_method(_request) do
    build_response(nil, 204, %{
      "Access-Control-Allow-Methods" => "GET, POST, PUT, PATCH, DELETE",
      "Vary" => "Origin",
      "Access-Control-Max-Age" => "3600",
      "Access-Control-Allow-Origin" => "https://app.example.com"
    })
  end

  @doc """
  Handler for fixture: cors - CORS preflight for PUT method
  """
  def handle_cors_cors_preflight_for_put_method(_request) do
    build_response(nil, 204, %{
      "Access-Control-Max-Age" => "3600",
      "Vary" => "Origin",
      "Access-Control-Allow-Headers" => "Content-Type, X-Custom-Header",
      "Access-Control-Allow-Origin" => "https://app.example.com",
      "Access-Control-Allow-Methods" => "GET, POST, PUT, PATCH, DELETE"
    })
  end

  @doc """
  Handler for fixture: cors - CORS preflight request
  """
  def handle_cors_cors_preflight_request(_request) do
    build_response(nil, 200, %{
      "Access-Control-Allow-Origin" => "https://example.com",
      "Access-Control-Allow-Methods" => "GET, POST, PUT, DELETE, OPTIONS",
      "Access-Control-Allow-Headers" => "Content-Type, X-Custom-Header",
      "Access-Control-Max-Age" => "600"
    })
  end

  @doc """
  Handler for fixture: cors - CORS regex pattern matching for origins
  """
  def handle_cors_cors_regex_pattern_matching_for_origins(_request) do
    build_response(%{"data" => "resource data"}, 200, %{
      "Access-Control-Allow-Origin" => "https://subdomain.example.com",
      "Vary" => "Origin"
    })
  end

  @doc """
  Handler for fixture: cors - CORS request blocked
  """
  def handle_cors_cors_request_blocked(_request) do
    build_response(
      %{"detail" => "CORS request from origin 'https://malicious-site.com' not allowed"},
      403,
      %{}
    )
  end

  @doc """
  Handler for fixture: cors - CORS safelisted headers without preflight
  """
  def handle_cors_cors_safelisted_headers_without_preflight(_request) do
    build_response(%{"message" => "Success"}, 200, %{
      "Access-Control-Allow-Origin" => "https://app.example.com",
      "Vary" => "Origin"
    })
  end

  @doc """
  Handler for fixture: cors - CORS wildcard origin
  """
  def handle_cors_cors_wildcard_origin(_request) do
    build_response(%{"data" => "public"}, 200, %{"Access-Control-Allow-Origin" => "*"})
  end

  @doc """
  Handler for fixture: cors - CORS with credentials
  """
  def handle_cors_cors_with_credentials(_request) do
    build_response(%{"username" => "john"}, 200, %{
      "Access-Control-Allow-Origin" => "https://app.example.com",
      "Access-Control-Allow-Credentials" => "true",
      "Vary" => "Origin"
    })
  end

  @doc """
  Handler for fixture: cors - Simple CORS request
  """
  def handle_cors_simple_cors_request(_request) do
    build_response(%{"items" => []}, 200, %{
      "Vary" => "Origin",
      "Access-Control-Allow-Origin" => "https://example.com"
    })
  end

  @doc """
  Handler for fixture: di - Async factory dependency - success
  """
  def handle_di_async_factory_dependency___success(_request) do
    build_response(%{"pool_status" => "connected", "max_size" => 10}, 200, %{})
  end

  @doc """
  Handler for fixture: di - Circular dependency detection - error
  """
  def handle_di_circular_dependency_detection___error(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/dependency-error",
        "title" => "Dependency Resolution Failed",
        "status" => 500,
        "detail" => "Circular dependency detected",
        "errors" => [
          %{
            "type" => "circular_dependency",
            "msg" => "Circular dependency detected in dependency graph",
            "cycle" => ["service_a", "service_b", "service_a"]
          }
        ]
      },
      500,
      %{}
    )
  end

  @doc """
  Handler for fixture: di - Dependency injection in lifecycle hooks - success
  """
  def handle_di_dependency_injection_in_lifecycle_hooks___success(_request) do
    build_response(%{"authenticated" => true, "logged" => true}, 200, %{
      "X-Log-Level" => "debug",
      "X-Auth-Mode" => "strict"
    })
  end

  @doc """
  Handler for fixture: di - Factory dependency - success
  """
  def handle_di_factory_dependency___success(_request) do
    build_response(%{"timestamp" => "<<present>>"}, 200, %{})
  end

  @doc """
  Handler for fixture: di - Missing dependency - error
  """
  def handle_di_missing_dependency___error(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/dependency-error",
        "title" => "Dependency Resolution Failed",
        "status" => 500,
        "detail" => "Required dependency not found",
        "errors" => [
          %{
            "type" => "missing_dependency",
            "msg" => "Dependency 'non_existent_service' is not registered",
            "dependency_key" => "non_existent_service"
          }
        ]
      },
      500,
      %{}
    )
  end

  @doc """
  Handler for fixture: di - Mixed singleton and per-request caching - success
  """
  def handle_di_mixed_singleton_and_per_request_caching___success(_request) do
    build_response(
      %{"app_name" => "MyApp", "pool_id" => "<<uuid>>", "context_id" => "<<uuid>>"},
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: di - Multiple dependencies with cleanup - success
  """
  def handle_di_multiple_dependencies_with_cleanup___success(_request) do
    build_response(%{"session_active" => true}, 200, %{})
  end

  @doc """
  Handler for fixture: di - Nested dependencies (3 levels) - success
  """
  def handle_di_nested_dependencies__3_levels____success(_request) do
    build_response(%{"auth_enabled" => true, "has_db" => true, "has_cache" => true}, 200, %{})
  end

  @doc """
  Handler for fixture: di - Node.js object destructuring injection - success
  """
  def handle_di_node_js_object_destructuring_injection___success(_request) do
    build_response(%{"db_name" => "PostgreSQL", "log_level" => "info"}, 200, %{})
  end

  @doc """
  Handler for fixture: di - Per-request dependency caching - success
  """
  def handle_di_per_request_dependency_caching___success(_request) do
    build_response(%{"first_id" => "<<uuid>>", "second_id" => "<<same_as:first_id>>"}, 200, %{})
  end

  @doc """
  Handler for fixture: di - Python parameter name-based injection - success
  """
  def handle_di_python_parameter_name_based_injection___success(_request) do
    build_response(%{"db_status" => "connected", "cache_status" => "ready"}, 200, %{})
  end

  @doc """
  Handler for fixture: di - Python type annotation-based injection - success
  """
  def handle_di_python_type_annotation_based_injection___success(_request) do
    build_response(%{"pool_type" => "PostgreSQL", "cache_type" => "Redis"}, 200, %{})
  end

  @doc """
  Handler for fixture: di - Resource cleanup after request - success
  """
  def handle_di_resource_cleanup_after_request___success(_request) do
    build_response(%{"session_id" => "<<uuid>>", "status" => "completed"}, 200, %{})
  end

  @doc """
  Handler for fixture: di - Route-level dependency override - success
  """
  def handle_di_route_level_dependency_override___success(_request) do
    build_response(%{"mode" => "test", "strict" => false}, 200, %{})
  end

  @doc """
  Handler for fixture: di - Ruby keyword argument injection - success
  """
  def handle_di_ruby_keyword_argument_injection___success(_request) do
    build_response(%{"adapter" => "postgresql", "user_id" => 42}, 200, %{})
  end

  @doc """
  Handler for fixture: di - Singleton dependency caching - success
  """
  def handle_di_singleton_dependency_caching___success(_request) do
    build_response(%{"counter_id" => "<<uuid>>", "count" => 1}, 200, %{})
  end

  @doc """
  Handler for fixture: di - Type mismatch in dependency resolution - error
  """
  def handle_di_type_mismatch_in_dependency_resolution___error(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/dependency-error",
        "title" => "Dependency Resolution Failed",
        "status" => 500,
        "detail" => "Dependency type mismatch",
        "errors" => [
          %{
            "type" => "type_mismatch",
            "msg" => "Dependency 'config' type mismatch: expected object, got string",
            "dependency_key" => "config",
            "expected_type" => "object",
            "actual_type" => "string"
          }
        ]
      },
      500,
      %{}
    )
  end

  @doc """
  Handler for fixture: di - Value dependency injection - success
  """
  def handle_di_value_dependency_injection___success(_request) do
    build_response(
      %{"app_name" => "SpikardApp", "version" => "1.0.0", "max_connections" => 100},
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: edge_cases - 11_utf8_query_parameter
  """
  def handle_edge_cases_11_utf8_query_parameter(_request) do
    build_response(%{"term" => "café"}, 200, %{})
  end

  @doc """
  Handler for fixture: edge_cases - 12_percent_encoded_special_chars
  """
  def handle_edge_cases_12_percent_encoded_special_chars(_request) do
    build_response(%{"term" => "hi there"}, 200, %{})
  end

  @doc """
  Handler for fixture: edge_cases - 13_empty_string_query_param_preserved
  """
  def handle_edge_cases_13_empty_string_query_param_preserved(_request) do
    build_response(%{"filter" => ""}, 200, %{})
  end

  @doc """
  Handler for fixture: edge_cases - 14_large_integer_boundary
  """
  def handle_edge_cases_14_large_integer_boundary(_request) do
    build_response(%{"id" => 9_007_199_254_740_991}, 200, %{})
  end

  @doc """
  Handler for fixture: edge_cases - 15_float_precision_preservation
  """
  def handle_edge_cases_15_float_precision_preservation(_request) do
    build_response(%{"value" => 3.141592653589793}, 201, %{})
  end

  @doc """
  Handler for fixture: edge_cases - 16_negative_zero_handling
  """
  def handle_edge_cases_16_negative_zero_handling(_request) do
    build_response(%{"offset" => 0}, 201, %{})
  end

  @doc """
  Handler for fixture: edge_cases - 17_extremely_long_string
  """
  def handle_edge_cases_17_extremely_long_string(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "string_too_long",
            "loc" => ["body", "content"],
            "msg" => "String should have at most 10000 characters",
            "input" =>
              "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "ctx" => %{"max_length" => 10000}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: edge_cases - 18_unicode_normalization
  """
  def handle_edge_cases_18_unicode_normalization(_request) do
    build_response(%{"name" => "café"}, 201, %{})
  end

  @doc """
  Handler for fixture: edge_cases - 19_emoji_in_strings
  """
  def handle_edge_cases_19_emoji_in_strings(_request) do
    build_response(%{"text" => "Hello 👋 World 🌍"}, 201, %{})
  end

  @doc """
  Handler for fixture: edge_cases - 20_null_byte_in_string
  """
  def handle_edge_cases_20_null_byte_in_string(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "string_pattern_mismatch",
            "loc" => ["body", "filename"],
            "msg" => "String should match pattern '^[^\\x00]+$'",
            "input" => "file .txt",
            "ctx" => %{"pattern" => "^[^\\x00]+$"}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: edge_cases - 21_scientific_notation_number
  """
  def handle_edge_cases_21_scientific_notation_number(_request) do
    build_response(%{"value" => 123_000}, 201, %{})
  end

  @doc """
  Handler for fixture: edge_cases - 22_leading_zeros_integer
  """
  def handle_edge_cases_22_leading_zeros_integer(_request) do
    build_response(%{"value" => 123}, 200, %{})
  end

  @doc """
  Handler for fixture: edge_cases - 23_deeply_nested_json_limit
  """
  def handle_edge_cases_23_deeply_nested_json_limit(_request) do
    build_response(%{"error" => "Request body exceeds maximum nesting depth of 32"}, 400, %{})
  end

  @doc """
  Handler for fixture: edge_cases - 24_array_with_holes
  """
  def handle_edge_cases_24_array_with_holes(_request) do
    build_response(
      %{"error" => "Failed to parse URL-encoded form data: missing index, expected: 1 got 2"},
      400,
      %{}
    )
  end

  @doc """
  Handler for fixture: edge_cases - Deeply nested structure (10+ levels)
  """
  def handle_edge_cases_deeply_nested_structure__10__levels_(_request) do
    build_response(
      %{
        "message" => "Processed deeply nested structure",
        "max_depth" => 10,
        "value_found" => "deep"
      },
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: edge_cases - Empty and null value handling
  """
  def handle_edge_cases_empty_and_null_value_handling(_request) do
    build_response(
      %{
        "explicit_null_is_null" => true,
        "empty_string_length" => 0,
        "empty_array_length" => 0,
        "empty_object_keys" => 0,
        "zero_is_falsy" => true,
        "false_is_false" => true
      },
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: edge_cases - Float precision and rounding
  """
  def handle_edge_cases_float_precision_and_rounding(_request) do
    build_response(
      %{
        "sum" => 0.30000000000000004,
        "precise_value" => 3.141592653589793,
        "very_small" => 1.0e-10,
        "very_large" => 1.7976931348623157e+308
      },
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: edge_cases - Large integer boundary values
  """
  def handle_edge_cases_large_integer_boundary_values(_request) do
    build_response(
      %{
        "max_safe_int" => 9_007_199_254_740_991,
        "large_int" => 9_223_372_036_854_775_807,
        "negative_large" => -9_223_372_036_854_775_808
      },
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: edge_cases - Special string values and escaping
  """
  def handle_edge_cases_special_string_values_and_escaping(_request) do
    build_response(
      %{
        "empty_string" => "",
        "whitespace" => "   ",
        "tabs_newlines" => "line1\n\tline2\r\nline3",
        "quotes" => "He said \"hello\" and 'goodbye'",
        "backslashes" => "C:\\\\Users\\\\Path",
        "unicode_escapes" => "Hello",
        "special_chars" => "!@#$%^&*()_+-=[]{}|;':\",./<>?"
      },
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: edge_cases - Unicode and emoji handling
  """
  def handle_edge_cases_unicode_and_emoji_handling(_request) do
    build_response(
      %{
        "id" => 1,
        "name" => "Coffee Shop ☕",
        "description" => "Best café in München 🇩🇪",
        "tags" => ["食べ物", "音楽", "💰"],
        "emoji_reactions" => "👍❤️😂🎉"
      },
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: headers - 30_bearer_token_format_valid
  """
  def handle_headers_30_bearer_token_format_valid(_request) do
    build_response(nil, 200, %{})
  end

  @doc """
  Handler for fixture: headers - 31_bearer_token_format_invalid
  """
  def handle_headers_31_bearer_token_format_invalid(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["headers", "authorization"],
            "msg" => "Invalid Bearer token format",
            "ctx" => %{
              "pattern" => "^Bearer [A-Za-z0-9-._~+/]+=*$",
              "value" => "Bearer invalid token with spaces"
            }
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: headers - 32_bearer_token_missing_prefix
  """
  def handle_headers_32_bearer_token_missing_prefix(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["headers", "authorization"],
            "msg" => "Invalid Bearer token format",
            "ctx" => %{
              "pattern" => "^Bearer [A-Za-z0-9-._~+/]+=*$",
              "value" => "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"
            }
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: headers - 33_api_key_header_valid
  """
  def handle_headers_33_api_key_header_valid(_request) do
    build_response(nil, 200, %{})
  end

  @doc """
  Handler for fixture: headers - 34_api_key_header_invalid
  """
  def handle_headers_34_api_key_header_invalid(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["headers", "x-api-key"],
            "msg" => "Invalid API key format",
            "ctx" => %{"pattern" => "^[a-f0-9]{32}$", "value" => "invalid-key"}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: headers - Accept header - JSON
  """
  def handle_headers_accept_header___json(_request) do
    build_response(%{"accept" => "application/json"}, 200, %{})
  end

  @doc """
  Handler for fixture: headers - Accept-Encoding header
  """
  def handle_headers_accept_encoding_header(_request) do
    build_response(%{"accept_encoding" => "gzip, deflate, br"}, 200, %{})
  end

  @doc """
  Handler for fixture: headers - Accept-Language header
  """
  def handle_headers_accept_language_header(_request) do
    build_response(%{"accept_language" => "en-US,en;q=0.9"}, 200, %{})
  end

  @doc """
  Handler for fixture: headers - Authorization header - missing
  """
  def handle_headers_authorization_header___missing(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "missing",
            "loc" => ["headers", "authorization"],
            "msg" => "Field required",
            "input" => nil
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: headers - Authorization header - success
  """
  def handle_headers_authorization_header___success(_request) do
    build_response(%{"scheme" => "Digest", "credentials" => "foobar"}, 200, %{})
  end

  @doc """
  Handler for fixture: headers - Authorization header - wrong scheme
  """
  def handle_headers_authorization_header___wrong_scheme(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "string_pattern_mismatch",
            "loc" => ["headers", "authorization"],
            "msg" => "String should match pattern '^Digest .+'",
            "input" => "Other invalidauthorization"
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: headers - Basic authentication - success
  """
  def handle_headers_basic_authentication___success(_request) do
    build_response(%{"username" => "username", "password" => "password"}, 200, %{})
  end

  @doc """
  Handler for fixture: headers - Bearer token authentication - missing
  """
  def handle_headers_bearer_token_authentication___missing(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "missing",
            "loc" => ["headers", "authorization"],
            "msg" => "Field required",
            "input" => nil
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: headers - Bearer token authentication - success
  """
  def handle_headers_bearer_token_authentication___success(_request) do
    build_response(%{"token" => "valid_token_123"}, 200, %{})
  end

  @doc """
  Handler for fixture: headers - Content-Type header - application/json
  """
  def handle_headers_content_type_header___application_json(_request) do
    build_response(%{"content_type" => "application/json"}, 200, %{})
  end

  @doc """
  Handler for fixture: headers - Header case insensitivity - access
  """
  def handle_headers_header_case_insensitivity___access(_request) do
    build_response(
      %{
        "content_type_lower" => "application/json",
        "content_type_upper" => "application/json",
        "content_type_mixed" => "application/json"
      },
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: headers - Header regex validation - fail
  """
  def handle_headers_header_regex_validation___fail(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "string_pattern_mismatch",
            "loc" => ["headers", "x-request-id"],
            "msg" => "String should match pattern '^[0-9]{3,}$'",
            "input" => "invalid-format",
            "ctx" => %{"pattern" => "^[0-9]{3,}$"}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: headers - Header regex validation - success
  """
  def handle_headers_header_regex_validation___success(_request) do
    build_response(%{"x_request_id" => "12345"}, 200, %{})
  end

  @doc """
  Handler for fixture: headers - Header validation - max_length constraint fail
  """
  def handle_headers_header_validation___max_length_constraint_fail(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "string_too_long",
            "loc" => ["headers", "x-session-id"],
            "msg" => "String should have at most 20 characters",
            "input" => "this_is_way_too_long_for_validation",
            "ctx" => %{"max_length" => 20}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: headers - Header validation - min_length constraint
  """
  def handle_headers_header_validation___min_length_constraint(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "string_too_short",
            "loc" => ["headers", "x-token"],
            "msg" => "String should have at least 3 characters",
            "input" => "ab",
            "ctx" => %{"min_length" => 3}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: headers - Header with underscore conversion - explicit
  """
  def handle_headers_header_with_underscore_conversion___explicit(_request) do
    build_response(%{"x_token" => "secret123"}, 200, %{})
  end

  @doc """
  Handler for fixture: headers - Host header
  """
  def handle_headers_host_header(_request) do
    build_response(%{"host" => "example.com:8080"}, 200, %{})
  end

  @doc """
  Handler for fixture: headers - Multiple custom headers
  """
  def handle_headers_multiple_custom_headers(_request) do
    build_response(
      %{
        "x_request_id" => "req-12345",
        "x_client_version" => "1.2.3",
        "x_trace_id" => "trace-abc"
      },
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: headers - Multiple header values - X-Token
  """
  def handle_headers_multiple_header_values___x_token(_request) do
    build_response(%{"X-Token values" => ["foo", "bar"]}, 200, %{})
  end

  @doc """
  Handler for fixture: headers - Optional header with None default - missing
  """
  def handle_headers_optional_header_with_none_default___missing(_request) do
    build_response(%{"strange_header" => nil}, 200, %{})
  end

  @doc """
  Handler for fixture: headers - Origin header
  """
  def handle_headers_origin_header(_request) do
    build_response(%{"origin" => "https://example.com"}, 200, %{})
  end

  @doc """
  Handler for fixture: headers - Referer header
  """
  def handle_headers_referer_header(_request) do
    build_response(%{"referer" => "https://example.com/page"}, 200, %{})
  end

  @doc """
  Handler for fixture: headers - User-Agent header - custom value
  """
  def handle_headers_user_agent_header___custom_value(_request) do
    build_response(%{"User-Agent" => "Mozilla/5.0 Custom Browser"}, 200, %{})
  end

  @doc """
  Handler for fixture: headers - User-Agent header - default value
  """
  def handle_headers_user_agent_header___default_value(_request) do
    build_response(%{"User-Agent" => "testclient"}, 200, %{})
  end

  @doc """
  Handler for fixture: headers - X-API-Key optional header - missing
  """
  def handle_headers_x_api_key_optional_header___missing(_request) do
    build_response(%{"msg" => "Hello World"}, 200, %{})
  end

  @doc """
  Handler for fixture: headers - X-API-Key optional header - success
  """
  def handle_headers_x_api_key_optional_header___success(_request) do
    build_response(%{"msg" => "Hello secret"}, 200, %{})
  end

  @doc """
  Handler for fixture: headers - X-API-Key required header - missing
  """
  def handle_headers_x_api_key_required_header___missing(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "missing",
            "loc" => ["headers", "x-api-key"],
            "msg" => "Field required",
            "input" => nil
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: headers - X-API-Key required header - success
  """
  def handle_headers_x_api_key_required_header___success(_request) do
    build_response(%{"username" => "secret"}, 200, %{})
  end

  @doc """
  Handler for fixture: http_methods - DELETE - Remove resource
  """
  def handle_http_methods_delete___remove_resource(_request) do
    build_response(%{}, 200, %{})
  end

  @doc """
  Handler for fixture: http_methods - DELETE - Resource not found
  """
  def handle_http_methods_delete___resource_not_found(_request) do
    build_response(%{}, 200, %{})
  end

  @doc """
  Handler for fixture: http_methods - DELETE - With response body
  """
  def handle_http_methods_delete___with_response_body(_request) do
    build_response(
      %{"id" => 1, "name" => "Deleted Item", "message" => "Item deleted successfully"},
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: http_methods - HEAD - Get metadata without body
  """
  def handle_http_methods_head___get_metadata_without_body(_request) do
    build_response(nil, 200, %{"Content-Length" => "85", "Content-Type" => "application/json"})
  end

  @doc """
  Handler for fixture: http_methods - OPTIONS - CORS preflight request
  """
  def handle_http_methods_options___cors_preflight_request(_request) do
    build_response(nil, 200, %{
      "Access-Control-Allow-Headers" => "Content-Type",
      "Access-Control-Allow-Origin" => "https://example.com",
      "Access-Control-Max-Age" => "86400",
      "Access-Control-Allow-Methods" => "GET, POST, PUT, DELETE, OPTIONS"
    })
  end

  @doc """
  Handler for fixture: http_methods - PATCH - Partial update
  """
  def handle_http_methods_patch___partial_update(_request) do
    build_response(
      %{"id" => 1, "name" => "Existing Item", "price" => 79.99, "in_stock" => true},
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: http_methods - PATCH - Update multiple fields
  """
  def handle_http_methods_patch___update_multiple_fields(_request) do
    build_response(
      %{"id" => 1, "name" => "Updated Name", "price" => 89.99, "in_stock" => false},
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: http_methods - PUT - Complete resource replacement
  """
  def handle_http_methods_put___complete_resource_replacement(_request) do
    build_response(
      %{
        "id" => 1,
        "name" => "Updated Item",
        "description" => "Completely replaced",
        "price" => 99.99,
        "in_stock" => true
      },
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: http_methods - PUT - Create resource if doesn't exist
  """
  def handle_http_methods_put___create_resource_if_doesn_t_exist(_request) do
    build_response(%{"id" => 999, "name" => "New Item", "price" => 49.99}, 200, %{})
  end

  @doc """
  Handler for fixture: http_methods - PUT - Idempotent operation
  """
  def handle_http_methods_put___idempotent_operation(_request) do
    build_response(%{"id" => 1, "name" => "Fixed Name", "price" => 50.0}, 200, %{})
  end

  @doc """
  Handler for fixture: http_methods - PUT - Missing required field
  """
  def handle_http_methods_put___missing_required_field(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "missing",
            "loc" => ["body", "price"],
            "msg" => "Field required",
            "input" => %{"id" => 1, "name" => "Item Name"}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: http_methods - PUT - Validation error
  """
  def handle_http_methods_put___validation_error(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "2 validation errors in request",
        "errors" => [
          %{
            "type" => "string_too_short",
            "loc" => ["body", "name"],
            "msg" => "String should have at least 3 characters",
            "input" => "X",
            "ctx" => %{"min_length" => 3}
          },
          %{
            "type" => "greater_than",
            "loc" => ["body", "price"],
            "msg" => "Input should be greater than 0",
            "input" => -10,
            "ctx" => %{"gt" => 0}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: json_bodies - 29_nested_object_validation_success
  """
  def handle_json_bodies_29_nested_object_validation_success(_request) do
    build_response(nil, 201, %{})
  end

  @doc """
  Handler for fixture: json_bodies - 30_nested_object_missing_field
  """
  def handle_json_bodies_30_nested_object_missing_field(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "missing",
            "loc" => ["body", "profile", "email"],
            "msg" => "Field required",
            "input" => %{"name" => "John Doe"}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: json_bodies - 31_nullable_property_null_value
  """
  def handle_json_bodies_31_nullable_property_null_value(_request) do
    build_response(nil, 201, %{})
  end

  @doc """
  Handler for fixture: json_bodies - 32_schema_ref_definitions
  """
  def handle_json_bodies_32_schema_ref_definitions(_request) do
    build_response(nil, 201, %{})
  end

  @doc """
  Handler for fixture: json_bodies - 33_allof_schema_composition
  """
  def handle_json_bodies_33_allof_schema_composition(_request) do
    build_response(nil, 201, %{})
  end

  @doc """
  Handler for fixture: json_bodies - 34_additional_properties_false
  """
  def handle_json_bodies_34_additional_properties_false(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["body", "extra_field"],
            "msg" => "Additional properties are not allowed",
            "ctx" => %{"additional_properties" => false, "unexpected_field" => "extra_field"},
            "input" => %{
              "name" => "John",
              "email" => "john@example.com",
              "extra_field" => "should fail"
            }
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: json_bodies - 35_oneof_schema_success
  """
  def handle_json_bodies_35_oneof_schema_success(_request) do
    build_response(nil, 201, %{})
  end

  @doc """
  Handler for fixture: json_bodies - 36_oneof_schema_multiple_match_failure
  """
  def handle_json_bodies_36_oneof_schema_multiple_match_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["body"],
            "msg" =>
              "{\"credit_card\":\"1234567812345678\",\"paypal_email\":\"user@example.com\"} is valid under more than one of the schemas listed in the 'oneOf' keyword",
            "input" => %{
              "credit_card" => "1234567812345678",
              "paypal_email" => "user@example.com"
            }
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: json_bodies - 37_oneof_schema_no_match_failure
  """
  def handle_json_bodies_37_oneof_schema_no_match_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["body"],
            "msg" =>
              "{\"bitcoin_address\":\"1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa\"} is not valid under any of the schemas listed in the 'oneOf' keyword",
            "input" => %{"bitcoin_address" => "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: json_bodies - 38_anyof_schema_success
  """
  def handle_json_bodies_38_anyof_schema_success(_request) do
    build_response(nil, 201, %{})
  end

  @doc """
  Handler for fixture: json_bodies - 39_anyof_schema_multiple_match_success
  """
  def handle_json_bodies_39_anyof_schema_multiple_match_success(_request) do
    build_response(nil, 201, %{})
  end

  @doc """
  Handler for fixture: json_bodies - 40_anyof_schema_failure
  """
  def handle_json_bodies_40_anyof_schema_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["body"],
            "msg" =>
              "{\"name\":\"John Doe\"} is not valid under any of the schemas listed in the 'anyOf' keyword",
            "input" => %{"name" => "John Doe"}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: json_bodies - 41_not_schema_success
  """
  def handle_json_bodies_41_not_schema_success(_request) do
    build_response(nil, 201, %{})
  end

  @doc """
  Handler for fixture: json_bodies - 42_not_schema_failure
  """
  def handle_json_bodies_42_not_schema_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["body", "username"],
            "msg" => "{\"enum\":[\"admin\",\"root\",\"system\"]} is not allowed for \"admin\"",
            "input" => "admin"
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: json_bodies - 43_const_validation_success
  """
  def handle_json_bodies_43_const_validation_success(_request) do
    build_response(nil, 201, %{})
  end

  @doc """
  Handler for fixture: json_bodies - 44_const_validation_failure
  """
  def handle_json_bodies_44_const_validation_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["body", "version"],
            "msg" => "\"1.0\" was expected",
            "input" => "2.0"
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: json_bodies - 45_minproperties_validation_success
  """
  def handle_json_bodies_45_minproperties_validation_success(_request) do
    build_response(nil, 201, %{})
  end

  @doc """
  Handler for fixture: json_bodies - 46_minproperties_validation_failure
  """
  def handle_json_bodies_46_minproperties_validation_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["body"],
            "msg" => "{\"host\":\"localhost\"} has less than 2 properties",
            "input" => %{"host" => "localhost"}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: json_bodies - 47_maxproperties_validation_failure
  """
  def handle_json_bodies_47_maxproperties_validation_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["body"],
            "msg" =>
              "{\"host\":\"localhost\",\"port\":8080,\"ssl\":true,\"debug\":false} has more than 3 properties",
            "input" => %{"host" => "localhost", "port" => 8080, "ssl" => true, "debug" => false}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: json_bodies - 48_dependencies_validation_success
  """
  def handle_json_bodies_48_dependencies_validation_success(_request) do
    build_response(nil, 201, %{})
  end

  @doc """
  Handler for fixture: json_bodies - 49_dependencies_validation_failure
  """
  def handle_json_bodies_49_dependencies_validation_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["body"],
            "msg" => "\"billing_address\" is a required property",
            "input" => %{"name" => "John Doe", "credit_card" => "1234567812345678"}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: json_bodies - 50_deep_nesting_4_levels
  """
  def handle_json_bodies_50_deep_nesting_4_levels(_request) do
    build_response(nil, 201, %{})
  end

  @doc """
  Handler for fixture: json_bodies - Array of objects - success
  """
  def handle_json_bodies_array_of_objects___success(_request) do
    build_response(
      %{
        "name" => "Product Bundle",
        "tags" => ["electronics", "gadget"],
        "images" => [
          %{"url" => "https://example.com/img1.jpg", "name" => "Front"},
          %{"url" => "https://example.com/img2.jpg", "name" => "Back"}
        ]
      },
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: json_bodies - Array of primitive values
  """
  def handle_json_bodies_array_of_primitive_values(_request) do
    build_response(
      %{
        "name" => "Product",
        "tags" => ["electronics", "gadget", "new"],
        "ratings" => [4.5, 4.8, 5.0, 4.2]
      },
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: json_bodies - Body with query parameters
  """
  def handle_json_bodies_body_with_query_parameters(_request) do
    build_response(%{"item" => %{"name" => "Item", "price" => 42.0}, "limit" => 10}, 200, %{})
  end

  @doc """
  Handler for fixture: json_bodies - Boolean field - success
  """
  def handle_json_bodies_boolean_field___success(_request) do
    build_response(%{"name" => "Item", "price" => 42.0, "in_stock" => true}, 200, %{})
  end

  @doc """
  Handler for fixture: json_bodies - Date field - success
  """
  def handle_json_bodies_date_field___success(_request) do
    build_response(%{"name" => "Conference", "event_date" => "2024-03-15"}, 200, %{})
  end

  @doc """
  Handler for fixture: json_bodies - Datetime field - success
  """
  def handle_json_bodies_datetime_field___success(_request) do
    build_response(%{"name" => "Meeting", "created_at" => "2024-03-15T10:30:00Z"}, 200, %{})
  end

  @doc """
  Handler for fixture: json_bodies - Deeply nested objects
  """
  def handle_json_bodies_deeply_nested_objects(_request) do
    build_response(
      %{
        "name" => "Product",
        "price" => 100.0,
        "seller" => %{
          "name" => "John Doe",
          "address" => %{
            "street" => "123 Main St",
            "city" => "Springfield",
            "country" => %{"name" => "USA", "code" => "US"}
          }
        }
      },
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: json_bodies - Empty JSON object
  """
  def handle_json_bodies_empty_json_object(_request) do
    build_response(%{"name" => nil, "description" => nil, "price" => nil, "tax" => nil}, 200, %{})
  end

  @doc """
  Handler for fixture: json_bodies - Empty array validation - fail
  """
  def handle_json_bodies_empty_array_validation___fail(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "too_short",
            "loc" => ["body", "tags"],
            "msg" => "List should have at least 1 item after validation",
            "input" => [],
            "ctx" => %{"min_length" => 1}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: json_bodies - Enum field - invalid value
  """
  def handle_json_bodies_enum_field___invalid_value(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "enum",
            "loc" => ["body", "category"],
            "msg" => "Input should be 'electronics', 'clothing' or 'books'",
            "input" => "furniture",
            "ctx" => %{"expected" => "'electronics', 'clothing' or 'books'"}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: json_bodies - Enum field - success
  """
  def handle_json_bodies_enum_field___success(_request) do
    build_response(%{"name" => "Item", "category" => "electronics"}, 200, %{})
  end

  @doc """
  Handler for fixture: json_bodies - Extra fields ignored (no additionalProperties)
  """
  def handle_json_bodies_extra_fields_ignored__no_additionalproperties_(_request) do
    build_response(%{"name" => "Item", "price" => 42.0}, 200, %{})
  end

  @doc """
  Handler for fixture: json_bodies - Field type validation - invalid type
  """
  def handle_json_bodies_field_type_validation___invalid_type(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "float_parsing",
            "loc" => ["body", "price"],
            "msg" => "Input should be a valid number, unable to parse string as a number",
            "input" => "not a number"
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: json_bodies - Nested object - success
  """
  def handle_json_bodies_nested_object___success(_request) do
    build_response(
      %{
        "name" => "Foo",
        "price" => 42.0,
        "image" => %{"url" => "https://example.com/image.jpg", "name" => "Product Image"}
      },
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: json_bodies - Null value for optional field
  """
  def handle_json_bodies_null_value_for_optional_field(_request) do
    build_response(
      %{"name" => "Item", "price" => 42.0, "description" => nil, "tax" => nil},
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: json_bodies - Numeric ge validation - fail
  """
  def handle_json_bodies_numeric_ge_validation___fail(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "greater_than_equal",
            "loc" => ["body", "price"],
            "msg" => "Input should be greater than or equal to 1",
            "input" => 0.5,
            "ctx" => %{"ge" => 1}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: json_bodies - Numeric le validation - success
  """
  def handle_json_bodies_numeric_le_validation___success(_request) do
    build_response(%{"name" => "Item", "price" => 100.0}, 200, %{})
  end

  @doc """
  Handler for fixture: json_bodies - Optional fields - omitted
  """
  def handle_json_bodies_optional_fields___omitted(_request) do
    build_response(
      %{"name" => "Foo", "price" => 35.4, "description" => nil, "tax" => nil},
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: json_bodies - PATCH partial update
  """
  def handle_json_bodies_patch_partial_update(_request) do
    build_response(
      %{"name" => "Original Item", "price" => 45.0, "description" => "Original description"},
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: json_bodies - Required field missing - validation error
  """
  def handle_json_bodies_required_field_missing___validation_error(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "missing",
            "loc" => ["body", "name"],
            "msg" => "Field required",
            "input" => %{"description" => "A very nice Item", "price" => 35.4}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: json_bodies - Simple JSON object - success
  """
  def handle_json_bodies_simple_json_object___success(_request) do
    build_response(
      %{"name" => "Foo", "description" => "A very nice Item", "price" => 35.4, "tax" => 3.2},
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: json_bodies - String max_length validation - fail
  """
  def handle_json_bodies_string_max_length_validation___fail(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "string_too_long",
            "loc" => ["body", "name"],
            "msg" => "String should have at most 50 characters",
            "input" => "This is a very long name that exceeds the maximum length",
            "ctx" => %{"max_length" => 50}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: json_bodies - String min_length validation - fail
  """
  def handle_json_bodies_string_min_length_validation___fail(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "string_too_short",
            "loc" => ["body", "name"],
            "msg" => "String should have at least 3 characters",
            "input" => "ab",
            "ctx" => %{"min_length" => 3}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: json_bodies - String pattern validation - fail
  """
  def handle_json_bodies_string_pattern_validation___fail(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "string_pattern_mismatch",
            "loc" => ["body", "sku"],
            "msg" => "String should match pattern '^[A-Z]{3}[0-9]{4}$'",
            "input" => "ABC-123",
            "ctx" => %{"pattern" => "^[A-Z]{3}[0-9]{4}$"}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: json_bodies - String pattern validation - success
  """
  def handle_json_bodies_string_pattern_validation___success(_request) do
    build_response(%{"name" => "Item", "sku" => "ABC1234"}, 200, %{})
  end

  @doc """
  Handler for fixture: json_bodies - UUID field - invalid format
  """
  def handle_json_bodies_uuid_field___invalid_format(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "uuid_parsing",
            "loc" => ["body", "item_id"],
            "msg" => "Input should be a valid UUID",
            "input" => "not-a-valid-uuid"
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: json_bodies - UUID field - success
  """
  def handle_json_bodies_uuid_field___success(_request) do
    build_response(
      %{"name" => "Item", "item_id" => "c892496f-b1fd-4b91-bdb8-b46f92df1716"},
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: lifecycle_hooks - Hook Execution Order
  """
  def handle_lifecycle_hooks_hook_execution_order(_request) do
    build_response(
      %{
        "message" => "Hooks executed in order",
        "execution_order" => ["first_hook", "second_hook", "third_hook"]
      },
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: lifecycle_hooks - Multiple Hooks - All Phases
  """
  def handle_lifecycle_hooks_multiple_hooks___all_phases(_request) do
    build_response(
      %{
        "message" => "Action completed successfully",
        "user_id" => "user-123",
        "action" => "update_profile",
        "request_id" => ".*"
      },
      200,
      %{
        "X-Frame-Options" => "DENY",
        "X-Response-Time" => ".*ms",
        "X-Request-ID" => ".*",
        "X-Content-Type-Options" => "nosniff"
      }
    )
  end

  @doc """
  Handler for fixture: lifecycle_hooks - onError - Error Logging
  """
  def handle_lifecycle_hooks_onerror___error_logging(_request) do
    build_response(
      %{
        "error" => "Internal Server Error",
        "message" => "An unexpected error occurred",
        "error_id" => ".*"
      },
      500,
      %{"Content-Type" => "application/json"}
    )
  end

  @doc """
  Handler for fixture: lifecycle_hooks - onRequest - Request Logging
  """
  def handle_lifecycle_hooks_onrequest___request_logging(_request) do
    build_response(
      %{
        "message" => "onRequest hooks executed",
        "request_logged" => true,
        "has_request_id" => true
      },
      200,
      %{"X-Request-ID" => ".*"}
    )
  end

  @doc """
  Handler for fixture: lifecycle_hooks - onResponse - Response Timing
  """
  def handle_lifecycle_hooks_onresponse___response_timing(_request) do
    build_response(%{"message" => "Response with timing info"}, 200, %{
      "X-Response-Time" => ".*ms"
    })
  end

  @doc """
  Handler for fixture: lifecycle_hooks - onResponse - Security Headers
  """
  def handle_lifecycle_hooks_onresponse___security_headers(_request) do
    build_response(%{"message" => "Response with security headers"}, 200, %{
      "X-XSS-Protection" => "1; mode=block",
      "X-Frame-Options" => "DENY",
      "Strict-Transport-Security" => "max-age=31536000; includeSubDomains",
      "X-Content-Type-Options" => "nosniff"
    })
  end

  @doc """
  Handler for fixture: lifecycle_hooks - preHandler - Authentication Failed (Short Circuit)
  """
  def handle_lifecycle_hooks_prehandler___authentication_failed__short_circuit_(_request) do
    build_response(
      %{"error" => "Unauthorized", "message" => "Invalid or expired authentication token"},
      401,
      %{}
    )
  end

  @doc """
  Handler for fixture: lifecycle_hooks - preHandler - Authentication Success
  """
  def handle_lifecycle_hooks_prehandler___authentication_success(_request) do
    build_response(
      %{"message" => "Access granted", "user_id" => "user-123", "authenticated" => true},
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: lifecycle_hooks - preHandler - Authorization Check
  """
  def handle_lifecycle_hooks_prehandler___authorization_check(_request) do
    build_response(
      %{"message" => "Admin access granted", "user_id" => "admin-456", "role" => "admin"},
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: lifecycle_hooks - preHandler - Authorization Forbidden (Short Circuit)
  """
  def handle_lifecycle_hooks_prehandler___authorization_forbidden__short_circuit_(_request) do
    build_response(
      %{"error" => "Forbidden", "message" => "Admin role required for this endpoint"},
      403,
      %{}
    )
  end

  @doc """
  Handler for fixture: lifecycle_hooks - preValidation - Rate Limit Exceeded (Short Circuit)
  """
  def handle_lifecycle_hooks_prevalidation___rate_limit_exceeded__short_circuit_(_request) do
    build_response(
      %{
        "error" => "Rate limit exceeded",
        "message" => "Too many requests, please try again later"
      },
      429,
      %{"Retry-After" => "60"}
    )
  end

  @doc """
  Handler for fixture: lifecycle_hooks - preValidation - Rate Limiting
  """
  def handle_lifecycle_hooks_prevalidation___rate_limiting(_request) do
    build_response(%{"message" => "Request accepted", "rate_limit_checked" => true}, 200, %{})
  end

  @doc """
  Handler for fixture: multipart - 17_file_magic_number_png_success
  """
  def handle_multipart_17_file_magic_number_png_success(_request) do
    build_response(nil, 201, %{})
  end

  @doc """
  Handler for fixture: multipart - 18_file_magic_number_jpeg_success
  """
  def handle_multipart_18_file_magic_number_jpeg_success(_request) do
    build_response(nil, 201, %{})
  end

  @doc """
  Handler for fixture: multipart - 19_file_mime_spoofing_png_as_jpeg
  """
  def handle_multipart_19_file_mime_spoofing_png_as_jpeg(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["files", "image"],
            "msg" =>
              "File type mismatch: MIME type is image/jpeg but magic numbers indicate image/png",
            "ctx" => %{
              "declared_mime" => "image/jpeg",
              "detected_type" => "image/png",
              "magic_bytes" => "89504e470d0a1a0a"
            }
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: multipart - 20_file_mime_spoofing_jpeg_as_png
  """
  def handle_multipart_20_file_mime_spoofing_jpeg_as_png(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["files", "image"],
            "msg" =>
              "File type mismatch: MIME type is image/png but magic numbers indicate image/jpeg",
            "ctx" => %{
              "declared_mime" => "image/png",
              "detected_type" => "image/jpeg",
              "magic_bytes" => "ffd8ffe0"
            }
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: multipart - 21_file_pdf_magic_number_success
  """
  def handle_multipart_21_file_pdf_magic_number_success(_request) do
    build_response(nil, 201, %{})
  end

  @doc """
  Handler for fixture: multipart - 22_file_empty_buffer
  """
  def handle_multipart_22_file_empty_buffer(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["files", "file"],
            "msg" => "File buffer is empty",
            "ctx" => %{"buffer_size" => 0}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: multipart - Content-Type validation - invalid type
  """
  def handle_multipart_content_type_validation___invalid_type(_request) do
    build_response(nil, 422, %{})
  end

  @doc """
  Handler for fixture: multipart - Empty file upload
  """
  def handle_multipart_empty_file_upload(_request) do
    build_response(%{"filename" => "empty.txt", "size" => 0}, 200, %{})
  end

  @doc """
  Handler for fixture: multipart - File list upload (array of files)
  """
  def handle_multipart_file_list_upload__array_of_files_(_request) do
    build_response(%{"filenames" => ["file1.txt", "file2.txt"], "total_size" => 35}, 200, %{})
  end

  @doc """
  Handler for fixture: multipart - File size validation - too large
  """
  def handle_multipart_file_size_validation___too_large(_request) do
    build_response(%{"detail" => "File too large. Maximum size is 1MB"}, 413, %{})
  end

  @doc """
  Handler for fixture: multipart - File upload with custom headers
  """
  def handle_multipart_file_upload_with_custom_headers(_request) do
    build_response(
      %{
        "test2" => %{
          "filename" => "test2.txt",
          "size" => 15,
          "content" => "<file2 content>",
          "content_type" => "text/plain",
          "headers" => [
            ["content-disposition", "form-data; name=\"test2\"; filename=\"test2.txt\""],
            ["content-type", "text/plain"],
            ["x-custom", "f2"]
          ]
        }
      },
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: multipart - File upload without filename
  """
  def handle_multipart_file_upload_without_filename(_request) do
    build_response(%{"test1" => "<file1 content>"}, 200, %{})
  end

  @doc """
  Handler for fixture: multipart - Form data without files
  """
  def handle_multipart_form_data_without_files(_request) do
    build_response(%{"some" => "data"}, 200, %{})
  end

  @doc """
  Handler for fixture: multipart - Image file upload
  """
  def handle_multipart_image_file_upload(_request) do
    build_response(
      %{"filename" => "photo.jpg", "content_type" => "image/jpeg", "size" => 22},
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: multipart - Mixed files and form data
  """
  def handle_multipart_mixed_files_and_form_data(_request) do
    build_response(
      %{
        "file" => %{
          "filename" => "upload.txt",
          "size" => 14,
          "content" => "file data here",
          "content_type" => "text/plain"
        },
        "username" => "testuser",
        "age" => "25",
        "active" => "true"
      },
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: multipart - Multiple file uploads
  """
  def handle_multipart_multiple_file_uploads(_request) do
    build_response(
      %{
        "test1" => %{
          "filename" => "test1.txt",
          "size" => 15,
          "content" => "<file1 content>",
          "content_type" => "text/plain"
        },
        "test2" => %{
          "filename" => "test2.txt",
          "size" => 15,
          "content" => "<file2 content>",
          "content_type" => "text/plain"
        }
      },
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: multipart - Multiple values for same field name
  """
  def handle_multipart_multiple_values_for_same_field_name(_request) do
    build_response(
      %{
        "files" => [
          %{
            "filename" => "file1.txt",
            "size" => 10,
            "content" => "first file",
            "content_type" => "text/plain"
          },
          %{
            "filename" => "file2.txt",
            "size" => 11,
            "content" => "second file",
            "content_type" => "text/plain"
          }
        ],
        "tags" => ["python", "rust", "web"]
      },
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: multipart - Optional file upload - missing
  """
  def handle_multipart_optional_file_upload___missing(_request) do
    build_response(%{"file" => nil}, 200, %{})
  end

  @doc """
  Handler for fixture: multipart - Optional file upload - provided
  """
  def handle_multipart_optional_file_upload___provided(_request) do
    build_response(
      %{"filename" => "optional.txt", "content_type" => "text/plain", "size" => 21},
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: multipart - PDF file upload
  """
  def handle_multipart_pdf_file_upload(_request) do
    build_response(
      %{"filename" => "report.pdf", "content_type" => "application/pdf", "size" => 16},
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: multipart - Required file upload - missing
  """
  def handle_multipart_required_file_upload___missing(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "missing",
            "loc" => ["body", "file"],
            "msg" => "Field required",
            "input" => []
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: multipart - Simple file upload
  """
  def handle_multipart_simple_file_upload(_request) do
    build_response(
      %{
        "test" => %{
          "filename" => "test.txt",
          "size" => 14,
          "content" => "<file content>",
          "content_type" => "text/plain"
        }
      },
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: path_params - 20_uuid_v3_path_param_success
  """
  def handle_path_params_20_uuid_v3_path_param_success(_request) do
    build_response(%{"id" => "e8b5a51d-11c8-3310-a6ab-367563f20686"}, 200, %{})
  end

  @doc """
  Handler for fixture: path_params - 21_uuid_v5_path_param_success
  """
  def handle_path_params_21_uuid_v5_path_param_success(_request) do
    build_response(%{"id" => "630eb68f-e0fa-5ecc-887a-7c7a62614681"}, 200, %{})
  end

  @doc """
  Handler for fixture: path_params - 24_date_format_path_param_success
  """
  def handle_path_params_24_date_format_path_param_success(_request) do
    build_response(%{"date" => "2025-10-30"}, 200, %{})
  end

  @doc """
  Handler for fixture: path_params - 25_date_format_invalid_failure
  """
  def handle_path_params_25_date_format_invalid_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["path", "date"],
            "msg" => "Invalid date format",
            "ctx" => %{"format" => "date", "value" => "2025-13-45"}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: path_params - 27_datetime_format_path_param_success
  """
  def handle_path_params_27_datetime_format_path_param_success(_request) do
    build_response(%{"timestamp" => "2025-10-30T14:30:00Z"}, 200, %{})
  end

  @doc """
  Handler for fixture: path_params - 28_duration_format_path_param_success
  """
  def handle_path_params_28_duration_format_path_param_success(_request) do
    build_response(%{"duration" => "P1DT2H30M"}, 200, %{})
  end

  @doc """
  Handler for fixture: path_params - 29_decimal_path_param_success
  """
  def handle_path_params_29_decimal_path_param_success(_request) do
    build_response(%{"amount" => "19.99"}, 200, %{})
  end

  @doc """
  Handler for fixture: path_params - 30_string_minlength_path_success
  """
  def handle_path_params_30_string_minlength_path_success(_request) do
    build_response(%{"username" => "alice"}, 200, %{})
  end

  @doc """
  Handler for fixture: path_params - 31_string_minlength_path_failure
  """
  def handle_path_params_31_string_minlength_path_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["path", "username"],
            "msg" => "String length must be at least 3",
            "ctx" => %{"min_length" => 3, "actual_length" => 2}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: path_params - 32_string_maxlength_path_failure
  """
  def handle_path_params_32_string_maxlength_path_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["path", "username"],
            "msg" => "String length must not exceed 20",
            "ctx" => %{"max_length" => 20, "actual_length" => 42}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: path_params - 33_string_pattern_path_success
  """
  def handle_path_params_33_string_pattern_path_success(_request) do
    build_response(%{"owner" => "spikard-labs", "repo" => "spikard-http"}, 200, %{})
  end

  @doc """
  Handler for fixture: path_params - 34_string_pattern_path_failure
  """
  def handle_path_params_34_string_pattern_path_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["path", "owner"],
            "msg" => "String does not match pattern",
            "ctx" => %{"pattern" => "^[a-zA-Z0-9-]+$", "value" => "invalid@owner"}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: path_params - 35_negative_integer_path_param
  """
  def handle_path_params_35_negative_integer_path_param(_request) do
    build_response(%{"value" => -100}, 200, %{})
  end

  @doc """
  Handler for fixture: path_params - Boolean path parameter - True
  """
  def handle_path_params_boolean_path_parameter___true(_request) do
    build_response(%{"item_id" => true}, 200, %{})
  end

  @doc """
  Handler for fixture: path_params - Boolean path parameter - numeric 1
  """
  def handle_path_params_boolean_path_parameter___numeric_1(_request) do
    build_response(%{"item_id" => true}, 200, %{})
  end

  @doc """
  Handler for fixture: path_params - Date path parameter - success
  """
  def handle_path_params_date_path_parameter___success(_request) do
    build_response(%{"date_param" => "2023-07-15"}, 200, %{})
  end

  @doc """
  Handler for fixture: path_params - Enum path parameter - invalid value
  """
  def handle_path_params_enum_path_parameter___invalid_value(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "enum",
            "loc" => ["path", "model_name"],
            "msg" => "Input should be 'alexnet', 'resnet' or 'lenet'",
            "input" => "foo",
            "ctx" => %{"expected" => "'alexnet', 'resnet' or 'lenet'"}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: path_params - Enum path parameter - success
  """
  def handle_path_params_enum_path_parameter___success(_request) do
    build_response(%{"model_name" => "alexnet"}, 200, %{})
  end

  @doc """
  Handler for fixture: path_params - Float path parameter - success
  """
  def handle_path_params_float_path_parameter___success(_request) do
    build_response(%{"item_id" => 42.5}, 200, %{})
  end

  @doc """
  Handler for fixture: path_params - Integer path parameter - invalid string
  """
  def handle_path_params_integer_path_parameter___invalid_string(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "int_parsing",
            "loc" => ["path", "item_id"],
            "msg" => "Input should be a valid integer, unable to parse string as an integer",
            "input" => "foobar"
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: path_params - Integer path parameter - success
  """
  def handle_path_params_integer_path_parameter___success(_request) do
    build_response(%{"item_id" => 42}, 200, %{})
  end

  @doc """
  Handler for fixture: path_params - Integer path parameter with combined lt and gt constraints - success
  """
  def handle_path_params_integer_path_parameter_with_combined_lt_and_gt_constraints___success(
        _request
      ) do
    build_response(%{"item_id" => 2}, 200, %{})
  end

  @doc """
  Handler for fixture: path_params - Integer path parameter with ge constraint - success
  """
  def handle_path_params_integer_path_parameter_with_ge_constraint___success(_request) do
    build_response(%{"item_id" => 3}, 200, %{})
  end

  @doc """
  Handler for fixture: path_params - Integer path parameter with gt constraint - failure
  """
  def handle_path_params_integer_path_parameter_with_gt_constraint___failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "greater_than",
            "loc" => ["path", "item_id"],
            "msg" => "Input should be greater than 3",
            "input" => 2,
            "ctx" => %{"gt" => 3}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: path_params - Integer path parameter with gt constraint - success
  """
  def handle_path_params_integer_path_parameter_with_gt_constraint___success(_request) do
    build_response(%{"item_id" => 42}, 200, %{})
  end

  @doc """
  Handler for fixture: path_params - Integer path parameter with le constraint - success
  """
  def handle_path_params_integer_path_parameter_with_le_constraint___success(_request) do
    build_response(%{"item_id" => 3}, 200, %{})
  end

  @doc """
  Handler for fixture: path_params - Integer path parameter with lt constraint - success
  """
  def handle_path_params_integer_path_parameter_with_lt_constraint___success(_request) do
    build_response(%{"item_id" => 2}, 200, %{})
  end

  @doc """
  Handler for fixture: path_params - Multiple path parameters - success
  """
  def handle_path_params_multiple_path_parameters___success(_request) do
    build_response(
      %{
        "version" => 1.0,
        "service_id" => 1,
        "user_id" => "abc",
        "order_id" => "c892496f-b1fd-4b91-bdb8-b46f92df1716"
      },
      200,
      %{}
    )
  end

  @doc """
  Handler for fixture: path_params - Path parameter type syntax - invalid UUID
  """
  def handle_path_params_path_parameter_type_syntax___invalid_uuid(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "uuid_parsing",
            "loc" => ["path", "id"],
            "msg" => "Input should be a valid UUID",
            "input" => "not-a-uuid"
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: path_params - Path parameter type syntax with override
  """
  def handle_path_params_path_parameter_type_syntax_with_override(_request) do
    build_response(%{"count" => "50"}, 200, %{})
  end

  @doc """
  Handler for fixture: path_params - Path parameter with type syntax - UUID
  """
  def handle_path_params_path_parameter_with_type_syntax___uuid(_request) do
    build_response(%{"id" => "550e8400-e29b-41d4-a716-446655440000"}, 200, %{})
  end

  @doc """
  Handler for fixture: path_params - Path parameter with type syntax - integer
  """
  def handle_path_params_path_parameter_with_type_syntax___integer(_request) do
    build_response(%{"user_id" => "42"}, 200, %{})
  end

  @doc """
  Handler for fixture: path_params - Path type parameter - file path
  """
  def handle_path_params_path_type_parameter___file_path(_request) do
    build_response(%{"file_path" => "home/johndoe/myfile.txt"}, 200, %{})
  end

  @doc """
  Handler for fixture: path_params - String path parameter - success
  """
  def handle_path_params_string_path_parameter___success(_request) do
    build_response(%{"item_id" => "foobar"}, 200, %{})
  end

  @doc """
  Handler for fixture: path_params - String path parameter with max_length - failure
  """
  def handle_path_params_string_path_parameter_with_max_length___failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "string_too_long",
            "loc" => ["path", "item_id"],
            "msg" => "String should have at most 3 characters",
            "input" => "foobar",
            "ctx" => %{"max_length" => 3}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: path_params - String path parameter with min_length - failure
  """
  def handle_path_params_string_path_parameter_with_min_length___failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "string_too_short",
            "loc" => ["path", "item_id"],
            "msg" => "String should have at least 3 characters",
            "input" => "fo",
            "ctx" => %{"min_length" => 3}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: path_params - UUID path parameter - success
  """
  def handle_path_params_uuid_path_parameter___success(_request) do
    build_response(%{"item_id" => "ec38df32-ceda-4cfa-9b4a-1aeb94ad551a"}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - 42_negative_integer_query_param
  """
  def handle_query_params_42_negative_integer_query_param(_request) do
    build_response(%{"offset" => -10}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - 43_scientific_notation_float
  """
  def handle_query_params_43_scientific_notation_float(_request) do
    build_response(%{"threshold" => 0.0015}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - 44_string_minlength_validation_success
  """
  def handle_query_params_44_string_minlength_validation_success(_request) do
    build_response(%{"term" => "foo"}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - 45_string_minlength_validation_failure
  """
  def handle_query_params_45_string_minlength_validation_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["query", "term"],
            "msg" => "String length must be at least 3",
            "ctx" => %{"min_length" => 3, "actual_length" => 2}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: query_params - 46_string_maxlength_validation_failure
  """
  def handle_query_params_46_string_maxlength_validation_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["query", "term"],
            "msg" => "String length must not exceed 10",
            "ctx" => %{"max_length" => 10, "actual_length" => 21}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: query_params - 47_pattern_validation_email_success
  """
  def handle_query_params_47_pattern_validation_email_success(_request) do
    build_response(%{"email" => "user@example.com"}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - 48_pattern_validation_email_failure
  """
  def handle_query_params_48_pattern_validation_email_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["query", "email"],
            "msg" => "String does not match pattern",
            "ctx" => %{
              "pattern" => "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$",
              "value" => "invalid-email"
            }
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: query_params - 49_integer_gt_constraint_success
  """
  def handle_query_params_49_integer_gt_constraint_success(_request) do
    build_response(%{"limit" => 5}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - 50_integer_gt_constraint_failure
  """
  def handle_query_params_50_integer_gt_constraint_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["query", "limit"],
            "msg" => "Value must be greater than 0",
            "ctx" => %{"exclusive_minimum" => 0, "value" => 0}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: query_params - 51_integer_ge_constraint_boundary
  """
  def handle_query_params_51_integer_ge_constraint_boundary(_request) do
    build_response(%{"offset" => 0}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - 52_integer_le_constraint_boundary
  """
  def handle_query_params_52_integer_le_constraint_boundary(_request) do
    build_response(%{"limit" => 100}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - 53_integer_le_constraint_failure
  """
  def handle_query_params_53_integer_le_constraint_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["query", "limit"],
            "msg" => "Value must not exceed 100",
            "ctx" => %{"maximum" => 100, "value" => 101}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: query_params - 54_array_minitems_constraint_success
  """
  def handle_query_params_54_array_minitems_constraint_success(_request) do
    build_response(%{"ids" => [1, 2, 3]}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - 55_array_minitems_constraint_failure
  """
  def handle_query_params_55_array_minitems_constraint_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["query", "ids"],
            "msg" => "Array must contain at least 2 items",
            "ctx" => %{"min_items" => 2, "actual_items" => 1}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: query_params - 56_array_maxitems_constraint_failure
  """
  def handle_query_params_56_array_maxitems_constraint_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["query", "tags"],
            "msg" => "Array must not contain more than 5 items",
            "ctx" => %{"max_items" => 5, "actual_items" => 6}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: query_params - 57_boolean_empty_string_coercion
  """
  def handle_query_params_57_boolean_empty_string_coercion(_request) do
    build_response(%{"active" => false}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - 58_format_email_success
  """
  def handle_query_params_58_format_email_success(_request) do
    build_response(%{"email" => "user@example.com"}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - 59_format_email_failure
  """
  def handle_query_params_59_format_email_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["query", "email"],
            "msg" => "Invalid email format",
            "ctx" => %{"format" => "email", "value" => "not-an-email"}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: query_params - 60_format_ipv4_success
  """
  def handle_query_params_60_format_ipv4_success(_request) do
    build_response(%{"ip" => "192.168.1.1"}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - 61_format_ipv4_failure
  """
  def handle_query_params_61_format_ipv4_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["query", "ip"],
            "msg" => "Invalid IPv4 address format",
            "ctx" => %{"format" => "ipv4", "value" => "999.999.999.999"}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: query_params - 62_format_ipv6_success
  """
  def handle_query_params_62_format_ipv6_success(_request) do
    build_response(%{"ip" => "2001:0db8:85a3:0000:0000:8a2e:0370:7334"}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - 63_format_uri_success
  """
  def handle_query_params_63_format_uri_success(_request) do
    build_response(%{"url" => "https://example.com/path?query=value"}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - 64_format_uri_failure
  """
  def handle_query_params_64_format_uri_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["query", "url"],
            "msg" => "Invalid URI format",
            "ctx" => %{"format" => "uri", "value" => "not a uri"}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: query_params - 65_format_hostname_success
  """
  def handle_query_params_65_format_hostname_success(_request) do
    build_response(%{"host" => "api.example.com"}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - 66_multipleof_constraint_success
  """
  def handle_query_params_66_multipleof_constraint_success(_request) do
    build_response(%{"quantity" => 15}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - 67_multipleof_constraint_failure
  """
  def handle_query_params_67_multipleof_constraint_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["query", "quantity"],
            "msg" => "Value must be a multiple of 5",
            "ctx" => %{"multiple_of" => 5, "value" => 17}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: query_params - 68_array_uniqueitems_success
  """
  def handle_query_params_68_array_uniqueitems_success(_request) do
    build_response(%{"ids" => [1, 2, 3, 4]}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - 69_array_uniqueitems_failure
  """
  def handle_query_params_69_array_uniqueitems_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["query", "ids"],
            "msg" => "Array items must be unique",
            "ctx" => %{"unique_items" => true, "duplicate_value" => 2, "duplicate_index" => 2}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: query_params - 70_array_separator_pipe
  """
  def handle_query_params_70_array_separator_pipe(_request) do
    build_response(%{"tags" => ["python", "rust", "typescript"]}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - 71_array_separator_semicolon
  """
  def handle_query_params_71_array_separator_semicolon(_request) do
    build_response(%{"colors" => ["red", "green", "blue"]}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - 72_array_separator_space
  """
  def handle_query_params_72_array_separator_space(_request) do
    build_response(%{"keywords" => ["rust", "web", "framework"]}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - Array query parameter - empty array
  """
  def handle_query_params_array_query_parameter___empty_array(_request) do
    build_response([], 200, %{})
  end

  @doc """
  Handler for fixture: query_params - Array query parameter - single value
  """
  def handle_query_params_array_query_parameter___single_value(_request) do
    build_response(["apple"], 200, %{})
  end

  @doc """
  Handler for fixture: query_params - Boolean query parameter - numeric 1
  """
  def handle_query_params_boolean_query_parameter___numeric_1(_request) do
    build_response(%{"flag" => true}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - Boolean query parameter - true
  """
  def handle_query_params_boolean_query_parameter___true(_request) do
    build_response(%{"flag" => true}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - Date query parameter - success
  """
  def handle_query_params_date_query_parameter___success(_request) do
    build_response(%{"event_date" => "2024-01-15"}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - Datetime query parameter - success
  """
  def handle_query_params_datetime_query_parameter___success(_request) do
    build_response(%{"timestamp" => "2024-01-15T10:30:00Z"}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - Enum query parameter - invalid value
  """
  def handle_query_params_enum_query_parameter___invalid_value(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "enum",
            "loc" => ["query", "model"],
            "msg" => "Input should be 'alexnet', 'resnet' or 'lenet'",
            "input" => "vgg16",
            "ctx" => %{"expected" => "'alexnet', 'resnet' or 'lenet'"}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: query_params - Enum query parameter - success
  """
  def handle_query_params_enum_query_parameter___success(_request) do
    build_response(%{"model" => "alexnet"}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - Float query param with ge constraint - success
  """
  def handle_query_params_float_query_param_with_ge_constraint___success(_request) do
    build_response(%{"price" => 0.01}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - Integer query param with ge constraint - boundary
  """
  def handle_query_params_integer_query_param_with_ge_constraint___boundary(_request) do
    build_response(%{"value" => 10}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - Integer query param with gt constraint - valid
  """
  def handle_query_params_integer_query_param_with_gt_constraint___valid(_request) do
    build_response(%{"value" => 1}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - Integer query param with le constraint - boundary
  """
  def handle_query_params_integer_query_param_with_le_constraint___boundary(_request) do
    build_response(%{"value" => 100}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - Integer query param with lt constraint - valid
  """
  def handle_query_params_integer_query_param_with_lt_constraint___valid(_request) do
    build_response(%{"value" => 49}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - Integer with default value - not provided
  """
  def handle_query_params_integer_with_default_value___not_provided(_request) do
    build_response("foo bar 10", 200, %{})
  end

  @doc """
  Handler for fixture: query_params - Integer with default value - override
  """
  def handle_query_params_integer_with_default_value___override(_request) do
    build_response("foo bar 50", 200, %{})
  end

  @doc """
  Handler for fixture: query_params - List of integers - multiple values
  """
  def handle_query_params_list_of_integers___multiple_values(_request) do
    build_response([1, 2], 200, %{})
  end

  @doc """
  Handler for fixture: query_params - List of strings - multiple values
  """
  def handle_query_params_list_of_strings___multiple_values(_request) do
    build_response(%{"q" => ["foo", "bar"]}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - List query parameter - required but missing
  """
  def handle_query_params_list_query_parameter___required_but_missing(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "missing",
            "loc" => ["query", "device_ids"],
            "msg" => "Field required",
            "input" => nil
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: query_params - List with default empty array - no values provided
  """
  def handle_query_params_list_with_default_empty_array___no_values_provided(_request) do
    build_response([], 200, %{})
  end

  @doc """
  Handler for fixture: query_params - Multiple query parameters with different types
  """
  def handle_query_params_multiple_query_parameters_with_different_types(_request) do
    build_response(%{"name" => "john", "age" => 30, "active" => true, "score" => 95.5}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - Optional integer query parameter - missing
  """
  def handle_query_params_optional_integer_query_parameter___missing(_request) do
    build_response("foo bar None", 200, %{})
  end

  @doc """
  Handler for fixture: query_params - Optional query parameter with default value
  """
  def handle_query_params_optional_query_parameter_with_default_value(_request) do
    build_response(%{"limit" => 10}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - Optional string query parameter - missing
  """
  def handle_query_params_optional_string_query_parameter___missing(_request) do
    build_response("foo bar None", 200, %{})
  end

  @doc """
  Handler for fixture: query_params - Optional string query parameter - provided
  """
  def handle_query_params_optional_string_query_parameter___provided(_request) do
    build_response("foo bar baz", 200, %{})
  end

  @doc """
  Handler for fixture: query_params - Query parameter with URL encoded space
  """
  def handle_query_params_query_parameter_with_url_encoded_space(_request) do
    build_response(%{"name" => "hello world"}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - Query parameter with URL encoded special characters
  """
  def handle_query_params_query_parameter_with_url_encoded_special_characters(_request) do
    build_response(%{"name" => "test&value=123"}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - Query parameter with special characters - URL encoding
  """
  def handle_query_params_query_parameter_with_special_characters___url_encoding(_request) do
    build_response(%{"email" => "x@test.com", "special" => "&@A.ac"}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - Required integer query parameter - float value
  """
  def handle_query_params_required_integer_query_parameter___float_value(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "int_parsing",
            "loc" => ["query", "query"],
            "msg" => "Input should be a valid integer, unable to parse string as an integer",
            "input" => 42.5
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: query_params - Required integer query parameter - invalid type
  """
  def handle_query_params_required_integer_query_parameter___invalid_type(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "int_parsing",
            "loc" => ["query", "query"],
            "msg" => "Input should be a valid integer, unable to parse string as an integer",
            "input" => "baz"
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: query_params - Required integer query parameter - missing
  """
  def handle_query_params_required_integer_query_parameter___missing(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "missing",
            "loc" => ["query", "query"],
            "msg" => "Field required",
            "input" => nil
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: query_params - Required integer query parameter - success
  """
  def handle_query_params_required_integer_query_parameter___success(_request) do
    build_response("foo bar 42", 200, %{})
  end

  @doc """
  Handler for fixture: query_params - Required string query parameter - missing
  """
  def handle_query_params_required_string_query_parameter___missing(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "missing",
            "loc" => ["query", "query"],
            "msg" => "Field required",
            "input" => nil
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: query_params - Required string query parameter - success
  """
  def handle_query_params_required_string_query_parameter___success(_request) do
    build_response("foo bar baz", 200, %{})
  end

  @doc """
  Handler for fixture: query_params - String query param with max_length constraint - fail
  """
  def handle_query_params_string_query_param_with_max_length_constraint___fail(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "string_too_long",
            "loc" => ["query", "name"],
            "msg" => "String should have at most 10 characters",
            "input" => "this_is_way_too_long",
            "ctx" => %{"max_length" => 10}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: query_params - String query param with min_length constraint - fail
  """
  def handle_query_params_string_query_param_with_min_length_constraint___fail(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "string_too_short",
            "loc" => ["query", "name"],
            "msg" => "String should have at least 3 characters",
            "input" => "ab",
            "ctx" => %{"min_length" => 3}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: query_params - String query param with regex pattern - fail
  """
  def handle_query_params_string_query_param_with_regex_pattern___fail(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "string_pattern_mismatch",
            "loc" => ["query", "code"],
            "msg" => "String should match pattern '^[0-9]{3,}$'",
            "input" => "abc123",
            "ctx" => %{"pattern" => "^[0-9]{3,}$"}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: query_params - String validation with regex - failure
  """
  def handle_query_params_string_validation_with_regex___failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "string_pattern_mismatch",
            "loc" => ["query", "item_query"],
            "msg" => "String should match pattern '^fixedquery$'",
            "input" => "nonregexquery",
            "ctx" => %{"pattern" => "^fixedquery$"}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: query_params - String validation with regex - success
  """
  def handle_query_params_string_validation_with_regex___success(_request) do
    build_response(%{"item_query" => "fixedquery"}, 200, %{})
  end

  @doc """
  Handler for fixture: query_params - UUID query parameter - invalid format
  """
  def handle_query_params_uuid_query_parameter___invalid_format(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "uuid_parsing",
            "loc" => ["query", "item_id"],
            "msg" => "Input should be a valid UUID",
            "input" => "not-a-uuid"
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: query_params - UUID query parameter - success
  """
  def handle_query_params_uuid_query_parameter___success(_request) do
    build_response(%{"item_id" => "c892496f-b1fd-4b91-bdb8-b46f92df1716"}, 200, %{})
  end

  @doc """
  Handler for fixture: rate_limit - Rate limit below threshold succeeds
  """
  def handle_rate_limit_rate_limit_below_threshold_succeeds(_request) do
    build_response(%{"status" => "ok", "request" => "under-limit"}, 200, %{})
  end

  @doc """
  Handler for fixture: rate_limit - Rate limit exceeded returns 429
  """
  def handle_rate_limit_rate_limit_exceeded_returns_429(_request) do
    build_response(nil, 429, %{})
  end

  @doc """
  Handler for fixture: request_id - Request ID header is preserved
  """
  def handle_request_id_request_id_header_is_preserved(_request) do
    build_response(%{"status" => "preserved", "echo" => "trace-123"}, 200, %{
      "x-request-id" => "trace-123"
    })
  end

  @doc """
  Handler for fixture: request_id - Request ID is generated when not provided
  """
  def handle_request_id_request_id_is_generated_when_not_provided(_request) do
    build_response(%{"status" => "generated"}, 200, %{"x-request-id" => "<<uuid>>"})
  end

  @doc """
  Handler for fixture: request_id - Request ID middleware can be disabled
  """
  def handle_request_id_request_id_middleware_can_be_disabled(_request) do
    build_response(%{"status" => "no-request-id"}, 200, %{"x-request-id" => "<<absent>>"})
  end

  @doc """
  Handler for fixture: request_timeout - Request completes before timeout
  """
  def handle_request_timeout_request_completes_before_timeout(_request) do
    build_response(%{"status" => "ok", "duration" => "fast"}, 200, %{})
  end

  @doc """
  Handler for fixture: request_timeout - Request exceeds timeout
  """
  def handle_request_timeout_request_exceeds_timeout(_request) do
    build_response(nil, 408, %{})
  end

  @doc """
  Handler for fixture: static_files - Static file server returns text file
  """
  def handle_static_files_static_file_server_returns_text_file(_request) do
    build_response("Hello from static storage", 200, %{
      "cache-control" => "public, max-age=60",
      "content-type" => "text/plain"
    })
  end

  @doc """
  Handler for fixture: static_files - Static server returns index.html for directory
  """
  def handle_static_files_static_server_returns_index_html_for_directory(_request) do
    build_response("<!doctype html><h1>Welcome</h1>", 200, %{"content-type" => "text/html"})
  end

  @doc """
  Handler for fixture: status_codes - 19_413_payload_too_large
  """
  def handle_status_codes_19_413_payload_too_large(_request) do
    build_response(
      %{
        "error" => "Payload Too Large",
        "message" => "Request body size exceeds maximum allowed size of 1024 bytes"
      },
      413,
      %{}
    )
  end

  @doc """
  Handler for fixture: status_codes - 200 OK - Success
  """
  def handle_status_codes_200_ok___success(_request) do
    build_response(%{"id" => 1, "name" => "Item 1"}, 200, %{})
  end

  @doc """
  Handler for fixture: status_codes - 201 Created - Resource created
  """
  def handle_status_codes_201_created___resource_created(_request) do
    build_response(%{"id" => 1, "name" => "New Item"}, 201, %{})
  end

  @doc """
  Handler for fixture: status_codes - 202 Accepted - Request accepted for processing
  """
  def handle_status_codes_202_accepted___request_accepted_for_processing(_request) do
    build_response(
      %{"message" => "Task accepted for processing", "task_id" => "abc123"},
      202,
      %{}
    )
  end

  @doc """
  Handler for fixture: status_codes - 204 No Content - Success with no body
  """
  def handle_status_codes_204_no_content___success_with_no_body(_request) do
    build_response(nil, 204, %{})
  end

  @doc """
  Handler for fixture: status_codes - 206 Partial Content
  """
  def handle_status_codes_206_partial_content(_request) do
    build_response("binary_data_1024_bytes", 206, %{
      "Content-Type" => "application/pdf",
      "Content-Range" => "bytes 0-21/5000",
      "Accept-Ranges" => "bytes"
    })
  end

  @doc """
  Handler for fixture: status_codes - 20_414_uri_too_long
  """
  def handle_status_codes_20_414_uri_too_long(_request) do
    build_response(%{}, 200, %{})
  end

  @doc """
  Handler for fixture: status_codes - 21_431_request_header_fields_too_large
  """
  def handle_status_codes_21_431_request_header_fields_too_large(_request) do
    build_response(
      %{
        "error" => "Request Header Fields Too Large",
        "message" => "Request headers exceed maximum allowed size of 8192 bytes"
      },
      431,
      %{}
    )
  end

  @doc """
  Handler for fixture: status_codes - 22_501_not_implemented
  """
  def handle_status_codes_22_501_not_implemented(_request) do
    build_response(nil, 405, %{})
  end

  @doc """
  Handler for fixture: status_codes - 23_503_service_unavailable
  """
  def handle_status_codes_23_503_service_unavailable(_request) do
    build_response(
      %{
        "error" => "Service Unavailable",
        "message" => "The service is temporarily unavailable. Please try again later."
      },
      503,
      %{"Retry-After" => "0"}
    )
  end

  @doc """
  Handler for fixture: status_codes - 301 Moved Permanently - Permanent redirect
  """
  def handle_status_codes_301_moved_permanently___permanent_redirect(_request) do
    build_response(nil, 301, %{"location" => "/new-path"})
  end

  @doc """
  Handler for fixture: status_codes - 302 Found - Temporary redirect
  """
  def handle_status_codes_302_found___temporary_redirect(_request) do
    build_response(nil, 302, %{"location" => "/target-path"})
  end

  @doc """
  Handler for fixture: status_codes - 304 Not Modified - Cached content valid
  """
  def handle_status_codes_304_not_modified___cached_content_valid(_request) do
    build_response(nil, 304, %{})
  end

  @doc """
  Handler for fixture: status_codes - 307 Temporary Redirect - Method preserved
  """
  def handle_status_codes_307_temporary_redirect___method_preserved(_request) do
    build_response(%{}, 307, %{"location" => "/target-post"})
  end

  @doc """
  Handler for fixture: status_codes - 400 Bad Request - Invalid request
  """
  def handle_status_codes_400_bad_request___invalid_request(_request) do
    build_response(%{"detail" => "Invalid request format"}, 400, %{})
  end

  @doc """
  Handler for fixture: status_codes - 401 Unauthorized - Missing authentication
  """
  def handle_status_codes_401_unauthorized___missing_authentication(_request) do
    build_response(%{"detail" => "Not authenticated"}, 401, %{"www-authenticate" => "Bearer"})
  end

  @doc """
  Handler for fixture: status_codes - 403 Forbidden - Insufficient permissions
  """
  def handle_status_codes_403_forbidden___insufficient_permissions(_request) do
    build_response(%{"detail" => "Not enough permissions"}, 403, %{})
  end

  @doc """
  Handler for fixture: status_codes - 404 Not Found - Resource not found
  """
  def handle_status_codes_404_not_found___resource_not_found(_request) do
    build_response(%{"detail" => "Item not found"}, 404, %{})
  end

  @doc """
  Handler for fixture: status_codes - 408 Request Timeout
  """
  def handle_status_codes_408_request_timeout(_request) do
    build_response(%{"detail" => "Request timeout"}, 408, %{"Connection" => "close"})
  end

  @doc """
  Handler for fixture: status_codes - 422 Unprocessable Entity - Validation error
  """
  def handle_status_codes_422_unprocessable_entity___validation_error(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "missing",
            "loc" => ["body", "name"],
            "msg" => "Field required",
            "input" => %{"price" => "not a number"}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: status_codes - 429 Too Many Requests
  """
  def handle_status_codes_429_too_many_requests(_request) do
    build_response(%{"detail" => "Rate limit exceeded. Try again in 60 seconds."}, 429, %{
      "X-RateLimit-Remaining" => "0",
      "X-RateLimit-Reset" => "1609459200",
      "X-RateLimit-Limit" => "100",
      "Retry-After" => "60"
    })
  end

  @doc """
  Handler for fixture: status_codes - 500 Internal Server Error - Server error
  """
  def handle_status_codes_500_internal_server_error___server_error(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/internal-server-error",
        "title" => "Internal Server Error",
        "status" => 500,
        "detail" => "Internal server error"
      },
      500,
      %{}
    )
  end

  @doc """
  Handler for fixture: status_codes - 503 Service Unavailable - Server overload
  """
  def handle_status_codes_503_service_unavailable___server_overload(_request) do
    build_response(%{"detail" => "Service temporarily unavailable"}, 503, %{"retry-after" => "0"})
  end

  @doc """
  Handler for fixture: streaming - Binary log download
  """
  def handle_streaming_binary_log_download(_request) do
    build_response("LOG:\\u0000\\u0001\\u0002\\u0003|TAIL|\\u0007\\n", 200, %{
      "content-type" => "application/octet-stream"
    })
  end

  @doc """
  Handler for fixture: streaming - Chunked CSV export
  """
  def handle_streaming_chunked_csv_export(_request) do
    build_response("id,name,value\\n1,Alice,42\\n2,Bob,7\\n", 200, %{"content-type" => "text/csv"})
  end

  @doc """
  Handler for fixture: streaming - Stream JSON lines
  """
  def handle_streaming_stream_json_lines(_request) do
    build_response(
      "{\"index\":0,\"payload\":\"alpha\"}\\n{\"index\":1,\"payload\":\"beta\"}\\n{\"index\":2,\"payload\":\"gamma\"}\\n",
      200,
      %{"content-type" => "application/x-ndjson"}
    )
  end

  @doc """
  Handler for fixture: url_encoded - 13_array_field_success
  """
  def handle_url_encoded_13_array_field_success(_request) do
    build_response(%{"tags" => ["python", "rust", "typescript"]}, 201, %{})
  end

  @doc """
  Handler for fixture: url_encoded - 14_nested_object_bracket_notation
  """
  def handle_url_encoded_14_nested_object_bracket_notation(_request) do
    build_response(
      %{"user" => %{"name" => "John Doe", "email" => "john@example.com", "age" => 30}},
      201,
      %{}
    )
  end

  @doc """
  Handler for fixture: url_encoded - 15_special_characters_field_names
  """
  def handle_url_encoded_15_special_characters_field_names(_request) do
    build_response(%{"user-name" => "JohnDoe", "contact.email" => "john@example.com"}, 201, %{})
  end

  @doc """
  Handler for fixture: url_encoded - 16_minlength_validation_failure
  """
  def handle_url_encoded_16_minlength_validation_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "string_too_short",
            "loc" => ["body", "username"],
            "msg" => "String should have at least 3 characters",
            "input" => "ab",
            "ctx" => %{"min_length" => 3}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: url_encoded - 17_pattern_validation_failure
  """
  def handle_url_encoded_17_pattern_validation_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "string_pattern_mismatch",
            "loc" => ["body", "account_id"],
            "msg" => "String should match pattern '^ACC-[0-9]{6}$'",
            "input" => "INVALID123",
            "ctx" => %{"pattern" => "^ACC-[0-9]{6}$"}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: url_encoded - 18_integer_minimum_validation_failure
  """
  def handle_url_encoded_18_integer_minimum_validation_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "greater_than_equal",
            "loc" => ["body", "quantity"],
            "msg" => "Input should be greater than or equal to 1",
            "input" => 0,
            "ctx" => %{"ge" => 1}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: url_encoded - 19_array_minitems_validation_failure
  """
  def handle_url_encoded_19_array_minitems_validation_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "too_short",
            "loc" => ["body", "tags"],
            "msg" => "List should have at least 2 item after validation",
            "input" => ["single"],
            "ctx" => %{"min_length" => 2}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: url_encoded - 20_format_email_validation_failure
  """
  def handle_url_encoded_20_format_email_validation_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "string_pattern_mismatch",
            "loc" => ["body", "email"],
            "msg" =>
              "String should match pattern '^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$'",
            "input" => "not-an-email",
            "ctx" => %{"pattern" => "^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$"}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: url_encoded - 21_integer_type_coercion_failure
  """
  def handle_url_encoded_21_integer_type_coercion_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "int_parsing",
            "loc" => ["body", "price"],
            "msg" => "Input should be a valid integer, unable to parse string as an integer",
            "input" => "not-a-number"
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: url_encoded - 22_additional_properties_strict_failure
  """
  def handle_url_encoded_22_additional_properties_strict_failure(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "validation_error",
            "loc" => ["body", "unknown_field"],
            "msg" => "Additional properties are not allowed",
            "input" => %{"theme" => "dark", "unknown_field" => "value"},
            "ctx" => %{"additional_properties" => false, "unexpected_field" => "unknown_field"}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: url_encoded - Boolean field conversion
  """
  def handle_url_encoded_boolean_field_conversion(_request) do
    build_response(%{"username" => "johndoe", "subscribe" => true}, 200, %{})
  end

  @doc """
  Handler for fixture: url_encoded - Empty string value
  """
  def handle_url_encoded_empty_string_value(_request) do
    build_response(%{"username" => "johndoe", "description" => ""}, 200, %{})
  end

  @doc """
  Handler for fixture: url_encoded - Multiple values for same field
  """
  def handle_url_encoded_multiple_values_for_same_field(_request) do
    build_response(%{"tags" => ["python", "fastapi", "web"]}, 200, %{})
  end

  @doc """
  Handler for fixture: url_encoded - Numeric field type conversion
  """
  def handle_url_encoded_numeric_field_type_conversion(_request) do
    build_response(%{"username" => "johndoe", "age" => 30}, 200, %{})
  end

  @doc """
  Handler for fixture: url_encoded - OAuth2 password grant flow
  """
  def handle_url_encoded_oauth2_password_grant_flow(_request) do
    build_response(%{"access_token" => "johndoe", "token_type" => "bearer"}, 200, %{})
  end

  @doc """
  Handler for fixture: url_encoded - Optional field missing - success
  """
  def handle_url_encoded_optional_field_missing___success(_request) do
    build_response(%{"username" => "johndoe", "email" => nil}, 200, %{})
  end

  @doc """
  Handler for fixture: url_encoded - Pattern validation - fail
  """
  def handle_url_encoded_pattern_validation___fail(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "string_pattern_mismatch",
            "loc" => ["body", "username"],
            "msg" => "String should match pattern '^[a-z0-9_]+$'",
            "input" => "john doe",
            "ctx" => %{"pattern" => "^[a-z0-9_]+$"}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: url_encoded - Required field missing - validation error
  """
  def handle_url_encoded_required_field_missing___validation_error(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "missing",
            "loc" => ["body", "username"],
            "msg" => "Field required",
            "input" => %{"password" => "secret"}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: url_encoded - Simple form submission - success
  """
  def handle_url_encoded_simple_form_submission___success(_request) do
    build_response(%{"username" => "johndoe"}, 200, %{})
  end

  @doc """
  Handler for fixture: url_encoded - Special characters encoding
  """
  def handle_url_encoded_special_characters_encoding(_request) do
    build_response(%{"name" => "John Doe", "description" => "Test & Development"}, 200, %{})
  end

  @doc """
  Handler for fixture: url_encoded - String max_length validation - fail
  """
  def handle_url_encoded_string_max_length_validation___fail(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "string_too_long",
            "loc" => ["body", "username"],
            "msg" => "String should have at most 20 characters",
            "input" => "this_is_a_very_long_username_that_exceeds_limit",
            "ctx" => %{"max_length" => 20}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: url_encoded - String min_length validation - fail
  """
  def handle_url_encoded_string_min_length_validation___fail(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "string_too_short",
            "loc" => ["body", "username"],
            "msg" => "String should have at least 3 characters",
            "input" => "ab",
            "ctx" => %{"min_length" => 3}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: validation_errors - 09_multiple_validation_errors
  """
  def handle_validation_errors_09_multiple_validation_errors(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "3 validation errors in request",
        "errors" => [
          %{
            "type" => "greater_than_equal",
            "loc" => ["body", "age"],
            "msg" => "Input should be greater than or equal to 18",
            "input" => 15,
            "ctx" => %{"ge" => 18}
          },
          %{
            "type" => "string_pattern_mismatch",
            "loc" => ["body", "email"],
            "msg" =>
              "String should match pattern '^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$'",
            "input" => "invalid-email",
            "ctx" => %{"pattern" => "^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$"}
          },
          %{
            "type" => "string_too_short",
            "loc" => ["body", "name"],
            "msg" => "String should have at least 3 characters",
            "input" => "ab",
            "ctx" => %{"min_length" => 3}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: validation_errors - 10_nested_error_path
  """
  def handle_validation_errors_10_nested_error_path(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "string_pattern_mismatch",
            "loc" => ["body", "profile", "contact", "email"],
            "msg" =>
              "String should match pattern '^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$'",
            "input" => "invalid",
            "ctx" => %{"pattern" => "^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\\.[a-zA-Z0-9-.]+$"}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: validation_errors - Array item validation error
  """
  def handle_validation_errors_array_item_validation_error(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "type_error",
            "loc" => ["body", "tags", "2"],
            "msg" => "Input should be a valid unknown",
            "input" => 123
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: validation_errors - Array max_items constraint violation
  """
  def handle_validation_errors_array_max_items_constraint_violation(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "too_long",
            "loc" => ["body", "tags"],
            "msg" => "List should have at most 10 items after validation",
            "input" => [
              "tag1",
              "tag2",
              "tag3",
              "tag4",
              "tag5",
              "tag6",
              "tag7",
              "tag8",
              "tag9",
              "tag10",
              "tag11"
            ],
            "ctx" => %{"max_length" => 10}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: validation_errors - Array min_items constraint violation
  """
  def handle_validation_errors_array_min_items_constraint_violation(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "too_short",
            "loc" => ["body", "tags"],
            "msg" => "List should have at least 1 item after validation",
            "input" => [],
            "ctx" => %{"min_length" => 1}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: validation_errors - Body field type error - string for float
  """
  def handle_validation_errors_body_field_type_error___string_for_float(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "float_parsing",
            "loc" => ["body", "price"],
            "msg" => "Input should be a valid number, unable to parse string as a number",
            "input" => "not_a_float"
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: validation_errors - Header validation error
  """
  def handle_validation_errors_header_validation_error(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "missing",
            "loc" => ["headers", "x-token"],
            "msg" => "Field required",
            "input" => nil
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: validation_errors - Invalid UUID format
  """
  def handle_validation_errors_invalid_uuid_format(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "uuid_parsing",
            "loc" => ["path", "item_id"],
            "msg" =>
              "Input should be a valid UUID, invalid character: expected an optional prefix of `urn:uuid:` followed by [0-9a-fA-F-], found `n` at 0",
            "input" => "not-a-uuid"
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: validation_errors - Invalid boolean value
  """
  def handle_validation_errors_invalid_boolean_value(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "bool_parsing",
            "loc" => ["query", "is_active"],
            "msg" => "Input should be a valid boolean, unable to interpret input",
            "input" => "maybe"
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: validation_errors - Invalid datetime format
  """
  def handle_validation_errors_invalid_datetime_format(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "datetime_parsing",
            "loc" => ["body", "created_at"],
            "msg" => "Input should be a valid datetime",
            "input" => "not-a-datetime"
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: validation_errors - Invalid enum value
  """
  def handle_validation_errors_invalid_enum_value(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "enum",
            "loc" => ["path", "model_name"],
            "msg" => "Input should be 'alexnet', 'resnet' or 'lenet'",
            "input" => "invalid_model",
            "ctx" => %{"expected" => "'alexnet', 'resnet' or 'lenet'"}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: validation_errors - Malformed JSON body
  """
  def handle_validation_errors_malformed_json_body(_request) do
    build_response(%{"detail" => "Invalid request format"}, 400, %{})
  end

  @doc """
  Handler for fixture: validation_errors - Missing required body field
  """
  def handle_validation_errors_missing_required_body_field(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "missing",
            "loc" => ["body", "price"],
            "msg" => "Field required",
            "input" => %{"name" => "Item"}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: validation_errors - Missing required query parameter
  """
  def handle_validation_errors_missing_required_query_parameter(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "missing",
            "loc" => ["query", "q"],
            "msg" => "Field required",
            "input" => nil
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: validation_errors - Multiple validation errors
  """
  def handle_validation_errors_multiple_validation_errors(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "3 validation errors in request",
        "errors" => [
          %{
            "type" => "string_too_short",
            "loc" => ["body", "name"],
            "msg" => "String should have at least 3 characters",
            "input" => "X",
            "ctx" => %{"min_length" => 3}
          },
          %{
            "type" => "greater_than",
            "loc" => ["body", "price"],
            "msg" => "Input should be greater than 0",
            "input" => -10,
            "ctx" => %{"gt" => 0}
          },
          %{
            "type" => "int_parsing",
            "loc" => ["body", "quantity"],
            "msg" => "Input should be a valid integer, unable to parse string as an integer",
            "input" => "not_a_number"
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: validation_errors - Nested object validation error
  """
  def handle_validation_errors_nested_object_validation_error(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "3 validation errors in request",
        "errors" => [
          %{
            "type" => "string_too_short",
            "loc" => ["body", "seller", "address", "city"],
            "msg" => "String should have at least 3 characters",
            "input" => "SF",
            "ctx" => %{"min_length" => 3}
          },
          %{
            "type" => "string_too_short",
            "loc" => ["body", "seller", "address", "zip_code"],
            "msg" => "String should have at least 5 characters",
            "input" => "123",
            "ctx" => %{"min_length" => 5}
          },
          %{
            "type" => "string_too_short",
            "loc" => ["body", "seller", "name"],
            "msg" => "String should have at least 3 characters",
            "input" => "Jo",
            "ctx" => %{"min_length" => 3}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: validation_errors - Numeric constraint violation - gt (greater than)
  """
  def handle_validation_errors_numeric_constraint_violation___gt__greater_than_(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "greater_than",
            "loc" => ["query", "price"],
            "msg" => "Input should be greater than 0",
            "input" => "0",
            "ctx" => %{"gt" => 0}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: validation_errors - Numeric constraint violation - le (less than or equal)
  """
  def handle_validation_errors_numeric_constraint_violation___le__less_than_or_equal_(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "less_than_equal",
            "loc" => ["query", "limit"],
            "msg" => "Input should be less than or equal to 100",
            "input" => "101",
            "ctx" => %{"le" => 100}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: validation_errors - Query param type error - string provided for int
  """
  def handle_validation_errors_query_param_type_error___string_provided_for_int(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "int_parsing",
            "loc" => ["query", "skip"],
            "msg" => "Input should be a valid integer, unable to parse string as an integer",
            "input" => "not_a_number"
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: validation_errors - String max_length constraint violation
  """
  def handle_validation_errors_string_max_length_constraint_violation(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "string_too_long",
            "loc" => ["query", "q"],
            "msg" => "String should have at most 50 characters",
            "input" =>
              "this_is_a_very_long_query_string_that_exceeds_maximum_length_limit_for_this_parameter",
            "ctx" => %{"max_length" => 50}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: validation_errors - String min_length constraint violation
  """
  def handle_validation_errors_string_min_length_constraint_violation(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "string_too_short",
            "loc" => ["query", "q"],
            "msg" => "String should have at least 3 characters",
            "input" => "ab",
            "ctx" => %{"min_length" => 3}
          }
        ]
      },
      422,
      %{}
    )
  end

  @doc """
  Handler for fixture: validation_errors - String regex pattern mismatch
  """
  def handle_validation_errors_string_regex_pattern_mismatch(_request) do
    build_response(
      %{
        "type" => "https://spikard.dev/errors/validation-error",
        "title" => "Request Validation Failed",
        "status" => 422,
        "detail" => "1 validation error in request",
        "errors" => [
          %{
            "type" => "string_pattern_mismatch",
            "loc" => ["query", "q"],
            "msg" => "String should match pattern '^[a-zA-Z0-9_-]+$'",
            "input" => "invalid!",
            "ctx" => %{"pattern" => "^[a-zA-Z0-9_-]+$"}
          }
        ]
      },
      422,
      %{}
    )
  end
end
