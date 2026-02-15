defmodule E2EElixirApp.AppFactories do
  @moduledoc """
  Generated app factory functions from test fixtures.

  Each factory function returns a route list for a single fixture.
  This avoids route conflicts when multiple fixtures test the same path
  (e.g., /query/basic with different query parameters).

  Usage:
      routes = E2EElixirApp.AppFactories.create_app_query_params_basic_success()
      {:ok, server} = Spikard.start(port: 59800, host: "127.0.0.1", routes: routes)
  """

  alias E2EElixirApp.Handlers

  @doc """
  App factory for fixture: auth - API key authentication - invalid key
  """
  def create_app_handle_auth_api_key_authentication___invalid_key() do
    routes = [
      %{
        method: :get,
        path: "/api/data",
        handler: &Handlers.handle_auth_api_key_authentication___invalid_key/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"X-API-Key" => %{"source" => "header", "type" => "string"}},
          "required" => ["X-API-Key"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: auth - API key authentication - missing header
  """
  def create_app_handle_auth_api_key_authentication___missing_header() do
    routes = [
      %{
        method: :get,
        path: "/api/data",
        handler: &Handlers.handle_auth_api_key_authentication___missing_header/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: auth - API key authentication - valid key
  """
  def create_app_handle_auth_api_key_authentication___valid_key() do
    routes = [
      %{
        method: :get,
        path: "/api/data",
        handler: &Handlers.handle_auth_api_key_authentication___valid_key/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"X-API-Key" => %{"source" => "header", "type" => "string"}},
          "required" => ["X-API-Key"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: auth - API key in query parameter
  """
  def create_app_handle_auth_api_key_in_query_parameter() do
    routes = [
      %{
        method: :get,
        path: "/api/data",
        handler: &Handlers.handle_auth_api_key_in_query_parameter/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: auth - API key rotation - old key still valid
  """
  def create_app_handle_auth_api_key_rotation___old_key_still_valid() do
    routes = [
      %{
        method: :get,
        path: "/api/data",
        handler: &Handlers.handle_auth_api_key_rotation___old_key_still_valid/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"X-API-Key" => %{"source" => "header", "type" => "string"}},
          "required" => ["X-API-Key"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: auth - API key with custom header name
  """
  def create_app_handle_auth_api_key_with_custom_header_name() do
    routes = [
      %{
        method: :get,
        path: "/api/data",
        handler: &Handlers.handle_auth_api_key_with_custom_header_name/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"X-API-Token" => %{"source" => "header", "type" => "string"}},
          "required" => ["X-API-Token"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: auth - Bearer token without prefix
  """
  def create_app_handle_auth_bearer_token_without_prefix() do
    routes = [
      %{
        method: :get,
        path: "/api/protected",
        handler: &Handlers.handle_auth_bearer_token_without_prefix/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"Authorization" => %{"source" => "header", "type" => "string"}},
          "required" => ["Authorization"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: auth - JWT authentication - expired token
  """
  def create_app_handle_auth_jwt_authentication___expired_token() do
    routes = [
      %{
        method: :get,
        path: "/protected/user",
        handler: &Handlers.handle_auth_jwt_authentication___expired_token/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"Authorization" => %{"source" => "header", "type" => "string"}},
          "required" => ["Authorization"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: auth - JWT authentication - invalid audience
  """
  def create_app_handle_auth_jwt_authentication___invalid_audience() do
    routes = [
      %{
        method: :get,
        path: "/protected/user",
        handler: &Handlers.handle_auth_jwt_authentication___invalid_audience/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"Authorization" => %{"source" => "header", "type" => "string"}},
          "required" => ["Authorization"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: auth - JWT authentication - invalid signature
  """
  def create_app_handle_auth_jwt_authentication___invalid_signature() do
    routes = [
      %{
        method: :get,
        path: "/protected/user",
        handler: &Handlers.handle_auth_jwt_authentication___invalid_signature/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"Authorization" => %{"source" => "header", "type" => "string"}},
          "required" => ["Authorization"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: auth - JWT authentication - missing Authorization header
  """
  def create_app_handle_auth_jwt_authentication___missing_authorization_header() do
    routes = [
      %{
        method: :get,
        path: "/protected/user",
        handler: &Handlers.handle_auth_jwt_authentication___missing_authorization_header/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: auth - JWT authentication - valid token
  """
  def create_app_handle_auth_jwt_authentication___valid_token() do
    routes = [
      %{
        method: :get,
        path: "/protected/user",
        handler: &Handlers.handle_auth_jwt_authentication___valid_token/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"Authorization" => %{"source" => "header", "type" => "string"}},
          "required" => ["Authorization"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: auth - JWT invalid issuer
  """
  def create_app_handle_auth_jwt_invalid_issuer() do
    routes = [
      %{
        method: :get,
        path: "/api/protected",
        handler: &Handlers.handle_auth_jwt_invalid_issuer/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"Authorization" => %{"source" => "header", "type" => "string"}},
          "required" => ["Authorization"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: auth - JWT malformed token format
  """
  def create_app_handle_auth_jwt_malformed_token_format() do
    routes = [
      %{
        method: :get,
        path: "/api/protected",
        handler: &Handlers.handle_auth_jwt_malformed_token_format/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"Authorization" => %{"source" => "header", "type" => "string"}},
          "required" => ["Authorization"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: auth - JWT missing required custom claims
  """
  def create_app_handle_auth_jwt_missing_required_custom_claims() do
    routes = [
      %{
        method: :get,
        path: "/api/admin",
        handler: &Handlers.handle_auth_jwt_missing_required_custom_claims/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"Authorization" => %{"source" => "header", "type" => "string"}},
          "required" => ["Authorization"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: auth - JWT not before claim in future
  """
  def create_app_handle_auth_jwt_not_before_claim_in_future() do
    routes = [
      %{
        method: :get,
        path: "/api/protected",
        handler: &Handlers.handle_auth_jwt_not_before_claim_in_future/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"Authorization" => %{"source" => "header", "type" => "string"}},
          "required" => ["Authorization"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: auth - JWT with multiple audiences
  """
  def create_app_handle_auth_jwt_with_multiple_audiences() do
    routes = [
      %{
        method: :get,
        path: "/api/protected",
        handler: &Handlers.handle_auth_jwt_with_multiple_audiences/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"Authorization" => %{"source" => "header", "type" => "string"}},
          "required" => ["Authorization"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: auth - Multiple authentication schemes - JWT precedence
  """
  def create_app_handle_auth_multiple_authentication_schemes___jwt_precedence() do
    routes = [
      %{
        method: :get,
        path: "/api/data",
        handler: &Handlers.handle_auth_multiple_authentication_schemes___jwt_precedence/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "Authorization" => %{"source" => "header", "type" => "string"},
            "X-API-Key" => %{"source" => "header", "type" => "string"}
          },
          "required" => ["Authorization", "X-API-Key"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: background - Background event logging
  """
  def create_app_handle_background_background_event_logging() do
    routes = [
      %{
        method: :post,
        path: "/background/events",
        handler: &Handlers.handle_background_background_event_logging/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"event" => %{"type" => "string"}},
          "required" => ["event"],
          "additionalProperties" => false
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: background - Background event logging - second payload
  """
  def create_app_handle_background_background_event_logging___second_payload() do
    routes = [
      %{
        method: :post,
        path: "/background/events",
        handler: &Handlers.handle_background_background_event_logging___second_payload/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"event" => %{"type" => "string"}},
          "required" => ["event"],
          "additionalProperties" => false
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: body_limits - Body over limit returns 413
  """
  def create_app_handle_body_limits_body_over_limit_returns_413() do
    routes = [
      %{
        method: :post,
        path: "/body-limit/over",
        handler: &Handlers.handle_body_limits_body_over_limit_returns_413/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"note" => %{"type" => "string"}},
          "required" => ["note"],
          "additionalProperties" => false
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: body_limits - Body under limit succeeds
  """
  def create_app_handle_body_limits_body_under_limit_succeeds() do
    routes = [
      %{
        method: :post,
        path: "/body-limit/under",
        handler: &Handlers.handle_body_limits_body_under_limit_succeeds/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"note" => %{"type" => "string"}},
          "required" => ["note"],
          "additionalProperties" => false
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: compression - Compression - gzip applied
  """
  def create_app_handle_compression_compression___gzip_applied() do
    routes = [
      %{
        method: :get,
        path: "/compression/gzip",
        handler: &Handlers.handle_compression_compression___gzip_applied/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: compression - Compression - payload below min_size is not compressed
  """
  def create_app_handle_compression_compression___payload_below_min_size_is_not_compressed() do
    routes = [
      %{
        method: :get,
        path: "/compression/skip",
        handler:
          &Handlers.handle_compression_compression___payload_below_min_size_is_not_compressed/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: content_types - 13_json_with_charset_utf16
  """
  def create_app_handle_content_types_13_json_with_charset_utf16() do
    routes = [
      %{
        method: :post,
        path: "/data",
        handler: &Handlers.handle_content_types_13_json_with_charset_utf16/1,
        request_schema: %{"type" => "object", "properties" => %{"value" => %{"type" => "string"}}}
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: content_types - 14_content_type_case_insensitive
  """
  def create_app_handle_content_types_14_content_type_case_insensitive() do
    routes = [
      %{
        method: :post,
        path: "/data",
        handler: &Handlers.handle_content_types_14_content_type_case_insensitive/1,
        request_schema: %{
          "type" => "object",
          "required" => ["name"],
          "properties" => %{"name" => %{"type" => "string"}}
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: content_types - 15_multipart_boundary_required
  """
  def create_app_handle_content_types_15_multipart_boundary_required() do
    routes = [
      %{
        method: :post,
        path: "/upload",
        handler: &Handlers.handle_content_types_15_multipart_boundary_required/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: content_types - 16_text_plain_not_accepted
  """
  def create_app_handle_content_types_16_text_plain_not_accepted() do
    routes = [
      %{
        method: :post,
        path: "/data",
        handler: &Handlers.handle_content_types_16_text_plain_not_accepted/1,
        request_schema: %{
          "type" => "object",
          "required" => ["data"],
          "properties" => %{"data" => %{"type" => "string"}}
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: content_types - 17_vendor_json_accepted
  """
  def create_app_handle_content_types_17_vendor_json_accepted() do
    routes = [
      %{
        method: :post,
        path: "/api/v1/resource",
        handler: &Handlers.handle_content_types_17_vendor_json_accepted/1,
        request_schema: %{
          "type" => "object",
          "required" => ["data"],
          "properties" => %{"data" => %{"type" => "string"}}
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: content_types - 18_content_type_with_multiple_params
  """
  def create_app_handle_content_types_18_content_type_with_multiple_params() do
    routes = [
      %{
        method: :post,
        path: "/data",
        handler: &Handlers.handle_content_types_18_content_type_with_multiple_params/1,
        request_schema: %{"type" => "object", "properties" => %{"value" => %{"type" => "string"}}}
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: content_types - 19_missing_content_type_default_json
  """
  def create_app_handle_content_types_19_missing_content_type_default_json() do
    routes = [
      %{
        method: :post,
        path: "/data",
        handler: &Handlers.handle_content_types_19_missing_content_type_default_json/1,
        request_schema: %{
          "type" => "object",
          "required" => ["name"],
          "properties" => %{"name" => %{"type" => "string"}}
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: content_types - 20_content_length_mismatch
  """
  def create_app_handle_content_types_20_content_length_mismatch() do
    routes = [
      %{
        method: :post,
        path: "/data",
        handler: &Handlers.handle_content_types_20_content_length_mismatch/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"Content-Length" => %{"source" => "header", "type" => "string"}},
          "required" => []
        },
        request_schema: %{"type" => "object", "properties" => %{"value" => %{"type" => "string"}}}
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: content_types - 415 Unsupported Media Type
  """
  def create_app_handle_content_types_415_unsupported_media_type() do
    routes = [
      %{
        method: :post,
        path: "/items/",
        handler: &Handlers.handle_content_types_415_unsupported_media_type/1,
        request_schema: %{"type" => "string"}
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: content_types - Binary response - application/octet-stream
  """
  def create_app_handle_content_types_binary_response___application_octet_stream() do
    routes = [
      %{
        method: :get,
        path: "/download/file.bin",
        handler: &Handlers.handle_content_types_binary_response___application_octet_stream/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: content_types - CSV response - text/csv
  """
  def create_app_handle_content_types_csv_response___text_csv() do
    routes = [
      %{
        method: :get,
        path: "/export/data.csv",
        handler: &Handlers.handle_content_types_csv_response___text_csv/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: content_types - Content negotiation - Accept header
  """
  def create_app_handle_content_types_content_negotiation___accept_header() do
    routes = [
      %{
        method: :get,
        path: "/accept-test/{id}",
        handler: &Handlers.handle_content_types_content_negotiation___accept_header/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"id" => %{"source" => "path", "type" => "string"}},
          "required" => ["id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: content_types - HTML response - text/html
  """
  def create_app_handle_content_types_html_response___text_html() do
    routes = [
      %{
        method: :get,
        path: "/html",
        handler: &Handlers.handle_content_types_html_response___text_html/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: content_types - JPEG image response - image/jpeg
  """
  def create_app_handle_content_types_jpeg_image_response___image_jpeg() do
    routes = [
      %{
        method: :get,
        path: "/images/photo.jpg",
        handler: &Handlers.handle_content_types_jpeg_image_response___image_jpeg/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: content_types - JSON response - application/json
  """
  def create_app_handle_content_types_json_response___application_json() do
    routes = [
      %{
        method: :get,
        path: "/items/json",
        handler: &Handlers.handle_content_types_json_response___application_json/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: content_types - JSON with UTF-8 charset
  """
  def create_app_handle_content_types_json_with_utf_8_charset() do
    routes = [
      %{
        method: :get,
        path: "/items/unicode",
        handler: &Handlers.handle_content_types_json_with_utf_8_charset/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: content_types - PDF response - application/pdf
  """
  def create_app_handle_content_types_pdf_response___application_pdf() do
    routes = [
      %{
        method: :get,
        path: "/download/document.pdf",
        handler: &Handlers.handle_content_types_pdf_response___application_pdf/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: content_types - PNG image response - image/png
  """
  def create_app_handle_content_types_png_image_response___image_png() do
    routes = [
      %{
        method: :get,
        path: "/images/logo.png",
        handler: &Handlers.handle_content_types_png_image_response___image_png/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: content_types - Plain text response - text/plain
  """
  def create_app_handle_content_types_plain_text_response___text_plain() do
    routes = [
      %{
        method: :get,
        path: "/text",
        handler: &Handlers.handle_content_types_plain_text_response___text_plain/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: content_types - XML response - application/xml
  """
  def create_app_handle_content_types_xml_response___application_xml() do
    routes = [
      %{
        method: :get,
        path: "/xml",
        handler: &Handlers.handle_content_types_xml_response___application_xml/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cookies - 24_cookie_samesite_strict
  """
  def create_app_handle_cookies_24_cookie_samesite_strict() do
    routes = [
      %{
        method: :get,
        path: "/secure",
        handler: &Handlers.handle_cookies_24_cookie_samesite_strict/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"session_id" => %{"source" => "cookie", "type" => "string"}},
          "required" => ["session_id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cookies - 25_cookie_samesite_lax
  """
  def create_app_handle_cookies_25_cookie_samesite_lax() do
    routes = [
      %{
        method: :get,
        path: "/data",
        handler: &Handlers.handle_cookies_25_cookie_samesite_lax/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"tracking" => %{"source" => "cookie", "type" => "string"}},
          "required" => ["tracking"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cookies - 26_cookie_secure_flag
  """
  def create_app_handle_cookies_26_cookie_secure_flag() do
    routes = [
      %{
        method: :get,
        path: "/secure",
        handler: &Handlers.handle_cookies_26_cookie_secure_flag/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"auth_token" => %{"source" => "cookie", "type" => "string"}},
          "required" => ["auth_token"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cookies - 27_cookie_httponly_flag
  """
  def create_app_handle_cookies_27_cookie_httponly_flag() do
    routes = [
      %{
        method: :get,
        path: "/secure",
        handler: &Handlers.handle_cookies_27_cookie_httponly_flag/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"session" => %{"source" => "cookie", "type" => "string"}},
          "required" => ["session"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cookies - APIKey cookie authentication - missing
  """
  def create_app_handle_cookies_apikey_cookie_authentication___missing() do
    routes = [
      %{
        method: :get,
        path: "/users/me/auth",
        handler: &Handlers.handle_cookies_apikey_cookie_authentication___missing/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"key" => %{"source" => "cookie", "type" => "string"}},
          "required" => ["key"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cookies - APIKey cookie authentication - success
  """
  def create_app_handle_cookies_apikey_cookie_authentication___success() do
    routes = [
      %{
        method: :get,
        path: "/users/me",
        handler: &Handlers.handle_cookies_apikey_cookie_authentication___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"key" => %{"source" => "cookie", "type" => "string"}},
          "required" => []
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cookies - Cookie regex pattern validation - fail
  """
  def create_app_handle_cookies_cookie_regex_pattern_validation___fail() do
    routes = [
      %{
        method: :get,
        path: "/cookies/pattern",
        handler: &Handlers.handle_cookies_cookie_regex_pattern_validation___fail/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "tracking_id" => %{
              "source" => "cookie",
              "type" => "string",
              "pattern" => "^[A-Z0-9]{8}$"
            }
          },
          "required" => ["tracking_id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cookies - Cookie regex pattern validation - success
  """
  def create_app_handle_cookies_cookie_regex_pattern_validation___success() do
    routes = [
      %{
        method: :get,
        path: "/cookies/pattern",
        handler: &Handlers.handle_cookies_cookie_regex_pattern_validation___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"tracking_id" => %{"source" => "cookie", "type" => "string"}},
          "required" => []
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cookies - Cookie validation - max_length constraint fail
  """
  def create_app_handle_cookies_cookie_validation___max_length_constraint_fail() do
    routes = [
      %{
        method: :get,
        path: "/cookies/validated",
        handler: &Handlers.handle_cookies_cookie_validation___max_length_constraint_fail/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "session_id" => %{"source" => "cookie", "type" => "string", "maxLength" => 20}
          },
          "required" => ["session_id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cookies - Cookie validation - min_length constraint success
  """
  def create_app_handle_cookies_cookie_validation___min_length_constraint_success() do
    routes = [
      %{
        method: :get,
        path: "/cookies/min-length",
        handler: &Handlers.handle_cookies_cookie_validation___min_length_constraint_success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"token" => %{"source" => "cookie", "type" => "string"}},
          "required" => []
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cookies - Cookie validation - min_length failure
  """
  def create_app_handle_cookies_cookie_validation___min_length_failure() do
    routes = [
      %{
        method: :get,
        path: "/items/",
        handler: &Handlers.handle_cookies_cookie_validation___min_length_failure/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "tracking_id" => %{"source" => "cookie", "type" => "string", "minLength" => 3}
          },
          "required" => ["tracking_id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cookies - Multiple cookies - success
  """
  def create_app_handle_cookies_multiple_cookies___success() do
    routes = [
      %{
        method: :get,
        path: "/items/",
        handler: &Handlers.handle_cookies_multiple_cookies___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "session_id" => %{"source" => "cookie", "type" => "string"},
            "fatebook_tracker" => %{"source" => "cookie", "type" => "string"},
            "googall_tracker" => %{"source" => "cookie", "type" => "string"}
          },
          "required" => []
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cookies - Optional APIKey cookie - missing
  """
  def create_app_handle_cookies_optional_apikey_cookie___missing() do
    routes = [
      %{
        method: :get,
        path: "/users/me",
        handler: &Handlers.handle_cookies_optional_apikey_cookie___missing/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"key" => %{"source" => "cookie", "type" => "string"}},
          "required" => []
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cookies - Optional cookie parameter - missing
  """
  def create_app_handle_cookies_optional_cookie_parameter___missing() do
    routes = [
      %{
        method: :get,
        path: "/items/",
        handler: &Handlers.handle_cookies_optional_cookie_parameter___missing/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"ads_id" => %{"source" => "cookie", "type" => "string"}},
          "required" => []
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cookies - Optional cookie parameter - success
  """
  def create_app_handle_cookies_optional_cookie_parameter___success() do
    routes = [
      %{
        method: :get,
        path: "/items/",
        handler: &Handlers.handle_cookies_optional_cookie_parameter___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"ads_id" => %{"source" => "cookie", "type" => "string"}},
          "required" => []
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cookies - Required cookie - missing
  """
  def create_app_handle_cookies_required_cookie___missing() do
    routes = [
      %{
        method: :get,
        path: "/items/cookies",
        handler: &Handlers.handle_cookies_required_cookie___missing/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "session_id" => %{"source" => "cookie", "type" => "string"},
            "fatebook_tracker" => %{"source" => "cookie", "type" => "string"}
          },
          "required" => ["session_id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cookies - Response - delete cookie
  """
  def create_app_handle_cookies_response___delete_cookie() do
    routes = [
      %{
        method: :post,
        path: "/cookies/delete",
        handler: &Handlers.handle_cookies_response___delete_cookie/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"session" => %{"source" => "cookie", "type" => "string"}},
          "required" => []
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cookies - Response - multiple cookies
  """
  def create_app_handle_cookies_response___multiple_cookies() do
    routes = [
      %{
        method: :post,
        path: "/cookies/multiple",
        handler: &Handlers.handle_cookies_response___multiple_cookies/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"user" => %{"type" => "string"}, "session" => %{"type" => "string"}},
          "required" => ["user", "session"],
          "additionalProperties" => false
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cookies - Response - session cookie (no max_age)
  """
  def create_app_handle_cookies_response___session_cookie__no_max_age_() do
    routes = [
      %{
        method: :post,
        path: "/cookies/session",
        handler: &Handlers.handle_cookies_response___session_cookie__no_max_age_/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"value" => %{"type" => "string"}},
          "required" => ["value"],
          "additionalProperties" => false
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cookies - Response cookie with SameSite=Lax
  """
  def create_app_handle_cookies_response_cookie_with_samesite_lax() do
    routes = [
      %{
        method: :post,
        path: "/cookies/samesite-lax",
        handler: &Handlers.handle_cookies_response_cookie_with_samesite_lax/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"value" => %{"type" => "string"}},
          "required" => ["value"],
          "additionalProperties" => false
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cookies - Response cookie with SameSite=None
  """
  def create_app_handle_cookies_response_cookie_with_samesite_none() do
    routes = [
      %{
        method: :post,
        path: "/cookies/samesite-none",
        handler: &Handlers.handle_cookies_response_cookie_with_samesite_none/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"value" => %{"type" => "string"}},
          "required" => ["value"],
          "additionalProperties" => false
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cookies - Response cookie with SameSite=Strict
  """
  def create_app_handle_cookies_response_cookie_with_samesite_strict() do
    routes = [
      %{
        method: :post,
        path: "/cookies/samesite-strict",
        handler: &Handlers.handle_cookies_response_cookie_with_samesite_strict/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"value" => %{"type" => "string"}},
          "required" => ["value"],
          "additionalProperties" => false
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cookies - Response cookie with attributes
  """
  def create_app_handle_cookies_response_cookie_with_attributes() do
    routes = [
      %{
        method: :get,
        path: "/cookie/set",
        handler: &Handlers.handle_cookies_response_cookie_with_attributes/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cookies - Response cookie with domain attribute
  """
  def create_app_handle_cookies_response_cookie_with_domain_attribute() do
    routes = [
      %{
        method: :post,
        path: "/cookies/set-with-domain",
        handler: &Handlers.handle_cookies_response_cookie_with_domain_attribute/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"value" => %{"type" => "string"}},
          "required" => ["value"],
          "additionalProperties" => false
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cookies - Response cookie with path attribute
  """
  def create_app_handle_cookies_response_cookie_with_path_attribute() do
    routes = [
      %{
        method: :post,
        path: "/cookies/set-with-path",
        handler: &Handlers.handle_cookies_response_cookie_with_path_attribute/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"value" => %{"type" => "string"}},
          "required" => ["value"],
          "additionalProperties" => false
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cookies - Response set cookie - basic
  """
  def create_app_handle_cookies_response_set_cookie___basic() do
    routes = [
      %{
        method: :post,
        path: "/cookie/",
        handler: &Handlers.handle_cookies_response_set_cookie___basic/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cors - 06_cors_preflight_method_not_allowed
  """
  def create_app_handle_cors_06_cors_preflight_method_not_allowed() do
    routes = [
      %{
        method: :options,
        path: "/api/data",
        handler: &Handlers.handle_cors_06_cors_preflight_method_not_allowed/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "Origin" => %{"source" => "header", "type" => "string"},
            "Access-Control-Request-Method" => %{"source" => "header", "type" => "string"},
            "Access-Control-Request-Headers" => %{"source" => "header", "type" => "string"}
          },
          "required" => []
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cors - 07_cors_preflight_header_not_allowed
  """
  def create_app_handle_cors_07_cors_preflight_header_not_allowed() do
    routes = [
      %{
        method: :options,
        path: "/api/data",
        handler: &Handlers.handle_cors_07_cors_preflight_header_not_allowed/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "Origin" => %{"source" => "header", "type" => "string"},
            "Access-Control-Request-Method" => %{"source" => "header", "type" => "string"},
            "Access-Control-Request-Headers" => %{"source" => "header", "type" => "string"}
          },
          "required" => []
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cors - 08_cors_max_age
  """
  def create_app_handle_cors_08_cors_max_age() do
    routes = [
      %{
        method: :options,
        path: "/api/data",
        handler: &Handlers.handle_cors_08_cors_max_age/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "Origin" => %{"source" => "header", "type" => "string"},
            "Access-Control-Request-Method" => %{"source" => "header", "type" => "string"},
            "Access-Control-Request-Headers" => %{"source" => "header", "type" => "string"}
          },
          "required" => []
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cors - 09_cors_expose_headers
  """
  def create_app_handle_cors_09_cors_expose_headers() do
    routes = [
      %{
        method: :get,
        path: "/api/data",
        handler: &Handlers.handle_cors_09_cors_expose_headers/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"Origin" => %{"source" => "header", "type" => "string"}},
          "required" => []
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cors - 10_cors_origin_null
  """
  def create_app_handle_cors_10_cors_origin_null() do
    routes = [
      %{
        method: :get,
        path: "/api/data",
        handler: &Handlers.handle_cors_10_cors_origin_null/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"Origin" => %{"source" => "header", "type" => "string"}},
          "required" => []
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cors - CORS Private Network Access
  """
  def create_app_handle_cors_cors_private_network_access() do
    routes = [
      %{
        method: :options,
        path: "/api/local-resource",
        handler: &Handlers.handle_cors_cors_private_network_access/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cors - CORS Vary header for proper caching
  """
  def create_app_handle_cors_cors_vary_header_for_proper_caching() do
    routes = [
      %{
        method: :get,
        path: "/api/cached-resource",
        handler: &Handlers.handle_cors_cors_vary_header_for_proper_caching/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cors - CORS multiple allowed origins
  """
  def create_app_handle_cors_cors_multiple_allowed_origins() do
    routes = [
      %{
        method: :get,
        path: "/api/data",
        handler: &Handlers.handle_cors_cors_multiple_allowed_origins/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cors - CORS origin case sensitivity
  """
  def create_app_handle_cors_cors_origin_case_sensitivity() do
    routes = [
      %{
        method: :get,
        path: "/api/data",
        handler: &Handlers.handle_cors_cors_origin_case_sensitivity/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cors - CORS preflight for DELETE method
  """
  def create_app_handle_cors_cors_preflight_for_delete_method() do
    routes = [
      %{
        method: :options,
        path: "/api/resource/456",
        handler: &Handlers.handle_cors_cors_preflight_for_delete_method/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cors - CORS preflight for PUT method
  """
  def create_app_handle_cors_cors_preflight_for_put_method() do
    routes = [
      %{
        method: :options,
        path: "/api/resource/123",
        handler: &Handlers.handle_cors_cors_preflight_for_put_method/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cors - CORS preflight request
  """
  def create_app_handle_cors_cors_preflight_request() do
    routes = [
      %{
        method: :options,
        path: "/items/",
        handler: &Handlers.handle_cors_cors_preflight_request/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cors - CORS regex pattern matching for origins
  """
  def create_app_handle_cors_cors_regex_pattern_matching_for_origins() do
    routes = [
      %{
        method: :get,
        path: "/api/data",
        handler: &Handlers.handle_cors_cors_regex_pattern_matching_for_origins/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cors - CORS request blocked
  """
  def create_app_handle_cors_cors_request_blocked() do
    routes = [
      %{
        method: :get,
        path: "/items/",
        handler: &Handlers.handle_cors_cors_request_blocked/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"Origin" => %{"source" => "header", "type" => "string"}},
          "required" => []
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cors - CORS safelisted headers without preflight
  """
  def create_app_handle_cors_cors_safelisted_headers_without_preflight() do
    routes = [
      %{
        method: :post,
        path: "/api/form",
        handler: &Handlers.handle_cors_cors_safelisted_headers_without_preflight/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cors - CORS wildcard origin
  """
  def create_app_handle_cors_cors_wildcard_origin() do
    routes = [
      %{method: :get, path: "/public/data", handler: &Handlers.handle_cors_cors_wildcard_origin/1}
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cors - CORS with credentials
  """
  def create_app_handle_cors_cors_with_credentials() do
    routes = [
      %{
        method: :get,
        path: "/api/user/profile",
        handler: &Handlers.handle_cors_cors_with_credentials/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: cors - Simple CORS request
  """
  def create_app_handle_cors_simple_cors_request() do
    routes = [
      %{method: :get, path: "/items/", handler: &Handlers.handle_cors_simple_cors_request/1}
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: di - Async factory dependency - success
  """
  def create_app_handle_di_async_factory_dependency___success() do
    routes = [
      %{
        method: :get,
        path: "/api/db-status",
        handler: &Handlers.handle_di_async_factory_dependency___success/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: di - Circular dependency detection - error
  """
  def create_app_handle_di_circular_dependency_detection___error() do
    routes = [
      %{
        method: :get,
        path: "/api/circular",
        handler: &Handlers.handle_di_circular_dependency_detection___error/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: di - Dependency injection in lifecycle hooks - success
  """
  def create_app_handle_di_dependency_injection_in_lifecycle_hooks___success() do
    routes = [
      %{
        method: :get,
        path: "/api/hook-di-test",
        handler: &Handlers.handle_di_dependency_injection_in_lifecycle_hooks___success/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: di - Factory dependency - success
  """
  def create_app_handle_di_factory_dependency___success() do
    routes = [
      %{
        method: :get,
        path: "/api/timestamp",
        handler: &Handlers.handle_di_factory_dependency___success/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: di - Missing dependency - error
  """
  def create_app_handle_di_missing_dependency___error() do
    routes = [
      %{
        method: :get,
        path: "/api/missing-dep",
        handler: &Handlers.handle_di_missing_dependency___error/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: di - Mixed singleton and per-request caching - success
  """
  def create_app_handle_di_mixed_singleton_and_per_request_caching___success() do
    routes = [
      %{
        method: :get,
        path: "/api/mixed-caching",
        handler: &Handlers.handle_di_mixed_singleton_and_per_request_caching___success/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: di - Multiple dependencies with cleanup - success
  """
  def create_app_handle_di_multiple_dependencies_with_cleanup___success() do
    routes = [
      %{
        method: :get,
        path: "/api/multi-cleanup-test",
        handler: &Handlers.handle_di_multiple_dependencies_with_cleanup___success/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: di - Nested dependencies (3 levels) - success
  """
  def create_app_handle_di_nested_dependencies__3_levels____success() do
    routes = [
      %{
        method: :get,
        path: "/api/auth-status",
        handler: &Handlers.handle_di_nested_dependencies__3_levels____success/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: di - Node.js object destructuring injection - success
  """
  def create_app_handle_di_node_js_object_destructuring_injection___success() do
    routes = [
      %{
        method: :get,
        path: "/api/node-destructure",
        handler: &Handlers.handle_di_node_js_object_destructuring_injection___success/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: di - Per-request dependency caching - success
  """
  def create_app_handle_di_per_request_dependency_caching___success() do
    routes = [
      %{
        method: :get,
        path: "/api/request-id",
        handler: &Handlers.handle_di_per_request_dependency_caching___success/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: di - Python parameter name-based injection - success
  """
  def create_app_handle_di_python_parameter_name_based_injection___success() do
    routes = [
      %{
        method: :get,
        path: "/api/python-name-inject",
        handler: &Handlers.handle_di_python_parameter_name_based_injection___success/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: di - Python type annotation-based injection - success
  """
  def create_app_handle_di_python_type_annotation_based_injection___success() do
    routes = [
      %{
        method: :get,
        path: "/api/python-type-inject",
        handler: &Handlers.handle_di_python_type_annotation_based_injection___success/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: di - Resource cleanup after request - success
  """
  def create_app_handle_di_resource_cleanup_after_request___success() do
    routes = [
      %{
        method: :get,
        path: "/api/cleanup-test",
        handler: &Handlers.handle_di_resource_cleanup_after_request___success/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: di - Route-level dependency override - success
  """
  def create_app_handle_di_route_level_dependency_override___success() do
    routes = [
      %{
        method: :get,
        path: "/api/override-test",
        handler: &Handlers.handle_di_route_level_dependency_override___success/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: di - Ruby keyword argument injection - success
  """
  def create_app_handle_di_ruby_keyword_argument_injection___success() do
    routes = [
      %{
        method: :get,
        path: "/api/ruby-kwargs",
        handler: &Handlers.handle_di_ruby_keyword_argument_injection___success/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: di - Singleton dependency caching - success
  """
  def create_app_handle_di_singleton_dependency_caching___success() do
    routes = [
      %{
        method: :get,
        path: "/api/app-counter",
        handler: &Handlers.handle_di_singleton_dependency_caching___success/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: di - Type mismatch in dependency resolution - error
  """
  def create_app_handle_di_type_mismatch_in_dependency_resolution___error() do
    routes = [
      %{
        method: :get,
        path: "/api/type-mismatch",
        handler: &Handlers.handle_di_type_mismatch_in_dependency_resolution___error/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: di - Value dependency injection - success
  """
  def create_app_handle_di_value_dependency_injection___success() do
    routes = [
      %{
        method: :get,
        path: "/api/config",
        handler: &Handlers.handle_di_value_dependency_injection___success/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: edge_cases - 11_utf8_query_parameter
  """
  def create_app_handle_edge_cases_11_utf8_query_parameter() do
    routes = [
      %{
        method: :get,
        path: "/search",
        handler: &Handlers.handle_edge_cases_11_utf8_query_parameter/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"term" => %{"source" => "query", "type" => "string"}},
          "required" => ["term"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: edge_cases - 12_percent_encoded_special_chars
  """
  def create_app_handle_edge_cases_12_percent_encoded_special_chars() do
    routes = [
      %{
        method: :get,
        path: "/search",
        handler: &Handlers.handle_edge_cases_12_percent_encoded_special_chars/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"term" => %{"source" => "query", "type" => "string"}},
          "required" => ["term"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: edge_cases - 13_empty_string_query_param_preserved
  """
  def create_app_handle_edge_cases_13_empty_string_query_param_preserved() do
    routes = [
      %{
        method: :get,
        path: "/items",
        handler: &Handlers.handle_edge_cases_13_empty_string_query_param_preserved/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"filter" => %{"source" => "query", "type" => "string"}},
          "required" => ["filter"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: edge_cases - 14_large_integer_boundary
  """
  def create_app_handle_edge_cases_14_large_integer_boundary() do
    routes = [
      %{
        method: :get,
        path: "/items",
        handler: &Handlers.handle_edge_cases_14_large_integer_boundary/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"id" => %{"source" => "query", "type" => "integer"}},
          "required" => ["id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: edge_cases - 15_float_precision_preservation
  """
  def create_app_handle_edge_cases_15_float_precision_preservation() do
    routes = [
      %{
        method: :post,
        path: "/calculate",
        handler: &Handlers.handle_edge_cases_15_float_precision_preservation/1,
        request_schema: %{
          "type" => "object",
          "required" => ["value"],
          "properties" => %{"value" => %{"type" => "number"}}
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: edge_cases - 16_negative_zero_handling
  """
  def create_app_handle_edge_cases_16_negative_zero_handling() do
    routes = [
      %{
        method: :post,
        path: "/data",
        handler: &Handlers.handle_edge_cases_16_negative_zero_handling/1,
        request_schema: %{
          "type" => "object",
          "required" => ["offset"],
          "properties" => %{"offset" => %{"type" => "number"}}
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: edge_cases - 17_extremely_long_string
  """
  def create_app_handle_edge_cases_17_extremely_long_string() do
    routes = [
      %{
        method: :post,
        path: "/text",
        handler: &Handlers.handle_edge_cases_17_extremely_long_string/1,
        request_schema: %{
          "type" => "object",
          "required" => ["content"],
          "properties" => %{"content" => %{"type" => "string", "maxLength" => 10000}}
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: edge_cases - 18_unicode_normalization
  """
  def create_app_handle_edge_cases_18_unicode_normalization() do
    routes = [
      %{
        method: :post,
        path: "/users",
        handler: &Handlers.handle_edge_cases_18_unicode_normalization/1,
        request_schema: %{
          "type" => "object",
          "required" => ["name"],
          "properties" => %{"name" => %{"type" => "string", "minLength" => 1}}
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: edge_cases - 19_emoji_in_strings
  """
  def create_app_handle_edge_cases_19_emoji_in_strings() do
    routes = [
      %{
        method: :post,
        path: "/messages",
        handler: &Handlers.handle_edge_cases_19_emoji_in_strings/1,
        request_schema: %{
          "type" => "object",
          "required" => ["text"],
          "properties" => %{"text" => %{"type" => "string", "minLength" => 1, "maxLength" => 100}}
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: edge_cases - 20_null_byte_in_string
  """
  def create_app_handle_edge_cases_20_null_byte_in_string() do
    routes = [
      %{
        method: :post,
        path: "/files",
        handler: &Handlers.handle_edge_cases_20_null_byte_in_string/1,
        request_schema: %{
          "type" => "object",
          "required" => ["filename"],
          "properties" => %{"filename" => %{"type" => "string", "pattern" => "^[^\\x00]+$"}}
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: edge_cases - 21_scientific_notation_number
  """
  def create_app_handle_edge_cases_21_scientific_notation_number() do
    routes = [
      %{
        method: :post,
        path: "/calculate",
        handler: &Handlers.handle_edge_cases_21_scientific_notation_number/1,
        request_schema: %{
          "type" => "object",
          "required" => ["value"],
          "properties" => %{"value" => %{"type" => "number", "minimum" => 0}}
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: edge_cases - 22_leading_zeros_integer
  """
  def create_app_handle_edge_cases_22_leading_zeros_integer() do
    routes = [
      %{
        method: :get,
        path: "/data",
        handler: &Handlers.handle_edge_cases_22_leading_zeros_integer/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"value" => %{"source" => "query", "type" => "integer"}},
          "required" => ["value"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: edge_cases - 23_deeply_nested_json_limit
  """
  def create_app_handle_edge_cases_23_deeply_nested_json_limit() do
    routes = [
      %{
        method: :post,
        path: "/data",
        handler: &Handlers.handle_edge_cases_23_deeply_nested_json_limit/1,
        request_schema: %{"type" => "object"}
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: edge_cases - 24_array_with_holes
  """
  def create_app_handle_edge_cases_24_array_with_holes() do
    routes = [
      %{
        method: :post,
        path: "/items",
        handler: &Handlers.handle_edge_cases_24_array_with_holes/1,
        request_schema: %{
          "type" => "object",
          "required" => ["items"],
          "properties" => %{"items" => %{"type" => "array", "items" => %{"type" => "string"}}}
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: edge_cases - Deeply nested structure (10+ levels)
  """
  def create_app_handle_edge_cases_deeply_nested_structure__10__levels_() do
    routes = [
      %{
        method: :post,
        path: "/nested/",
        handler: &Handlers.handle_edge_cases_deeply_nested_structure__10__levels_/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "level1" => %{
              "type" => "object",
              "properties" => %{
                "level2" => %{
                  "type" => "object",
                  "properties" => %{
                    "level3" => %{
                      "type" => "object",
                      "properties" => %{
                        "level4" => %{
                          "type" => "object",
                          "properties" => %{
                            "level5" => %{
                              "type" => "object",
                              "properties" => %{
                                "level6" => %{
                                  "type" => "object",
                                  "properties" => %{
                                    "level7" => %{
                                      "type" => "object",
                                      "properties" => %{
                                        "level8" => %{
                                          "type" => "object",
                                          "properties" => %{
                                            "level9" => %{
                                              "type" => "object",
                                              "properties" => %{
                                                "level10" => %{
                                                  "type" => "object",
                                                  "properties" => %{
                                                    "value" => %{"type" => "string"},
                                                    "depth" => %{"type" => "integer"}
                                                  },
                                                  "additionalProperties" => false,
                                                  "required" => ["value", "depth"]
                                                }
                                              },
                                              "additionalProperties" => false,
                                              "required" => ["level10"]
                                            }
                                          },
                                          "additionalProperties" => false,
                                          "required" => ["level9"]
                                        }
                                      },
                                      "additionalProperties" => false,
                                      "required" => ["level8"]
                                    }
                                  },
                                  "additionalProperties" => false,
                                  "required" => ["level7"]
                                }
                              },
                              "additionalProperties" => false,
                              "required" => ["level6"]
                            }
                          },
                          "additionalProperties" => false,
                          "required" => ["level5"]
                        }
                      },
                      "additionalProperties" => false,
                      "required" => ["level4"]
                    }
                  },
                  "additionalProperties" => false,
                  "required" => ["level3"]
                }
              },
              "additionalProperties" => false,
              "required" => ["level2"]
            }
          },
          "additionalProperties" => false,
          "required" => ["level1"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: edge_cases - Empty and null value handling
  """
  def create_app_handle_edge_cases_empty_and_null_value_handling() do
    routes = [
      %{
        method: :post,
        path: "/nulls/",
        handler: &Handlers.handle_edge_cases_empty_and_null_value_handling/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "explicit_null" => %{"type" => "null"},
            "empty_string" => %{"type" => "string"},
            "empty_array" => %{"type" => "array", "items" => %{}},
            "empty_object" => %{
              "type" => "object",
              "properties" => %{},
              "additionalProperties" => false
            },
            "zero_number" => %{"type" => "integer"},
            "false_boolean" => %{"type" => "boolean"}
          },
          "additionalProperties" => false,
          "required" => [
            "explicit_null",
            "empty_string",
            "empty_array",
            "empty_object",
            "zero_number",
            "false_boolean"
          ]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: edge_cases - Float precision and rounding
  """
  def create_app_handle_edge_cases_float_precision_and_rounding() do
    routes = [
      %{
        method: :post,
        path: "/calculations/",
        handler: &Handlers.handle_edge_cases_float_precision_and_rounding/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "value1" => %{"type" => "number"},
            "value2" => %{"type" => "number"},
            "expected_sum" => %{"type" => "number"},
            "precise_value" => %{"type" => "number"},
            "very_small" => %{"type" => "number"},
            "very_large" => %{"type" => "number"}
          },
          "additionalProperties" => false,
          "required" => [
            "value1",
            "value2",
            "expected_sum",
            "precise_value",
            "very_small",
            "very_large"
          ]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: edge_cases - Large integer boundary values
  """
  def create_app_handle_edge_cases_large_integer_boundary_values() do
    routes = [
      %{
        method: :post,
        path: "/numbers/",
        handler: &Handlers.handle_edge_cases_large_integer_boundary_values/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "max_safe_int" => %{"type" => "integer"},
            "large_int" => %{"type" => "integer"},
            "negative_large" => %{"type" => "integer"}
          },
          "additionalProperties" => false,
          "required" => ["max_safe_int", "large_int", "negative_large"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: edge_cases - Special string values and escaping
  """
  def create_app_handle_edge_cases_special_string_values_and_escaping() do
    routes = [
      %{
        method: :post,
        path: "/strings/",
        handler: &Handlers.handle_edge_cases_special_string_values_and_escaping/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "empty_string" => %{"type" => "string"},
            "whitespace" => %{"type" => "string"},
            "tabs_newlines" => %{"type" => "string"},
            "quotes" => %{"type" => "string"},
            "backslashes" => %{"type" => "string"},
            "unicode_escapes" => %{"type" => "string"},
            "special_chars" => %{"type" => "string"}
          },
          "additionalProperties" => false,
          "required" => [
            "empty_string",
            "whitespace",
            "tabs_newlines",
            "quotes",
            "backslashes",
            "unicode_escapes",
            "special_chars"
          ]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: edge_cases - Unicode and emoji handling
  """
  def create_app_handle_edge_cases_unicode_and_emoji_handling() do
    routes = [
      %{
        method: :post,
        path: "/items/",
        handler: &Handlers.handle_edge_cases_unicode_and_emoji_handling/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"type" => "string"},
            "description" => %{"type" => "string"},
            "tags" => %{"type" => "array", "items" => %{"type" => "string"}},
            "emoji_reactions" => %{"type" => "string"}
          },
          "additionalProperties" => false,
          "required" => ["name", "description", "tags", "emoji_reactions"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - 30_bearer_token_format_valid
  """
  def create_app_handle_headers_30_bearer_token_format_valid() do
    routes = [
      %{
        method: :get,
        path: "/protected",
        handler: &Handlers.handle_headers_30_bearer_token_format_valid/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "Authorization" => %{
              "source" => "header",
              "type" => "string",
              "pattern" => "^Bearer [A-Za-z0-9-._~+/]+=*$"
            }
          },
          "required" => ["Authorization"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - 31_bearer_token_format_invalid
  """
  def create_app_handle_headers_31_bearer_token_format_invalid() do
    routes = [
      %{
        method: :get,
        path: "/protected",
        handler: &Handlers.handle_headers_31_bearer_token_format_invalid/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "Authorization" => %{
              "source" => "header",
              "type" => "string",
              "pattern" => "^Bearer [A-Za-z0-9-._~+/]+=*$"
            }
          },
          "required" => ["Authorization"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - 32_bearer_token_missing_prefix
  """
  def create_app_handle_headers_32_bearer_token_missing_prefix() do
    routes = [
      %{
        method: :get,
        path: "/protected",
        handler: &Handlers.handle_headers_32_bearer_token_missing_prefix/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "Authorization" => %{
              "source" => "header",
              "type" => "string",
              "pattern" => "^Bearer [A-Za-z0-9-._~+/]+=*$"
            }
          },
          "required" => ["Authorization"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - 33_api_key_header_valid
  """
  def create_app_handle_headers_33_api_key_header_valid() do
    routes = [
      %{
        method: :get,
        path: "/api/data",
        handler: &Handlers.handle_headers_33_api_key_header_valid/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "X-API-Key" => %{
              "source" => "header",
              "type" => "string",
              "pattern" => "^[a-f0-9]{32}$"
            }
          },
          "required" => ["X-API-Key"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - 34_api_key_header_invalid
  """
  def create_app_handle_headers_34_api_key_header_invalid() do
    routes = [
      %{
        method: :get,
        path: "/api/data",
        handler: &Handlers.handle_headers_34_api_key_header_invalid/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "X-API-Key" => %{
              "source" => "header",
              "type" => "string",
              "pattern" => "^[a-f0-9]{32}$"
            }
          },
          "required" => ["X-API-Key"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - Accept header - JSON
  """
  def create_app_handle_headers_accept_header___json() do
    routes = [
      %{
        method: :get,
        path: "/headers/accept",
        handler: &Handlers.handle_headers_accept_header___json/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"Accept" => %{"source" => "header", "type" => "string"}},
          "required" => ["Accept"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - Accept-Encoding header
  """
  def create_app_handle_headers_accept_encoding_header() do
    routes = [
      %{
        method: :get,
        path: "/headers/accept-encoding",
        handler: &Handlers.handle_headers_accept_encoding_header/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"Accept-Encoding" => %{"source" => "header", "type" => "string"}},
          "required" => ["Accept-Encoding"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - Accept-Language header
  """
  def create_app_handle_headers_accept_language_header() do
    routes = [
      %{
        method: :get,
        path: "/headers/accept-language",
        handler: &Handlers.handle_headers_accept_language_header/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"Accept-Language" => %{"source" => "header", "type" => "string"}},
          "required" => ["Accept-Language"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - Authorization header - missing
  """
  def create_app_handle_headers_authorization_header___missing() do
    routes = [
      %{
        method: :get,
        path: "/users/me",
        handler: &Handlers.handle_headers_authorization_header___missing/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"Authorization" => %{"source" => "header", "type" => "string"}},
          "required" => ["Authorization"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - Authorization header - success
  """
  def create_app_handle_headers_authorization_header___success() do
    routes = [
      %{
        method: :get,
        path: "/users/me",
        handler: &Handlers.handle_headers_authorization_header___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"Authorization" => %{"source" => "header", "type" => "string"}},
          "required" => ["Authorization"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - Authorization header - wrong scheme
  """
  def create_app_handle_headers_authorization_header___wrong_scheme() do
    routes = [
      %{
        method: :get,
        path: "/users/me",
        handler: &Handlers.handle_headers_authorization_header___wrong_scheme/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "Authorization" => %{
              "source" => "header",
              "type" => "string",
              "pattern" => "^Digest .+"
            }
          },
          "required" => ["Authorization"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - Basic authentication - success
  """
  def create_app_handle_headers_basic_authentication___success() do
    routes = [
      %{
        method: :get,
        path: "/headers/basic-auth",
        handler: &Handlers.handle_headers_basic_authentication___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"Authorization" => %{"source" => "header", "type" => "string"}},
          "required" => ["Authorization"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - Bearer token authentication - missing
  """
  def create_app_handle_headers_bearer_token_authentication___missing() do
    routes = [
      %{
        method: :get,
        path: "/headers/bearer-auth",
        handler: &Handlers.handle_headers_bearer_token_authentication___missing/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "Authorization" => %{
              "source" => "header",
              "type" => "string",
              "pattern" => "^Bearer .+"
            }
          },
          "required" => ["Authorization"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - Bearer token authentication - success
  """
  def create_app_handle_headers_bearer_token_authentication___success() do
    routes = [
      %{
        method: :get,
        path: "/headers/bearer-auth",
        handler: &Handlers.handle_headers_bearer_token_authentication___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"Authorization" => %{"source" => "header", "type" => "string"}},
          "required" => ["Authorization"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - Content-Type header - application/json
  """
  def create_app_handle_headers_content_type_header___application_json() do
    routes = [
      %{
        method: :get,
        path: "/headers/content-type",
        handler: &Handlers.handle_headers_content_type_header___application_json/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"Content-Type" => %{"source" => "header", "type" => "string"}},
          "required" => ["Content-Type"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - Header case insensitivity - access
  """
  def create_app_handle_headers_header_case_insensitivity___access() do
    routes = [
      %{
        method: :post,
        path: "/echo",
        handler: &Handlers.handle_headers_header_case_insensitivity___access/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"test" => %{"type" => "string"}},
          "additionalProperties" => false,
          "required" => ["test"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - Header regex validation - fail
  """
  def create_app_handle_headers_header_regex_validation___fail() do
    routes = [
      %{
        method: :get,
        path: "/headers/pattern",
        handler: &Handlers.handle_headers_header_regex_validation___fail/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "X-Request-Id" => %{
              "source" => "header",
              "type" => "string",
              "pattern" => "^[0-9]{3,}$"
            }
          },
          "required" => ["X-Request-Id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - Header regex validation - success
  """
  def create_app_handle_headers_header_regex_validation___success() do
    routes = [
      %{
        method: :get,
        path: "/headers/pattern",
        handler: &Handlers.handle_headers_header_regex_validation___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "X-Request-Id" => %{
              "source" => "header",
              "type" => "string",
              "pattern" => "^[0-9]{3,}$"
            }
          },
          "required" => ["X-Request-Id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - Header validation - max_length constraint fail
  """
  def create_app_handle_headers_header_validation___max_length_constraint_fail() do
    routes = [
      %{
        method: :get,
        path: "/headers/max-length",
        handler: &Handlers.handle_headers_header_validation___max_length_constraint_fail/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "X-Session-Id" => %{"source" => "header", "type" => "string", "maxLength" => 20}
          },
          "required" => ["X-Session-Id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - Header validation - min_length constraint
  """
  def create_app_handle_headers_header_validation___min_length_constraint() do
    routes = [
      %{
        method: :get,
        path: "/headers/validated",
        handler: &Handlers.handle_headers_header_validation___min_length_constraint/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "X-Token" => %{"source" => "header", "type" => "string", "minLength" => 3}
          },
          "required" => ["X-Token"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - Header with underscore conversion - explicit
  """
  def create_app_handle_headers_header_with_underscore_conversion___explicit() do
    routes = [
      %{
        method: :get,
        path: "/headers/underscore",
        handler: &Handlers.handle_headers_header_with_underscore_conversion___explicit/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"X-Token" => %{"source" => "header", "type" => "string"}},
          "required" => ["X-Token"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - Host header
  """
  def create_app_handle_headers_host_header() do
    routes = [
      %{
        method: :get,
        path: "/headers/host",
        handler: &Handlers.handle_headers_host_header/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"Host" => %{"source" => "header", "type" => "string"}},
          "required" => ["Host"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - Multiple custom headers
  """
  def create_app_handle_headers_multiple_custom_headers() do
    routes = [
      %{
        method: :get,
        path: "/headers/multiple",
        handler: &Handlers.handle_headers_multiple_custom_headers/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "X-Request-Id" => %{"source" => "header", "type" => "string"},
            "X-Client-Version" => %{"source" => "header", "type" => "string"},
            "X-Trace-Id" => %{"source" => "header", "type" => "string"}
          },
          "required" => ["X-Request-Id", "X-Client-Version", "X-Trace-Id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - Multiple header values - X-Token
  """
  def create_app_handle_headers_multiple_header_values___x_token() do
    routes = [
      %{
        method: :get,
        path: "/items/",
        handler: &Handlers.handle_headers_multiple_header_values___x_token/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"x-token" => %{"source" => "header", "type" => "string"}},
          "required" => ["x-token"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - Optional header with None default - missing
  """
  def create_app_handle_headers_optional_header_with_none_default___missing() do
    routes = [
      %{
        method: :get,
        path: "/items/",
        handler: &Handlers.handle_headers_optional_header_with_none_default___missing/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "strange-header" => %{"source" => "header", "type" => "string", "default" => nil}
          },
          "required" => []
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - Origin header
  """
  def create_app_handle_headers_origin_header() do
    routes = [
      %{
        method: :get,
        path: "/headers/origin",
        handler: &Handlers.handle_headers_origin_header/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"Origin" => %{"source" => "header", "type" => "string"}},
          "required" => ["Origin"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - Referer header
  """
  def create_app_handle_headers_referer_header() do
    routes = [
      %{
        method: :get,
        path: "/headers/referer",
        handler: &Handlers.handle_headers_referer_header/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"Referer" => %{"source" => "header", "type" => "string"}},
          "required" => ["Referer"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - User-Agent header - custom value
  """
  def create_app_handle_headers_user_agent_header___custom_value() do
    routes = [
      %{
        method: :get,
        path: "/items/",
        handler: &Handlers.handle_headers_user_agent_header___custom_value/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"User-Agent" => %{"source" => "header", "type" => "string"}},
          "required" => ["User-Agent"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - User-Agent header - default value
  """
  def create_app_handle_headers_user_agent_header___default_value() do
    routes = [
      %{
        method: :get,
        path: "/items/",
        handler: &Handlers.handle_headers_user_agent_header___default_value/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "User-Agent" => %{"source" => "header", "type" => "string", "default" => "testclient"}
          },
          "required" => []
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - X-API-Key optional header - missing
  """
  def create_app_handle_headers_x_api_key_optional_header___missing() do
    routes = [
      %{
        method: :get,
        path: "/users/me",
        handler: &Handlers.handle_headers_x_api_key_optional_header___missing/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"key" => %{"source" => "header", "type" => "string"}},
          "required" => []
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - X-API-Key optional header - success
  """
  def create_app_handle_headers_x_api_key_optional_header___success() do
    routes = [
      %{
        method: :get,
        path: "/users/me",
        handler: &Handlers.handle_headers_x_api_key_optional_header___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"key" => %{"source" => "header", "type" => "string"}},
          "required" => []
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - X-API-Key required header - missing
  """
  def create_app_handle_headers_x_api_key_required_header___missing() do
    routes = [
      %{
        method: :get,
        path: "/users/me",
        handler: &Handlers.handle_headers_x_api_key_required_header___missing/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"X-API-Key" => %{"source" => "header", "type" => "string"}},
          "required" => ["X-API-Key"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: headers - X-API-Key required header - success
  """
  def create_app_handle_headers_x_api_key_required_header___success() do
    routes = [
      %{
        method: :get,
        path: "/users/me",
        handler: &Handlers.handle_headers_x_api_key_required_header___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"key" => %{"source" => "header", "type" => "string"}},
          "required" => ["key"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: http_methods - DELETE - Remove resource
  """
  def create_app_handle_http_methods_delete___remove_resource() do
    routes = [
      %{
        method: :delete,
        path: "/items/{id}",
        handler: &Handlers.handle_http_methods_delete___remove_resource/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"id" => %{"source" => "path", "type" => "string"}},
          "required" => ["id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: http_methods - DELETE - Resource not found
  """
  def create_app_handle_http_methods_delete___resource_not_found() do
    routes = [
      %{
        method: :delete,
        path: "/items/{id}",
        handler: &Handlers.handle_http_methods_delete___resource_not_found/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"id" => %{"source" => "path", "type" => "string"}},
          "required" => ["id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: http_methods - DELETE - With response body
  """
  def create_app_handle_http_methods_delete___with_response_body() do
    routes = [
      %{
        method: :delete,
        path: "/items/{id}",
        handler: &Handlers.handle_http_methods_delete___with_response_body/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"id" => %{"source" => "path", "type" => "string"}},
          "required" => ["id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: http_methods - HEAD - Get metadata without body
  """
  def create_app_handle_http_methods_head___get_metadata_without_body() do
    routes = [
      %{
        method: :head,
        path: "/items/{id}",
        handler: &Handlers.handle_http_methods_head___get_metadata_without_body/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"id" => %{"source" => "path", "type" => "string"}},
          "required" => ["id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: http_methods - OPTIONS - CORS preflight request
  """
  def create_app_handle_http_methods_options___cors_preflight_request() do
    routes = [
      %{
        method: :options,
        path: "/items/",
        handler: &Handlers.handle_http_methods_options___cors_preflight_request/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: http_methods - PATCH - Partial update
  """
  def create_app_handle_http_methods_patch___partial_update() do
    routes = [
      %{
        method: :patch,
        path: "/items/{id}",
        handler: &Handlers.handle_http_methods_patch___partial_update/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"id" => %{"source" => "path", "type" => "string"}},
          "required" => ["id"]
        },
        request_schema: %{
          "type" => "object",
          "properties" => %{"price" => %{"type" => "number"}},
          "required" => ["price"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: http_methods - PATCH - Update multiple fields
  """
  def create_app_handle_http_methods_patch___update_multiple_fields() do
    routes = [
      %{
        method: :patch,
        path: "/items/{id}",
        handler: &Handlers.handle_http_methods_patch___update_multiple_fields/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"id" => %{"source" => "path", "type" => "string"}},
          "required" => ["id"]
        },
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"type" => "string"},
            "price" => %{"type" => "number"},
            "in_stock" => %{"type" => "boolean"}
          },
          "required" => ["in_stock", "name", "price"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: http_methods - PUT - Complete resource replacement
  """
  def create_app_handle_http_methods_put___complete_resource_replacement() do
    routes = [
      %{
        method: :put,
        path: "/items/{id}",
        handler: &Handlers.handle_http_methods_put___complete_resource_replacement/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"id" => %{"source" => "path", "type" => "string"}},
          "required" => ["id"]
        },
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "id" => %{"type" => "integer"},
            "name" => %{"type" => "string"},
            "description" => %{"type" => "string"},
            "price" => %{"type" => "number"},
            "in_stock" => %{"type" => "boolean"}
          },
          "required" => ["description", "id", "in_stock", "name", "price"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: http_methods - PUT - Create resource if doesn't exist
  """
  def create_app_handle_http_methods_put___create_resource_if_doesn_t_exist() do
    routes = [
      %{
        method: :put,
        path: "/items/{id}",
        handler: &Handlers.handle_http_methods_put___create_resource_if_doesn_t_exist/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"id" => %{"source" => "path", "type" => "string"}},
          "required" => ["id"]
        },
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "id" => %{"type" => "integer"},
            "name" => %{"type" => "string"},
            "price" => %{"type" => "number"}
          },
          "required" => ["id", "name", "price"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: http_methods - PUT - Idempotent operation
  """
  def create_app_handle_http_methods_put___idempotent_operation() do
    routes = [
      %{
        method: :put,
        path: "/items/{id}",
        handler: &Handlers.handle_http_methods_put___idempotent_operation/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"id" => %{"source" => "path", "type" => "string"}},
          "required" => ["id"]
        },
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "id" => %{"type" => "integer"},
            "name" => %{"type" => "string"},
            "price" => %{"type" => "number"}
          },
          "required" => ["id", "name", "price"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: http_methods - PUT - Missing required field
  """
  def create_app_handle_http_methods_put___missing_required_field() do
    routes = [
      %{
        method: :put,
        path: "/items/{id}",
        handler: &Handlers.handle_http_methods_put___missing_required_field/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"id" => %{"source" => "path", "type" => "string"}},
          "required" => ["id"]
        },
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "id" => %{"type" => "integer"},
            "name" => %{"type" => "string"},
            "price" => %{"type" => "string"}
          },
          "required" => ["price"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: http_methods - PUT - Validation error
  """
  def create_app_handle_http_methods_put___validation_error() do
    routes = [
      %{
        method: :put,
        path: "/items/{id}",
        handler: &Handlers.handle_http_methods_put___validation_error/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"id" => %{"source" => "path", "type" => "string"}},
          "required" => ["id"]
        },
        request_schema: %{
          "$schema" => "https://json-schema.org/draft/2020-12/schema",
          "type" => "object",
          "required" => ["id", "name", "price"],
          "properties" => %{
            "id" => %{"type" => "integer"},
            "name" => %{"type" => "string", "minLength" => 3},
            "price" => %{"type" => "number", "exclusiveMinimum" => 0}
          }
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - 29_nested_object_validation_success
  """
  def create_app_handle_json_bodies_29_nested_object_validation_success() do
    routes = [
      %{
        method: :post,
        path: "/users",
        handler: &Handlers.handle_json_bodies_29_nested_object_validation_success/1,
        request_schema: %{
          "type" => "object",
          "required" => ["profile"],
          "properties" => %{
            "profile" => %{
              "type" => "object",
              "required" => ["name", "email"],
              "properties" => %{
                "name" => %{"type" => "string", "minLength" => 1},
                "email" => %{"type" => "string", "format" => "email"}
              }
            }
          }
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - 30_nested_object_missing_field
  """
  def create_app_handle_json_bodies_30_nested_object_missing_field() do
    routes = [
      %{
        method: :post,
        path: "/users",
        handler: &Handlers.handle_json_bodies_30_nested_object_missing_field/1,
        request_schema: %{
          "type" => "object",
          "required" => ["profile"],
          "properties" => %{
            "profile" => %{
              "type" => "object",
              "required" => ["name", "email"],
              "properties" => %{
                "name" => %{"type" => "string", "minLength" => 1},
                "email" => %{"type" => "string", "format" => "email"}
              }
            }
          }
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - 31_nullable_property_null_value
  """
  def create_app_handle_json_bodies_31_nullable_property_null_value() do
    routes = [
      %{
        method: :post,
        path: "/users",
        handler: &Handlers.handle_json_bodies_31_nullable_property_null_value/1,
        request_schema: %{
          "type" => "object",
          "required" => ["name"],
          "properties" => %{
            "name" => %{"type" => "string"},
            "description" => %{"type" => ["string", "null"]}
          }
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - 32_schema_ref_definitions
  """
  def create_app_handle_json_bodies_32_schema_ref_definitions() do
    routes = [
      %{
        method: :post,
        path: "/products",
        handler: &Handlers.handle_json_bodies_32_schema_ref_definitions/1,
        request_schema: %{
          "type" => "object",
          "required" => ["product"],
          "properties" => %{"product" => %{"$ref" => "#/definitions/Product"}},
          "definitions" => %{
            "Product" => %{
              "type" => "object",
              "required" => ["name", "price"],
              "properties" => %{
                "name" => %{"type" => "string"},
                "price" => %{"type" => "number", "minimum" => 0}
              }
            }
          }
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - 33_allof_schema_composition
  """
  def create_app_handle_json_bodies_33_allof_schema_composition() do
    routes = [
      %{
        method: :post,
        path: "/items",
        handler: &Handlers.handle_json_bodies_33_allof_schema_composition/1,
        request_schema: %{
          "allOf" => [
            %{
              "type" => "object",
              "required" => ["name"],
              "properties" => %{"name" => %{"type" => "string"}}
            },
            %{
              "type" => "object",
              "required" => ["price"],
              "properties" => %{"price" => %{"type" => "number", "minimum" => 0}}
            }
          ]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - 34_additional_properties_false
  """
  def create_app_handle_json_bodies_34_additional_properties_false() do
    routes = [
      %{
        method: :post,
        path: "/users",
        handler: &Handlers.handle_json_bodies_34_additional_properties_false/1,
        request_schema: %{
          "type" => "object",
          "required" => ["name"],
          "properties" => %{"name" => %{"type" => "string"}, "email" => %{"type" => "string"}},
          "additionalProperties" => false
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - 35_oneof_schema_success
  """
  def create_app_handle_json_bodies_35_oneof_schema_success() do
    routes = [
      %{
        method: :post,
        path: "/payment",
        handler: &Handlers.handle_json_bodies_35_oneof_schema_success/1,
        request_schema: %{
          "oneOf" => [
            %{
              "type" => "object",
              "required" => ["credit_card"],
              "properties" => %{
                "credit_card" => %{"type" => "string", "pattern" => "^[0-9]{16}$"}
              }
            },
            %{
              "type" => "object",
              "required" => ["paypal_email"],
              "properties" => %{"paypal_email" => %{"type" => "string", "format" => "email"}}
            }
          ]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - 36_oneof_schema_multiple_match_failure
  """
  def create_app_handle_json_bodies_36_oneof_schema_multiple_match_failure() do
    routes = [
      %{
        method: :post,
        path: "/payment",
        handler: &Handlers.handle_json_bodies_36_oneof_schema_multiple_match_failure/1,
        request_schema: %{
          "oneOf" => [
            %{
              "type" => "object",
              "required" => ["credit_card"],
              "properties" => %{
                "credit_card" => %{"type" => "string", "pattern" => "^[0-9]{16}$"}
              }
            },
            %{
              "type" => "object",
              "required" => ["paypal_email"],
              "properties" => %{"paypal_email" => %{"type" => "string", "format" => "email"}}
            }
          ]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - 37_oneof_schema_no_match_failure
  """
  def create_app_handle_json_bodies_37_oneof_schema_no_match_failure() do
    routes = [
      %{
        method: :post,
        path: "/payment",
        handler: &Handlers.handle_json_bodies_37_oneof_schema_no_match_failure/1,
        request_schema: %{
          "oneOf" => [
            %{
              "type" => "object",
              "required" => ["credit_card"],
              "properties" => %{
                "credit_card" => %{"type" => "string", "pattern" => "^[0-9]{16}$"}
              }
            },
            %{
              "type" => "object",
              "required" => ["paypal_email"],
              "properties" => %{"paypal_email" => %{"type" => "string", "format" => "email"}}
            }
          ]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - 38_anyof_schema_success
  """
  def create_app_handle_json_bodies_38_anyof_schema_success() do
    routes = [
      %{
        method: :post,
        path: "/contact",
        handler: &Handlers.handle_json_bodies_38_anyof_schema_success/1,
        request_schema: %{
          "type" => "object",
          "required" => ["name"],
          "properties" => %{"name" => %{"type" => "string"}},
          "anyOf" => [%{"required" => ["email"]}, %{"required" => ["phone"]}]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - 39_anyof_schema_multiple_match_success
  """
  def create_app_handle_json_bodies_39_anyof_schema_multiple_match_success() do
    routes = [
      %{
        method: :post,
        path: "/contact",
        handler: &Handlers.handle_json_bodies_39_anyof_schema_multiple_match_success/1,
        request_schema: %{
          "type" => "object",
          "required" => ["name"],
          "properties" => %{
            "name" => %{"type" => "string"},
            "email" => %{"type" => "string", "format" => "email"},
            "phone" => %{"type" => "string"}
          },
          "anyOf" => [%{"required" => ["email"]}, %{"required" => ["phone"]}]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - 40_anyof_schema_failure
  """
  def create_app_handle_json_bodies_40_anyof_schema_failure() do
    routes = [
      %{
        method: :post,
        path: "/contact",
        handler: &Handlers.handle_json_bodies_40_anyof_schema_failure/1,
        request_schema: %{
          "type" => "object",
          "required" => ["name"],
          "properties" => %{
            "name" => %{"type" => "string"},
            "email" => %{"type" => "string", "format" => "email"},
            "phone" => %{"type" => "string"}
          },
          "anyOf" => [%{"required" => ["email"]}, %{"required" => ["phone"]}]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - 41_not_schema_success
  """
  def create_app_handle_json_bodies_41_not_schema_success() do
    routes = [
      %{
        method: :post,
        path: "/users",
        handler: &Handlers.handle_json_bodies_41_not_schema_success/1,
        request_schema: %{
          "type" => "object",
          "required" => ["username"],
          "properties" => %{
            "username" => %{"type" => "string", "not" => %{"enum" => ["admin", "root", "system"]}}
          }
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - 42_not_schema_failure
  """
  def create_app_handle_json_bodies_42_not_schema_failure() do
    routes = [
      %{
        method: :post,
        path: "/users",
        handler: &Handlers.handle_json_bodies_42_not_schema_failure/1,
        request_schema: %{
          "type" => "object",
          "required" => ["username"],
          "properties" => %{
            "username" => %{"type" => "string", "not" => %{"enum" => ["admin", "root", "system"]}}
          }
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - 43_const_validation_success
  """
  def create_app_handle_json_bodies_43_const_validation_success() do
    routes = [
      %{
        method: :post,
        path: "/api/v1/data",
        handler: &Handlers.handle_json_bodies_43_const_validation_success/1,
        request_schema: %{
          "type" => "object",
          "required" => ["version", "data"],
          "properties" => %{
            "version" => %{"type" => "string", "const" => "1.0"},
            "data" => %{"type" => "string"}
          }
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - 44_const_validation_failure
  """
  def create_app_handle_json_bodies_44_const_validation_failure() do
    routes = [
      %{
        method: :post,
        path: "/api/v1/data",
        handler: &Handlers.handle_json_bodies_44_const_validation_failure/1,
        request_schema: %{
          "type" => "object",
          "required" => ["version", "data"],
          "properties" => %{
            "version" => %{"type" => "string", "const" => "1.0"},
            "data" => %{"type" => "string"}
          }
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - 45_minproperties_validation_success
  """
  def create_app_handle_json_bodies_45_minproperties_validation_success() do
    routes = [
      %{
        method: :post,
        path: "/config",
        handler: &Handlers.handle_json_bodies_45_minproperties_validation_success/1,
        request_schema: %{"type" => "object", "minProperties" => 2}
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - 46_minproperties_validation_failure
  """
  def create_app_handle_json_bodies_46_minproperties_validation_failure() do
    routes = [
      %{
        method: :post,
        path: "/config",
        handler: &Handlers.handle_json_bodies_46_minproperties_validation_failure/1,
        request_schema: %{"type" => "object", "minProperties" => 2}
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - 47_maxproperties_validation_failure
  """
  def create_app_handle_json_bodies_47_maxproperties_validation_failure() do
    routes = [
      %{
        method: :post,
        path: "/config",
        handler: &Handlers.handle_json_bodies_47_maxproperties_validation_failure/1,
        request_schema: %{"type" => "object", "maxProperties" => 3}
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - 48_dependencies_validation_success
  """
  def create_app_handle_json_bodies_48_dependencies_validation_success() do
    routes = [
      %{
        method: :post,
        path: "/billing",
        handler: &Handlers.handle_json_bodies_48_dependencies_validation_success/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"type" => "string"},
            "credit_card" => %{"type" => "string"},
            "billing_address" => %{"type" => "string"}
          },
          "dependencies" => %{"credit_card" => ["billing_address"]}
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - 49_dependencies_validation_failure
  """
  def create_app_handle_json_bodies_49_dependencies_validation_failure() do
    routes = [
      %{
        method: :post,
        path: "/billing",
        handler: &Handlers.handle_json_bodies_49_dependencies_validation_failure/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"type" => "string"},
            "credit_card" => %{"type" => "string"},
            "billing_address" => %{"type" => "string"}
          },
          "dependencies" => %{"credit_card" => ["billing_address"]}
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - 50_deep_nesting_4_levels
  """
  def create_app_handle_json_bodies_50_deep_nesting_4_levels() do
    routes = [
      %{
        method: :post,
        path: "/data",
        handler: &Handlers.handle_json_bodies_50_deep_nesting_4_levels/1,
        request_schema: %{
          "type" => "object",
          "required" => ["user"],
          "properties" => %{
            "user" => %{
              "type" => "object",
              "required" => ["profile"],
              "properties" => %{
                "profile" => %{
                  "type" => "object",
                  "required" => ["contact"],
                  "properties" => %{
                    "contact" => %{
                      "type" => "object",
                      "required" => ["address"],
                      "properties" => %{
                        "address" => %{
                          "type" => "object",
                          "required" => ["street"],
                          "properties" => %{"street" => %{"type" => "string"}}
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - Array of objects - success
  """
  def create_app_handle_json_bodies_array_of_objects___success() do
    routes = [
      %{
        method: :post,
        path: "/items/list",
        handler: &Handlers.handle_json_bodies_array_of_objects___success/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"type" => "string"},
            "tags" => %{"type" => "array", "items" => %{"type" => "string"}},
            "images" => %{
              "type" => "array",
              "items" => %{
                "type" => "object",
                "properties" => %{"url" => %{"type" => "string"}, "name" => %{"type" => "string"}},
                "additionalProperties" => false,
                "required" => ["url", "name"]
              }
            }
          },
          "additionalProperties" => false,
          "required" => ["name", "tags", "images"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - Array of primitive values
  """
  def create_app_handle_json_bodies_array_of_primitive_values() do
    routes = [
      %{
        method: :post,
        path: "/items/",
        handler: &Handlers.handle_json_bodies_array_of_primitive_values/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"type" => "string"},
            "tags" => %{"type" => "array", "items" => %{"type" => "string"}},
            "ratings" => %{"type" => "array", "items" => %{"type" => "number"}}
          },
          "additionalProperties" => false,
          "required" => ["name", "tags", "ratings"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - Body with query parameters
  """
  def create_app_handle_json_bodies_body_with_query_parameters() do
    routes = [
      %{
        method: :post,
        path: "/items/",
        handler: &Handlers.handle_json_bodies_body_with_query_parameters/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"limit" => %{"source" => "query", "type" => "integer"}},
          "required" => ["limit"]
        },
        request_schema: %{
          "type" => "object",
          "properties" => %{"name" => %{"type" => "string"}, "price" => %{"type" => "number"}},
          "additionalProperties" => false,
          "required" => ["name", "price"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - Boolean field - success
  """
  def create_app_handle_json_bodies_boolean_field___success() do
    routes = [
      %{
        method: :post,
        path: "/items/",
        handler: &Handlers.handle_json_bodies_boolean_field___success/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"type" => "string"},
            "price" => %{"type" => "number"},
            "in_stock" => %{"type" => "boolean"}
          },
          "additionalProperties" => false,
          "required" => ["name", "price", "in_stock"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - Date field - success
  """
  def create_app_handle_json_bodies_date_field___success() do
    routes = [
      %{
        method: :post,
        path: "/events/",
        handler: &Handlers.handle_json_bodies_date_field___success/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"type" => "string"},
            "event_date" => %{"type" => "string"}
          },
          "additionalProperties" => false,
          "required" => ["name", "event_date"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - Datetime field - success
  """
  def create_app_handle_json_bodies_datetime_field___success() do
    routes = [
      %{
        method: :post,
        path: "/events/",
        handler: &Handlers.handle_json_bodies_datetime_field___success/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"type" => "string"},
            "created_at" => %{"type" => "string", "format" => "date-time"}
          },
          "additionalProperties" => false,
          "required" => ["name", "created_at"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - Deeply nested objects
  """
  def create_app_handle_json_bodies_deeply_nested_objects() do
    routes = [
      %{
        method: :post,
        path: "/items/nested",
        handler: &Handlers.handle_json_bodies_deeply_nested_objects/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"type" => "string"},
            "price" => %{"type" => "number"},
            "seller" => %{
              "type" => "object",
              "properties" => %{
                "name" => %{"type" => "string"},
                "address" => %{
                  "type" => "object",
                  "properties" => %{
                    "street" => %{"type" => "string"},
                    "city" => %{"type" => "string"},
                    "country" => %{
                      "type" => "object",
                      "properties" => %{
                        "name" => %{"type" => "string"},
                        "code" => %{"type" => "string"}
                      },
                      "additionalProperties" => false,
                      "required" => ["name", "code"]
                    }
                  },
                  "additionalProperties" => false,
                  "required" => ["street", "city", "country"]
                }
              },
              "additionalProperties" => false,
              "required" => ["name", "address"]
            }
          },
          "additionalProperties" => false,
          "required" => ["name", "price", "seller"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - Empty JSON object
  """
  def create_app_handle_json_bodies_empty_json_object() do
    routes = [
      %{
        method: :post,
        path: "/items/optional-all",
        handler: &Handlers.handle_json_bodies_empty_json_object/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{},
          "additionalProperties" => false
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - Empty array validation - fail
  """
  def create_app_handle_json_bodies_empty_array_validation___fail() do
    routes = [
      %{
        method: :post,
        path: "/items/list-validated",
        handler: &Handlers.handle_json_bodies_empty_array_validation___fail/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"type" => "string"},
            "tags" => %{"type" => "array", "items" => %{}, "minItems" => 1}
          },
          "additionalProperties" => false,
          "required" => ["name", "tags"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - Enum field - invalid value
  """
  def create_app_handle_json_bodies_enum_field___invalid_value() do
    routes = [
      %{
        method: :post,
        path: "/items/",
        handler: &Handlers.handle_json_bodies_enum_field___invalid_value/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"type" => "string"},
            "category" => %{"type" => "string", "enum" => ["electronics", "clothing", "books"]}
          },
          "additionalProperties" => false,
          "required" => ["name", "category"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - Enum field - success
  """
  def create_app_handle_json_bodies_enum_field___success() do
    routes = [
      %{
        method: :post,
        path: "/items/",
        handler: &Handlers.handle_json_bodies_enum_field___success/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"name" => %{"type" => "string"}, "category" => %{"type" => "string"}},
          "additionalProperties" => false,
          "required" => ["name", "category"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - Extra fields ignored (no additionalProperties)
  """
  def create_app_handle_json_bodies_extra_fields_ignored__no_additionalproperties_() do
    routes = [
      %{
        method: :post,
        path: "/items/",
        handler: &Handlers.handle_json_bodies_extra_fields_ignored__no_additionalproperties_/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"type" => "string"},
            "price" => %{"type" => "number"},
            "extra_field" => %{"type" => "string"},
            "another_extra" => %{"type" => "integer"}
          },
          "additionalProperties" => false,
          "required" => ["name", "price", "extra_field", "another_extra"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - Field type validation - invalid type
  """
  def create_app_handle_json_bodies_field_type_validation___invalid_type() do
    routes = [
      %{
        method: :post,
        path: "/items/",
        handler: &Handlers.handle_json_bodies_field_type_validation___invalid_type/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"type" => "string"},
            "description" => %{"type" => "string"},
            "price" => %{"type" => "number"},
            "tax" => %{"type" => "number"}
          },
          "additionalProperties" => false,
          "required" => ["name", "description", "price", "tax"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - Nested object - success
  """
  def create_app_handle_json_bodies_nested_object___success() do
    routes = [
      %{
        method: :post,
        path: "/items/nested",
        handler: &Handlers.handle_json_bodies_nested_object___success/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"type" => "string"},
            "price" => %{"type" => "number"},
            "image" => %{
              "type" => "object",
              "properties" => %{"url" => %{"type" => "string"}, "name" => %{"type" => "string"}},
              "additionalProperties" => false,
              "required" => ["url", "name"]
            }
          },
          "additionalProperties" => false,
          "required" => ["name", "price", "image"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - Null value for optional field
  """
  def create_app_handle_json_bodies_null_value_for_optional_field() do
    routes = [
      %{
        method: :post,
        path: "/items/",
        handler: &Handlers.handle_json_bodies_null_value_for_optional_field/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"type" => "string"},
            "price" => %{"type" => "number"},
            "description" => %{"type" => "null"},
            "tax" => %{"type" => "null"}
          },
          "additionalProperties" => false,
          "required" => ["name", "price", "description", "tax"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - Numeric ge validation - fail
  """
  def create_app_handle_json_bodies_numeric_ge_validation___fail() do
    routes = [
      %{
        method: :post,
        path: "/items/validated",
        handler: &Handlers.handle_json_bodies_numeric_ge_validation___fail/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"type" => "string"},
            "price" => %{"type" => "number", "minimum" => 1}
          },
          "additionalProperties" => false,
          "required" => ["name", "price"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - Numeric le validation - success
  """
  def create_app_handle_json_bodies_numeric_le_validation___success() do
    routes = [
      %{
        method: :post,
        path: "/items/validated",
        handler: &Handlers.handle_json_bodies_numeric_le_validation___success/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"name" => %{"type" => "string"}, "price" => %{"type" => "number"}},
          "additionalProperties" => false,
          "required" => ["name", "price"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - Optional fields - omitted
  """
  def create_app_handle_json_bodies_optional_fields___omitted() do
    routes = [
      %{
        method: :post,
        path: "/items/",
        handler: &Handlers.handle_json_bodies_optional_fields___omitted/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"name" => %{"type" => "string"}, "price" => %{"type" => "number"}},
          "additionalProperties" => false,
          "required" => ["name", "price"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - PATCH partial update
  """
  def create_app_handle_json_bodies_patch_partial_update() do
    routes = [
      %{
        method: :patch,
        path: "/items/{id}",
        handler: &Handlers.handle_json_bodies_patch_partial_update/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"id" => %{"source" => "path", "type" => "string"}},
          "required" => ["id"]
        },
        request_schema: %{
          "type" => "object",
          "properties" => %{"price" => %{"type" => "number"}},
          "required" => ["price"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - Required field missing - validation error
  """
  def create_app_handle_json_bodies_required_field_missing___validation_error() do
    routes = [
      %{
        method: :post,
        path: "/items/",
        handler: &Handlers.handle_json_bodies_required_field_missing___validation_error/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "description" => %{"type" => "string"},
            "price" => %{"type" => "number"},
            "name" => %{"type" => "string"}
          },
          "additionalProperties" => false,
          "required" => ["description", "price", "name"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - Simple JSON object - success
  """
  def create_app_handle_json_bodies_simple_json_object___success() do
    routes = [
      %{
        method: :post,
        path: "/items/",
        handler: &Handlers.handle_json_bodies_simple_json_object___success/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"type" => "string"},
            "description" => %{"type" => "string"},
            "price" => %{"type" => "number"},
            "tax" => %{"type" => "number"}
          },
          "additionalProperties" => false,
          "required" => ["name", "description", "price", "tax"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - String max_length validation - fail
  """
  def create_app_handle_json_bodies_string_max_length_validation___fail() do
    routes = [
      %{
        method: :post,
        path: "/items/validated",
        handler: &Handlers.handle_json_bodies_string_max_length_validation___fail/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"type" => "string", "maxLength" => 50},
            "price" => %{"type" => "number"}
          },
          "additionalProperties" => false,
          "required" => ["name", "price"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - String min_length validation - fail
  """
  def create_app_handle_json_bodies_string_min_length_validation___fail() do
    routes = [
      %{
        method: :post,
        path: "/items/validated",
        handler: &Handlers.handle_json_bodies_string_min_length_validation___fail/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"type" => "string", "minLength" => 3},
            "price" => %{"type" => "number"}
          },
          "additionalProperties" => false,
          "required" => ["name", "price"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - String pattern validation - fail
  """
  def create_app_handle_json_bodies_string_pattern_validation___fail() do
    routes = [
      %{
        method: :post,
        path: "/items/validated",
        handler: &Handlers.handle_json_bodies_string_pattern_validation___fail/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"type" => "string"},
            "sku" => %{"type" => "string", "pattern" => "^[A-Z]{3}[0-9]{4}$"}
          },
          "additionalProperties" => false,
          "required" => ["name", "sku"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - String pattern validation - success
  """
  def create_app_handle_json_bodies_string_pattern_validation___success() do
    routes = [
      %{
        method: :post,
        path: "/items/validated",
        handler: &Handlers.handle_json_bodies_string_pattern_validation___success/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"name" => %{"type" => "string"}, "sku" => %{"type" => "string"}},
          "additionalProperties" => false,
          "required" => ["name", "sku"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - UUID field - invalid format
  """
  def create_app_handle_json_bodies_uuid_field___invalid_format() do
    routes = [
      %{
        method: :post,
        path: "/items/",
        handler: &Handlers.handle_json_bodies_uuid_field___invalid_format/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"type" => "string"},
            "item_id" => %{"type" => "string", "format" => "uuid"}
          },
          "additionalProperties" => false,
          "required" => ["name", "item_id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: json_bodies - UUID field - success
  """
  def create_app_handle_json_bodies_uuid_field___success() do
    routes = [
      %{
        method: :post,
        path: "/items/",
        handler: &Handlers.handle_json_bodies_uuid_field___success/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"type" => "string"},
            "item_id" => %{"type" => "string", "format" => "uuid"}
          },
          "additionalProperties" => false,
          "required" => ["name", "item_id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: lifecycle_hooks - Hook Execution Order
  """
  def create_app_handle_lifecycle_hooks_hook_execution_order() do
    routes = [
      %{
        method: :get,
        path: "/api/test-hook-order",
        handler: &Handlers.handle_lifecycle_hooks_hook_execution_order/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: lifecycle_hooks - Multiple Hooks - All Phases
  """
  def create_app_handle_lifecycle_hooks_multiple_hooks___all_phases() do
    routes = [
      %{
        method: :post,
        path: "/api/full-lifecycle",
        handler: &Handlers.handle_lifecycle_hooks_multiple_hooks___all_phases/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"user_id" => %{"type" => "string"}, "action" => %{"type" => "string"}},
          "required" => ["user_id", "action"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: lifecycle_hooks - onError - Error Logging
  """
  def create_app_handle_lifecycle_hooks_onerror___error_logging() do
    routes = [
      %{
        method: :get,
        path: "/api/test-error",
        handler: &Handlers.handle_lifecycle_hooks_onerror___error_logging/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: lifecycle_hooks - onRequest - Request Logging
  """
  def create_app_handle_lifecycle_hooks_onrequest___request_logging() do
    routes = [
      %{
        method: :get,
        path: "/api/test-on-request",
        handler: &Handlers.handle_lifecycle_hooks_onrequest___request_logging/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: lifecycle_hooks - onResponse - Response Timing
  """
  def create_app_handle_lifecycle_hooks_onresponse___response_timing() do
    routes = [
      %{
        method: :get,
        path: "/api/test-timing",
        handler: &Handlers.handle_lifecycle_hooks_onresponse___response_timing/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: lifecycle_hooks - onResponse - Security Headers
  """
  def create_app_handle_lifecycle_hooks_onresponse___security_headers() do
    routes = [
      %{
        method: :get,
        path: "/api/test-security-headers",
        handler: &Handlers.handle_lifecycle_hooks_onresponse___security_headers/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: lifecycle_hooks - preHandler - Authentication Failed (Short Circuit)
  """
  def create_app_handle_lifecycle_hooks_prehandler___authentication_failed__short_circuit_() do
    routes = [
      %{
        method: :get,
        path: "/api/protected-resource-fail",
        handler:
          &Handlers.handle_lifecycle_hooks_prehandler___authentication_failed__short_circuit_/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: lifecycle_hooks - preHandler - Authentication Success
  """
  def create_app_handle_lifecycle_hooks_prehandler___authentication_success() do
    routes = [
      %{
        method: :get,
        path: "/api/protected-resource",
        handler: &Handlers.handle_lifecycle_hooks_prehandler___authentication_success/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: lifecycle_hooks - preHandler - Authorization Check
  """
  def create_app_handle_lifecycle_hooks_prehandler___authorization_check() do
    routes = [
      %{
        method: :get,
        path: "/api/admin-only",
        handler: &Handlers.handle_lifecycle_hooks_prehandler___authorization_check/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: lifecycle_hooks - preHandler - Authorization Forbidden (Short Circuit)
  """
  def create_app_handle_lifecycle_hooks_prehandler___authorization_forbidden__short_circuit_() do
    routes = [
      %{
        method: :get,
        path: "/api/admin-only-forbidden",
        handler:
          &Handlers.handle_lifecycle_hooks_prehandler___authorization_forbidden__short_circuit_/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: lifecycle_hooks - preValidation - Rate Limit Exceeded (Short Circuit)
  """
  def create_app_handle_lifecycle_hooks_prevalidation___rate_limit_exceeded__short_circuit_() do
    routes = [
      %{
        method: :post,
        path: "/api/test-rate-limit-exceeded",
        handler:
          &Handlers.handle_lifecycle_hooks_prevalidation___rate_limit_exceeded__short_circuit_/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"data" => %{"type" => "string"}},
          "required" => ["data"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: lifecycle_hooks - preValidation - Rate Limiting
  """
  def create_app_handle_lifecycle_hooks_prevalidation___rate_limiting() do
    routes = [
      %{
        method: :post,
        path: "/api/test-rate-limit",
        handler: &Handlers.handle_lifecycle_hooks_prevalidation___rate_limiting/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"data" => %{"type" => "string"}},
          "required" => ["data"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: multipart - 17_file_magic_number_png_success
  """
  def create_app_handle_multipart_17_file_magic_number_png_success() do
    routes = [
      %{
        method: :post,
        path: "/upload",
        handler: &Handlers.handle_multipart_17_file_magic_number_png_success/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: multipart - 18_file_magic_number_jpeg_success
  """
  def create_app_handle_multipart_18_file_magic_number_jpeg_success() do
    routes = [
      %{
        method: :post,
        path: "/upload",
        handler: &Handlers.handle_multipart_18_file_magic_number_jpeg_success/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: multipart - 19_file_mime_spoofing_png_as_jpeg
  """
  def create_app_handle_multipart_19_file_mime_spoofing_png_as_jpeg() do
    routes = [
      %{
        method: :post,
        path: "/upload",
        handler: &Handlers.handle_multipart_19_file_mime_spoofing_png_as_jpeg/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: multipart - 20_file_mime_spoofing_jpeg_as_png
  """
  def create_app_handle_multipart_20_file_mime_spoofing_jpeg_as_png() do
    routes = [
      %{
        method: :post,
        path: "/upload",
        handler: &Handlers.handle_multipart_20_file_mime_spoofing_jpeg_as_png/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: multipart - 21_file_pdf_magic_number_success
  """
  def create_app_handle_multipart_21_file_pdf_magic_number_success() do
    routes = [
      %{
        method: :post,
        path: "/upload",
        handler: &Handlers.handle_multipart_21_file_pdf_magic_number_success/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: multipart - 22_file_empty_buffer
  """
  def create_app_handle_multipart_22_file_empty_buffer() do
    routes = [
      %{
        method: :post,
        path: "/upload",
        handler: &Handlers.handle_multipart_22_file_empty_buffer/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: multipart - Content-Type validation - invalid type
  """
  def create_app_handle_multipart_content_type_validation___invalid_type() do
    routes = [
      %{
        method: :post,
        path: "/files/images-only",
        handler: &Handlers.handle_multipart_content_type_validation___invalid_type/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"file" => %{"type" => "string", "format" => "binary"}},
          "additionalProperties" => false
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: multipart - Empty file upload
  """
  def create_app_handle_multipart_empty_file_upload() do
    routes = [
      %{
        method: :post,
        path: "/files/upload",
        handler: &Handlers.handle_multipart_empty_file_upload/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"file" => %{"type" => "string", "format" => "binary"}},
          "additionalProperties" => false,
          "required" => ["file"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: multipart - File list upload (array of files)
  """
  def create_app_handle_multipart_file_list_upload__array_of_files_() do
    routes = [
      %{
        method: :post,
        path: "/files/list",
        handler: &Handlers.handle_multipart_file_list_upload__array_of_files_/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "files" => %{
              "type" => "array",
              "items" => %{"type" => "string", "format" => "binary"}
            }
          },
          "additionalProperties" => false,
          "required" => ["files"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: multipart - File size validation - too large
  """
  def create_app_handle_multipart_file_size_validation___too_large() do
    routes = [
      %{
        method: :post,
        path: "/files/validated",
        handler: &Handlers.handle_multipart_file_size_validation___too_large/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"file" => %{"type" => "string", "format" => "binary"}},
          "additionalProperties" => false
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: multipart - File upload with custom headers
  """
  def create_app_handle_multipart_file_upload_with_custom_headers() do
    routes = [
      %{
        method: :post,
        path: "/",
        handler: &Handlers.handle_multipart_file_upload_with_custom_headers/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"test2" => %{"type" => "string", "format" => "binary"}},
          "additionalProperties" => false,
          "required" => ["test2"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: multipart - File upload without filename
  """
  def create_app_handle_multipart_file_upload_without_filename() do
    routes = [
      %{
        method: :post,
        path: "/",
        handler: &Handlers.handle_multipart_file_upload_without_filename/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"test1" => %{"type" => "string", "format" => "binary"}},
          "additionalProperties" => false,
          "required" => ["test1"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: multipart - Form data without files
  """
  def create_app_handle_multipart_form_data_without_files() do
    routes = [
      %{
        method: :post,
        path: "/",
        handler: &Handlers.handle_multipart_form_data_without_files/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"some" => %{"type" => "string"}},
          "additionalProperties" => false
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: multipart - Image file upload
  """
  def create_app_handle_multipart_image_file_upload() do
    routes = [
      %{
        method: :post,
        path: "/files/image",
        handler: &Handlers.handle_multipart_image_file_upload/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"image" => %{"type" => "string", "format" => "binary"}},
          "additionalProperties" => false,
          "required" => ["image"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: multipart - Mixed files and form data
  """
  def create_app_handle_multipart_mixed_files_and_form_data() do
    routes = [
      %{
        method: :post,
        path: "/",
        handler: &Handlers.handle_multipart_mixed_files_and_form_data/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "file" => %{"type" => "string", "format" => "binary"},
            "username" => %{"type" => "string"},
            "age" => %{"type" => "string"},
            "active" => %{"type" => "string"}
          },
          "additionalProperties" => false,
          "required" => ["file"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: multipart - Multiple file uploads
  """
  def create_app_handle_multipart_multiple_file_uploads() do
    routes = [
      %{
        method: :post,
        path: "/",
        handler: &Handlers.handle_multipart_multiple_file_uploads/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "test1" => %{"type" => "string", "format" => "binary"},
            "test2" => %{"type" => "string", "format" => "binary"}
          },
          "additionalProperties" => false,
          "required" => ["test1", "test2"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: multipart - Multiple values for same field name
  """
  def create_app_handle_multipart_multiple_values_for_same_field_name() do
    routes = [
      %{
        method: :post,
        path: "/",
        handler: &Handlers.handle_multipart_multiple_values_for_same_field_name/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "files" => %{
              "type" => "array",
              "items" => %{"type" => "string", "format" => "binary"}
            },
            "tags" => %{"type" => "array", "items" => %{"type" => "string"}}
          },
          "additionalProperties" => false,
          "required" => ["files"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: multipart - Optional file upload - missing
  """
  def create_app_handle_multipart_optional_file_upload___missing() do
    routes = [
      %{
        method: :post,
        path: "/files/optional",
        handler: &Handlers.handle_multipart_optional_file_upload___missing/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{},
          "additionalProperties" => false
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: multipart - Optional file upload - provided
  """
  def create_app_handle_multipart_optional_file_upload___provided() do
    routes = [
      %{
        method: :post,
        path: "/files/optional",
        handler: &Handlers.handle_multipart_optional_file_upload___provided/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"file" => %{"type" => "string", "format" => "binary"}},
          "additionalProperties" => false,
          "required" => ["file"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: multipart - PDF file upload
  """
  def create_app_handle_multipart_pdf_file_upload() do
    routes = [
      %{
        method: :post,
        path: "/files/document",
        handler: &Handlers.handle_multipart_pdf_file_upload/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"document" => %{"type" => "string", "format" => "binary"}},
          "additionalProperties" => false,
          "required" => ["document"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: multipart - Required file upload - missing
  """
  def create_app_handle_multipart_required_file_upload___missing() do
    routes = [
      %{
        method: :post,
        path: "/files/required",
        handler: &Handlers.handle_multipart_required_file_upload___missing/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"file" => %{"type" => "string", "format" => "binary"}},
          "required" => ["file"],
          "additionalProperties" => false
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: multipart - Simple file upload
  """
  def create_app_handle_multipart_simple_file_upload() do
    routes = [
      %{
        method: :post,
        path: "/",
        handler: &Handlers.handle_multipart_simple_file_upload/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"test" => %{"type" => "string", "format" => "binary"}},
          "additionalProperties" => false,
          "required" => ["test"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - 20_uuid_v3_path_param_success
  """
  def create_app_handle_path_params_20_uuid_v3_path_param_success() do
    routes = [
      %{
        method: :get,
        path: "/items/{id}",
        handler: &Handlers.handle_path_params_20_uuid_v3_path_param_success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"id" => %{"source" => "path", "type" => "string", "format" => "uuid"}},
          "required" => ["id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - 21_uuid_v5_path_param_success
  """
  def create_app_handle_path_params_21_uuid_v5_path_param_success() do
    routes = [
      %{
        method: :get,
        path: "/items/{id}",
        handler: &Handlers.handle_path_params_21_uuid_v5_path_param_success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"id" => %{"source" => "path", "type" => "string", "format" => "uuid"}},
          "required" => ["id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - 24_date_format_path_param_success
  """
  def create_app_handle_path_params_24_date_format_path_param_success() do
    routes = [
      %{
        method: :get,
        path: "/events/{date}",
        handler: &Handlers.handle_path_params_24_date_format_path_param_success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "date" => %{"source" => "path", "type" => "string", "format" => "date"}
          },
          "required" => ["date"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - 25_date_format_invalid_failure
  """
  def create_app_handle_path_params_25_date_format_invalid_failure() do
    routes = [
      %{
        method: :get,
        path: "/events/{date}",
        handler: &Handlers.handle_path_params_25_date_format_invalid_failure/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "date" => %{"source" => "path", "type" => "string", "format" => "date"}
          },
          "required" => ["date"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - 27_datetime_format_path_param_success
  """
  def create_app_handle_path_params_27_datetime_format_path_param_success() do
    routes = [
      %{
        method: :get,
        path: "/bookings/{timestamp}",
        handler: &Handlers.handle_path_params_27_datetime_format_path_param_success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "timestamp" => %{"source" => "path", "type" => "string", "format" => "date-time"}
          },
          "required" => ["timestamp"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - 28_duration_format_path_param_success
  """
  def create_app_handle_path_params_28_duration_format_path_param_success() do
    routes = [
      %{
        method: :get,
        path: "/delays/{duration}",
        handler: &Handlers.handle_path_params_28_duration_format_path_param_success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "duration" => %{"source" => "path", "type" => "string", "format" => "duration"}
          },
          "required" => ["duration"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - 29_decimal_path_param_success
  """
  def create_app_handle_path_params_29_decimal_path_param_success() do
    routes = [
      %{
        method: :get,
        path: "/prices/{amount}",
        handler: &Handlers.handle_path_params_29_decimal_path_param_success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "amount" => %{"source" => "path", "type" => "string", "format" => "decimal"}
          },
          "required" => ["amount"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - 30_string_minlength_path_success
  """
  def create_app_handle_path_params_30_string_minlength_path_success() do
    routes = [
      %{
        method: :get,
        path: "/users/{username}",
        handler: &Handlers.handle_path_params_30_string_minlength_path_success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "username" => %{"source" => "path", "type" => "string", "minLength" => 3}
          },
          "required" => ["username"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - 31_string_minlength_path_failure
  """
  def create_app_handle_path_params_31_string_minlength_path_failure() do
    routes = [
      %{
        method: :get,
        path: "/users/{username}",
        handler: &Handlers.handle_path_params_31_string_minlength_path_failure/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "username" => %{"source" => "path", "type" => "string", "minLength" => 3}
          },
          "required" => ["username"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - 32_string_maxlength_path_failure
  """
  def create_app_handle_path_params_32_string_maxlength_path_failure() do
    routes = [
      %{
        method: :get,
        path: "/users/{username}",
        handler: &Handlers.handle_path_params_32_string_maxlength_path_failure/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "username" => %{"source" => "path", "type" => "string", "maxLength" => 20}
          },
          "required" => ["username"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - 33_string_pattern_path_success
  """
  def create_app_handle_path_params_33_string_pattern_path_success() do
    routes = [
      %{
        method: :get,
        path: "/repos/{owner}/{repo}",
        handler: &Handlers.handle_path_params_33_string_pattern_path_success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "owner" => %{"source" => "path", "type" => "string", "pattern" => "^[a-zA-Z0-9-]+$"},
            "repo" => %{"source" => "path", "type" => "string", "pattern" => "^[a-zA-Z0-9-_]+$"}
          },
          "required" => ["owner", "repo"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - 34_string_pattern_path_failure
  """
  def create_app_handle_path_params_34_string_pattern_path_failure() do
    routes = [
      %{
        method: :get,
        path: "/repos/{owner}",
        handler: &Handlers.handle_path_params_34_string_pattern_path_failure/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "owner" => %{"source" => "path", "type" => "string", "pattern" => "^[a-zA-Z0-9-]+$"}
          },
          "required" => ["owner"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - 35_negative_integer_path_param
  """
  def create_app_handle_path_params_35_negative_integer_path_param() do
    routes = [
      %{
        method: :get,
        path: "/offset/{value}",
        handler: &Handlers.handle_path_params_35_negative_integer_path_param/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"value" => %{"source" => "path", "type" => "integer"}},
          "required" => ["value"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - Boolean path parameter - True
  """
  def create_app_handle_path_params_boolean_path_parameter___true() do
    routes = [
      %{
        method: :get,
        path: "/path/bool/{item_id}",
        handler: &Handlers.handle_path_params_boolean_path_parameter___true/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"item_id" => %{"source" => "path", "type" => "boolean"}},
          "required" => ["item_id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - Boolean path parameter - numeric 1
  """
  def create_app_handle_path_params_boolean_path_parameter___numeric_1() do
    routes = [
      %{
        method: :get,
        path: "/path/bool/{item_id}",
        handler: &Handlers.handle_path_params_boolean_path_parameter___numeric_1/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"item_id" => %{"source" => "path", "type" => "boolean"}},
          "required" => ["item_id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - Date path parameter - success
  """
  def create_app_handle_path_params_date_path_parameter___success() do
    routes = [
      %{
        method: :get,
        path: "/date/{date_param}",
        handler: &Handlers.handle_path_params_date_path_parameter___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "date_param" => %{"source" => "path", "type" => "string", "format" => "date"}
          },
          "required" => ["date_param"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - Enum path parameter - invalid value
  """
  def create_app_handle_path_params_enum_path_parameter___invalid_value() do
    routes = [
      %{
        method: :get,
        path: "/models/{model_name}",
        handler: &Handlers.handle_path_params_enum_path_parameter___invalid_value/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "model_name" => %{
              "source" => "path",
              "type" => "string",
              "enum" => ["alexnet", "resnet", "lenet"]
            }
          },
          "required" => ["model_name"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - Enum path parameter - success
  """
  def create_app_handle_path_params_enum_path_parameter___success() do
    routes = [
      %{
        method: :get,
        path: "/models/{model_name}",
        handler: &Handlers.handle_path_params_enum_path_parameter___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "model_name" => %{
              "source" => "path",
              "type" => "string",
              "enum" => ["alexnet", "lenet", "resnet"]
            }
          },
          "required" => ["model_name"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - Float path parameter - success
  """
  def create_app_handle_path_params_float_path_parameter___success() do
    routes = [
      %{
        method: :get,
        path: "/path/float/{item_id}",
        handler: &Handlers.handle_path_params_float_path_parameter___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"item_id" => %{"source" => "path", "type" => "number"}},
          "required" => ["item_id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - Integer path parameter - invalid string
  """
  def create_app_handle_path_params_integer_path_parameter___invalid_string() do
    routes = [
      %{
        method: :get,
        path: "/path/int/{item_id}",
        handler: &Handlers.handle_path_params_integer_path_parameter___invalid_string/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"item_id" => %{"source" => "path", "type" => "integer"}},
          "required" => ["item_id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - Integer path parameter - success
  """
  def create_app_handle_path_params_integer_path_parameter___success() do
    routes = [
      %{
        method: :get,
        path: "/path/int/{item_id}",
        handler: &Handlers.handle_path_params_integer_path_parameter___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"item_id" => %{"source" => "path", "type" => "integer"}},
          "required" => ["item_id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - Integer path parameter with combined lt and gt constraints - success
  """
  def create_app_handle_path_params_integer_path_parameter_with_combined_lt_and_gt_constraints___success() do
    routes = [
      %{
        method: :get,
        path: "/path/param-lt-gt/{item_id}",
        handler:
          &Handlers.handle_path_params_integer_path_parameter_with_combined_lt_and_gt_constraints___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"item_id" => %{"source" => "path", "type" => "integer"}},
          "required" => ["item_id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - Integer path parameter with ge constraint - success
  """
  def create_app_handle_path_params_integer_path_parameter_with_ge_constraint___success() do
    routes = [
      %{
        method: :get,
        path: "/path/param-ge/{item_id}",
        handler:
          &Handlers.handle_path_params_integer_path_parameter_with_ge_constraint___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "item_id" => %{"source" => "path", "type" => "integer", "minimum" => 3}
          },
          "required" => ["item_id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - Integer path parameter with gt constraint - failure
  """
  def create_app_handle_path_params_integer_path_parameter_with_gt_constraint___failure() do
    routes = [
      %{
        method: :get,
        path: "/path/param-gt/{item_id}",
        handler:
          &Handlers.handle_path_params_integer_path_parameter_with_gt_constraint___failure/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"item_id" => %{"source" => "path", "type" => "integer"}},
          "required" => ["item_id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - Integer path parameter with gt constraint - success
  """
  def create_app_handle_path_params_integer_path_parameter_with_gt_constraint___success() do
    routes = [
      %{
        method: :get,
        path: "/path/param-gt/{item_id}",
        handler:
          &Handlers.handle_path_params_integer_path_parameter_with_gt_constraint___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"item_id" => %{"source" => "path", "type" => "integer"}},
          "required" => ["item_id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - Integer path parameter with le constraint - success
  """
  def create_app_handle_path_params_integer_path_parameter_with_le_constraint___success() do
    routes = [
      %{
        method: :get,
        path: "/path/param-le/{item_id}",
        handler:
          &Handlers.handle_path_params_integer_path_parameter_with_le_constraint___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "item_id" => %{"source" => "path", "type" => "integer", "maximum" => 3}
          },
          "required" => ["item_id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - Integer path parameter with lt constraint - success
  """
  def create_app_handle_path_params_integer_path_parameter_with_lt_constraint___success() do
    routes = [
      %{
        method: :get,
        path: "/path/param-lt/{item_id}",
        handler:
          &Handlers.handle_path_params_integer_path_parameter_with_lt_constraint___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"item_id" => %{"source" => "path", "type" => "integer"}},
          "required" => ["item_id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - Multiple path parameters - success
  """
  def create_app_handle_path_params_multiple_path_parameters___success() do
    routes = [
      %{
        method: :get,
        path: "/{version}/{service_id}/{user_id}/{order_id}",
        handler: &Handlers.handle_path_params_multiple_path_parameters___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "version" => %{"source" => "path", "type" => "number"},
            "service_id" => %{"source" => "path", "type" => "integer"},
            "user_id" => %{"source" => "path", "type" => "string"},
            "order_id" => %{"source" => "path", "type" => "string", "format" => "uuid"}
          },
          "required" => ["version", "service_id", "user_id", "order_id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - Path parameter type syntax - invalid UUID
  """
  def create_app_handle_path_params_path_parameter_type_syntax___invalid_uuid() do
    routes = [
      %{
        method: :get,
        path: "/type-syntax/items/{id:uuid}",
        handler: &Handlers.handle_path_params_path_parameter_type_syntax___invalid_uuid/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - Path parameter type syntax with override
  """
  def create_app_handle_path_params_path_parameter_type_syntax_with_override() do
    routes = [
      %{
        method: :get,
        path: "/type-syntax/items-count/{count:int}",
        handler: &Handlers.handle_path_params_path_parameter_type_syntax_with_override/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "count" => %{
              "source" => "path",
              "type" => "integer",
              "minimum" => 1,
              "maximum" => 100
            }
          },
          "required" => ["count"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - Path parameter with type syntax - UUID
  """
  def create_app_handle_path_params_path_parameter_with_type_syntax___uuid() do
    routes = [
      %{
        method: :get,
        path: "/type-syntax/items/{id:uuid}",
        handler: &Handlers.handle_path_params_path_parameter_with_type_syntax___uuid/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - Path parameter with type syntax - integer
  """
  def create_app_handle_path_params_path_parameter_with_type_syntax___integer() do
    routes = [
      %{
        method: :get,
        path: "/type-syntax/users/{user_id:int}",
        handler: &Handlers.handle_path_params_path_parameter_with_type_syntax___integer/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - Path type parameter - file path
  """
  def create_app_handle_path_params_path_type_parameter___file_path() do
    routes = [
      %{
        method: :get,
        path: "/files/{file_path:path}",
        handler: &Handlers.handle_path_params_path_type_parameter___file_path/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"file_path" => %{"source" => "path", "type" => "string"}},
          "required" => ["file_path"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - String path parameter - success
  """
  def create_app_handle_path_params_string_path_parameter___success() do
    routes = [
      %{
        method: :get,
        path: "/path/str/{item_id}",
        handler: &Handlers.handle_path_params_string_path_parameter___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"item_id" => %{"source" => "path", "type" => "string"}},
          "required" => ["item_id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - String path parameter with max_length - failure
  """
  def create_app_handle_path_params_string_path_parameter_with_max_length___failure() do
    routes = [
      %{
        method: :get,
        path: "/path/param-maxlength/{item_id}",
        handler: &Handlers.handle_path_params_string_path_parameter_with_max_length___failure/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "item_id" => %{"source" => "path", "type" => "string", "maxLength" => 3}
          },
          "required" => ["item_id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - String path parameter with min_length - failure
  """
  def create_app_handle_path_params_string_path_parameter_with_min_length___failure() do
    routes = [
      %{
        method: :get,
        path: "/path/param-minlength/{item_id}",
        handler: &Handlers.handle_path_params_string_path_parameter_with_min_length___failure/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "item_id" => %{"source" => "path", "type" => "string", "minLength" => 3}
          },
          "required" => ["item_id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: path_params - UUID path parameter - success
  """
  def create_app_handle_path_params_uuid_path_parameter___success() do
    routes = [
      %{
        method: :get,
        path: "/items/{item_id}",
        handler: &Handlers.handle_path_params_uuid_path_parameter___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "item_id" => %{"source" => "path", "type" => "string", "format" => "uuid"}
          },
          "required" => ["item_id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 42_negative_integer_query_param
  """
  def create_app_handle_query_params_42_negative_integer_query_param() do
    routes = [
      %{
        method: :get,
        path: "/items/negative",
        handler: &Handlers.handle_query_params_42_negative_integer_query_param/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"offset" => %{"source" => "query", "type" => "integer"}},
          "required" => ["offset"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 43_scientific_notation_float
  """
  def create_app_handle_query_params_43_scientific_notation_float() do
    routes = [
      %{
        method: :get,
        path: "/stats",
        handler: &Handlers.handle_query_params_43_scientific_notation_float/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"threshold" => %{"source" => "query", "type" => "number"}},
          "required" => ["threshold"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 44_string_minlength_validation_success
  """
  def create_app_handle_query_params_44_string_minlength_validation_success() do
    routes = [
      %{
        method: :get,
        path: "/search",
        handler: &Handlers.handle_query_params_44_string_minlength_validation_success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "term" => %{"source" => "query", "type" => "string", "minLength" => 3}
          },
          "required" => ["term"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 45_string_minlength_validation_failure
  """
  def create_app_handle_query_params_45_string_minlength_validation_failure() do
    routes = [
      %{
        method: :get,
        path: "/search",
        handler: &Handlers.handle_query_params_45_string_minlength_validation_failure/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "term" => %{"source" => "query", "type" => "string", "minLength" => 3}
          },
          "required" => ["term"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 46_string_maxlength_validation_failure
  """
  def create_app_handle_query_params_46_string_maxlength_validation_failure() do
    routes = [
      %{
        method: :get,
        path: "/search",
        handler: &Handlers.handle_query_params_46_string_maxlength_validation_failure/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "term" => %{"source" => "query", "type" => "string", "maxLength" => 10}
          },
          "required" => ["term"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 47_pattern_validation_email_success
  """
  def create_app_handle_query_params_47_pattern_validation_email_success() do
    routes = [
      %{
        method: :get,
        path: "/subscribe",
        handler: &Handlers.handle_query_params_47_pattern_validation_email_success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "email" => %{
              "source" => "query",
              "type" => "string",
              "pattern" => "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"
            }
          },
          "required" => ["email"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 48_pattern_validation_email_failure
  """
  def create_app_handle_query_params_48_pattern_validation_email_failure() do
    routes = [
      %{
        method: :get,
        path: "/subscribe",
        handler: &Handlers.handle_query_params_48_pattern_validation_email_failure/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "email" => %{
              "source" => "query",
              "type" => "string",
              "pattern" => "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"
            }
          },
          "required" => ["email"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 49_integer_gt_constraint_success
  """
  def create_app_handle_query_params_49_integer_gt_constraint_success() do
    routes = [
      %{
        method: :get,
        path: "/items",
        handler: &Handlers.handle_query_params_49_integer_gt_constraint_success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"limit" => %{"source" => "query", "type" => "integer"}},
          "required" => ["limit"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 50_integer_gt_constraint_failure
  """
  def create_app_handle_query_params_50_integer_gt_constraint_failure() do
    routes = [
      %{
        method: :get,
        path: "/items",
        handler: &Handlers.handle_query_params_50_integer_gt_constraint_failure/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"limit" => %{"source" => "query", "type" => "integer"}},
          "required" => ["limit"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 51_integer_ge_constraint_boundary
  """
  def create_app_handle_query_params_51_integer_ge_constraint_boundary() do
    routes = [
      %{
        method: :get,
        path: "/items",
        handler: &Handlers.handle_query_params_51_integer_ge_constraint_boundary/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "offset" => %{"source" => "query", "type" => "integer", "minimum" => 0}
          },
          "required" => ["offset"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 52_integer_le_constraint_boundary
  """
  def create_app_handle_query_params_52_integer_le_constraint_boundary() do
    routes = [
      %{
        method: :get,
        path: "/items",
        handler: &Handlers.handle_query_params_52_integer_le_constraint_boundary/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "limit" => %{"source" => "query", "type" => "integer", "maximum" => 100}
          },
          "required" => ["limit"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 53_integer_le_constraint_failure
  """
  def create_app_handle_query_params_53_integer_le_constraint_failure() do
    routes = [
      %{
        method: :get,
        path: "/items",
        handler: &Handlers.handle_query_params_53_integer_le_constraint_failure/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "limit" => %{"source" => "query", "type" => "integer", "maximum" => 100}
          },
          "required" => ["limit"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 54_array_minitems_constraint_success
  """
  def create_app_handle_query_params_54_array_minitems_constraint_success() do
    routes = [
      %{
        method: :get,
        path: "/items",
        handler: &Handlers.handle_query_params_54_array_minitems_constraint_success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"ids" => %{"source" => "query", "type" => "array"}},
          "required" => ["ids"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 55_array_minitems_constraint_failure
  """
  def create_app_handle_query_params_55_array_minitems_constraint_failure() do
    routes = [
      %{
        method: :get,
        path: "/items",
        handler: &Handlers.handle_query_params_55_array_minitems_constraint_failure/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"ids" => %{"source" => "query", "type" => "array"}},
          "required" => ["ids"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 56_array_maxitems_constraint_failure
  """
  def create_app_handle_query_params_56_array_maxitems_constraint_failure() do
    routes = [
      %{
        method: :get,
        path: "/items",
        handler: &Handlers.handle_query_params_56_array_maxitems_constraint_failure/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"tags" => %{"source" => "query", "type" => "array"}},
          "required" => ["tags"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 57_boolean_empty_string_coercion
  """
  def create_app_handle_query_params_57_boolean_empty_string_coercion() do
    routes = [
      %{
        method: :get,
        path: "/items",
        handler: &Handlers.handle_query_params_57_boolean_empty_string_coercion/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"active" => %{"source" => "query", "type" => "boolean"}},
          "required" => ["active"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 58_format_email_success
  """
  def create_app_handle_query_params_58_format_email_success() do
    routes = [
      %{
        method: :get,
        path: "/subscribe",
        handler: &Handlers.handle_query_params_58_format_email_success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "email" => %{"source" => "query", "type" => "string", "format" => "email"}
          },
          "required" => ["email"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 59_format_email_failure
  """
  def create_app_handle_query_params_59_format_email_failure() do
    routes = [
      %{
        method: :get,
        path: "/subscribe",
        handler: &Handlers.handle_query_params_59_format_email_failure/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "email" => %{"source" => "query", "type" => "string", "format" => "email"}
          },
          "required" => ["email"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 60_format_ipv4_success
  """
  def create_app_handle_query_params_60_format_ipv4_success() do
    routes = [
      %{
        method: :get,
        path: "/network",
        handler: &Handlers.handle_query_params_60_format_ipv4_success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "ip" => %{"source" => "query", "type" => "string", "format" => "ipv4"}
          },
          "required" => ["ip"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 61_format_ipv4_failure
  """
  def create_app_handle_query_params_61_format_ipv4_failure() do
    routes = [
      %{
        method: :get,
        path: "/network",
        handler: &Handlers.handle_query_params_61_format_ipv4_failure/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "ip" => %{"source" => "query", "type" => "string", "format" => "ipv4"}
          },
          "required" => ["ip"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 62_format_ipv6_success
  """
  def create_app_handle_query_params_62_format_ipv6_success() do
    routes = [
      %{
        method: :get,
        path: "/network/ipv6",
        handler: &Handlers.handle_query_params_62_format_ipv6_success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "ip" => %{"source" => "query", "type" => "string", "format" => "ipv6"}
          },
          "required" => ["ip"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 63_format_uri_success
  """
  def create_app_handle_query_params_63_format_uri_success() do
    routes = [
      %{
        method: :get,
        path: "/redirect",
        handler: &Handlers.handle_query_params_63_format_uri_success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "url" => %{"source" => "query", "type" => "string", "format" => "uri"}
          },
          "required" => ["url"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 64_format_uri_failure
  """
  def create_app_handle_query_params_64_format_uri_failure() do
    routes = [
      %{
        method: :get,
        path: "/redirect",
        handler: &Handlers.handle_query_params_64_format_uri_failure/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "url" => %{"source" => "query", "type" => "string", "format" => "uri"}
          },
          "required" => ["url"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 65_format_hostname_success
  """
  def create_app_handle_query_params_65_format_hostname_success() do
    routes = [
      %{
        method: :get,
        path: "/dns",
        handler: &Handlers.handle_query_params_65_format_hostname_success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "host" => %{"source" => "query", "type" => "string", "format" => "hostname"}
          },
          "required" => ["host"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 66_multipleof_constraint_success
  """
  def create_app_handle_query_params_66_multipleof_constraint_success() do
    routes = [
      %{
        method: :get,
        path: "/items",
        handler: &Handlers.handle_query_params_66_multipleof_constraint_success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"quantity" => %{"source" => "query", "type" => "integer"}},
          "required" => ["quantity"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 67_multipleof_constraint_failure
  """
  def create_app_handle_query_params_67_multipleof_constraint_failure() do
    routes = [
      %{
        method: :get,
        path: "/items",
        handler: &Handlers.handle_query_params_67_multipleof_constraint_failure/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"quantity" => %{"source" => "query", "type" => "integer"}},
          "required" => ["quantity"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 68_array_uniqueitems_success
  """
  def create_app_handle_query_params_68_array_uniqueitems_success() do
    routes = [
      %{
        method: :get,
        path: "/items",
        handler: &Handlers.handle_query_params_68_array_uniqueitems_success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"ids" => %{"source" => "query", "type" => "array"}},
          "required" => ["ids"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 69_array_uniqueitems_failure
  """
  def create_app_handle_query_params_69_array_uniqueitems_failure() do
    routes = [
      %{
        method: :get,
        path: "/items",
        handler: &Handlers.handle_query_params_69_array_uniqueitems_failure/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"ids" => %{"source" => "query", "type" => "array"}},
          "required" => ["ids"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 70_array_separator_pipe
  """
  def create_app_handle_query_params_70_array_separator_pipe() do
    routes = [
      %{
        method: :get,
        path: "/items",
        handler: &Handlers.handle_query_params_70_array_separator_pipe/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"tags" => %{"source" => "query", "type" => "array"}},
          "required" => ["tags"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 71_array_separator_semicolon
  """
  def create_app_handle_query_params_71_array_separator_semicolon() do
    routes = [
      %{
        method: :get,
        path: "/items",
        handler: &Handlers.handle_query_params_71_array_separator_semicolon/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"colors" => %{"source" => "query", "type" => "array"}},
          "required" => ["colors"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - 72_array_separator_space
  """
  def create_app_handle_query_params_72_array_separator_space() do
    routes = [
      %{
        method: :get,
        path: "/search",
        handler: &Handlers.handle_query_params_72_array_separator_space/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"keywords" => %{"source" => "query", "type" => "array"}},
          "required" => ["keywords"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - Array query parameter - empty array
  """
  def create_app_handle_query_params_array_query_parameter___empty_array() do
    routes = [
      %{
        method: :get,
        path: "/query/list-default",
        handler: &Handlers.handle_query_params_array_query_parameter___empty_array/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"tags" => %{"source" => "query", "type" => "array", "default" => []}},
          "required" => []
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - Array query parameter - single value
  """
  def create_app_handle_query_params_array_query_parameter___single_value() do
    routes = [
      %{
        method: :get,
        path: "/query/list-default",
        handler: &Handlers.handle_query_params_array_query_parameter___single_value/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"tags" => %{"source" => "query", "type" => "array", "default" => []}},
          "required" => []
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - Boolean query parameter - numeric 1
  """
  def create_app_handle_query_params_boolean_query_parameter___numeric_1() do
    routes = [
      %{
        method: :get,
        path: "/query/bool",
        handler: &Handlers.handle_query_params_boolean_query_parameter___numeric_1/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"flag" => %{"source" => "query", "type" => "boolean"}},
          "required" => ["flag"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - Boolean query parameter - true
  """
  def create_app_handle_query_params_boolean_query_parameter___true() do
    routes = [
      %{
        method: :get,
        path: "/query/bool",
        handler: &Handlers.handle_query_params_boolean_query_parameter___true/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"flag" => %{"source" => "query", "type" => "boolean"}},
          "required" => ["flag"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - Date query parameter - success
  """
  def create_app_handle_query_params_date_query_parameter___success() do
    routes = [
      %{
        method: :get,
        path: "/query/date",
        handler: &Handlers.handle_query_params_date_query_parameter___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "event_date" => %{"source" => "query", "type" => "string", "format" => "date"}
          },
          "required" => ["event_date"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - Datetime query parameter - success
  """
  def create_app_handle_query_params_datetime_query_parameter___success() do
    routes = [
      %{
        method: :get,
        path: "/query/datetime",
        handler: &Handlers.handle_query_params_datetime_query_parameter___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "timestamp" => %{"source" => "query", "type" => "string", "format" => "date-time"}
          },
          "required" => ["timestamp"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - Enum query parameter - invalid value
  """
  def create_app_handle_query_params_enum_query_parameter___invalid_value() do
    routes = [
      %{
        method: :get,
        path: "/query/enum",
        handler: &Handlers.handle_query_params_enum_query_parameter___invalid_value/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "model" => %{
              "source" => "query",
              "type" => "string",
              "enum" => ["alexnet", "resnet", "lenet"]
            }
          },
          "required" => ["model"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - Enum query parameter - success
  """
  def create_app_handle_query_params_enum_query_parameter___success() do
    routes = [
      %{
        method: :get,
        path: "/query/enum",
        handler: &Handlers.handle_query_params_enum_query_parameter___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "model" => %{
              "source" => "query",
              "type" => "string",
              "enum" => ["alexnet", "resnet", "lenet"]
            }
          },
          "required" => ["model"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - Float query param with ge constraint - success
  """
  def create_app_handle_query_params_float_query_param_with_ge_constraint___success() do
    routes = [
      %{
        method: :get,
        path: "/query/float-ge",
        handler: &Handlers.handle_query_params_float_query_param_with_ge_constraint___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"price" => %{"source" => "query", "type" => "number"}},
          "required" => ["price"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - Integer query param with ge constraint - boundary
  """
  def create_app_handle_query_params_integer_query_param_with_ge_constraint___boundary() do
    routes = [
      %{
        method: :get,
        path: "/query/int-ge",
        handler:
          &Handlers.handle_query_params_integer_query_param_with_ge_constraint___boundary/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "value" => %{"source" => "query", "type" => "integer", "minimum" => 10}
          },
          "required" => ["value"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - Integer query param with gt constraint - valid
  """
  def create_app_handle_query_params_integer_query_param_with_gt_constraint___valid() do
    routes = [
      %{
        method: :get,
        path: "/query/int-gt",
        handler: &Handlers.handle_query_params_integer_query_param_with_gt_constraint___valid/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"value" => %{"source" => "query", "type" => "integer"}},
          "required" => ["value"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - Integer query param with le constraint - boundary
  """
  def create_app_handle_query_params_integer_query_param_with_le_constraint___boundary() do
    routes = [
      %{
        method: :get,
        path: "/query/int-le",
        handler:
          &Handlers.handle_query_params_integer_query_param_with_le_constraint___boundary/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "value" => %{"source" => "query", "type" => "integer", "maximum" => 100}
          },
          "required" => ["value"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - Integer query param with lt constraint - valid
  """
  def create_app_handle_query_params_integer_query_param_with_lt_constraint___valid() do
    routes = [
      %{
        method: :get,
        path: "/query/int-lt",
        handler: &Handlers.handle_query_params_integer_query_param_with_lt_constraint___valid/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"value" => %{"source" => "query", "type" => "integer"}},
          "required" => ["value"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - Integer with default value - not provided
  """
  def create_app_handle_query_params_integer_with_default_value___not_provided() do
    routes = [
      %{
        method: :get,
        path: "/query/int/default",
        handler: &Handlers.handle_query_params_integer_with_default_value___not_provided/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "query" => %{"source" => "query", "type" => "integer", "default" => 10}
          },
          "required" => []
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - Integer with default value - override
  """
  def create_app_handle_query_params_integer_with_default_value___override() do
    routes = [
      %{
        method: :get,
        path: "/query/int/default",
        handler: &Handlers.handle_query_params_integer_with_default_value___override/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "query" => %{"source" => "query", "type" => "integer", "default" => 10}
          },
          "required" => []
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - List of integers - multiple values
  """
  def create_app_handle_query_params_list_of_integers___multiple_values() do
    routes = [
      %{
        method: :get,
        path: "/query/list",
        handler: &Handlers.handle_query_params_list_of_integers___multiple_values/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"device_ids" => %{"source" => "query", "type" => "array"}},
          "required" => ["device_ids"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - List of strings - multiple values
  """
  def create_app_handle_query_params_list_of_strings___multiple_values() do
    routes = [
      %{
        method: :get,
        path: "/items/",
        handler: &Handlers.handle_query_params_list_of_strings___multiple_values/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"q" => %{"source" => "query", "type" => "array"}},
          "required" => []
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - List query parameter - required but missing
  """
  def create_app_handle_query_params_list_query_parameter___required_but_missing() do
    routes = [
      %{
        method: :get,
        path: "/query/list",
        handler: &Handlers.handle_query_params_list_query_parameter___required_but_missing/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"device_ids" => %{"source" => "query", "type" => "array"}},
          "required" => ["device_ids"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - List with default empty array - no values provided
  """
  def create_app_handle_query_params_list_with_default_empty_array___no_values_provided() do
    routes = [
      %{
        method: :get,
        path: "/query/list-default",
        handler:
          &Handlers.handle_query_params_list_with_default_empty_array___no_values_provided/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"tags" => %{"source" => "query", "type" => "array", "default" => []}},
          "required" => []
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - Multiple query parameters with different types
  """
  def create_app_handle_query_params_multiple_query_parameters_with_different_types() do
    routes = [
      %{
        method: :get,
        path: "/query/multi-type",
        handler: &Handlers.handle_query_params_multiple_query_parameters_with_different_types/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"source" => "query", "type" => "string"},
            "age" => %{"source" => "query", "type" => "integer"},
            "active" => %{"source" => "query", "type" => "boolean"},
            "score" => %{"source" => "query", "type" => "number"}
          },
          "required" => ["name", "age", "active", "score"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - Optional integer query parameter - missing
  """
  def create_app_handle_query_params_optional_integer_query_parameter___missing() do
    routes = [
      %{
        method: :get,
        path: "/query/int/optional",
        handler: &Handlers.handle_query_params_optional_integer_query_parameter___missing/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"query" => %{"source" => "query", "type" => "integer"}},
          "required" => []
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - Optional query parameter with default value
  """
  def create_app_handle_query_params_optional_query_parameter_with_default_value() do
    routes = [
      %{
        method: :get,
        path: "/query/optional-default",
        handler: &Handlers.handle_query_params_optional_query_parameter_with_default_value/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "limit" => %{"source" => "query", "type" => "integer", "default" => 10}
          },
          "required" => []
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - Optional string query parameter - missing
  """
  def create_app_handle_query_params_optional_string_query_parameter___missing() do
    routes = [
      %{
        method: :get,
        path: "/query/optional",
        handler: &Handlers.handle_query_params_optional_string_query_parameter___missing/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"query" => %{"source" => "query", "type" => "string"}},
          "required" => []
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - Optional string query parameter - provided
  """
  def create_app_handle_query_params_optional_string_query_parameter___provided() do
    routes = [
      %{
        method: :get,
        path: "/query/optional",
        handler: &Handlers.handle_query_params_optional_string_query_parameter___provided/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"query" => %{"source" => "query", "type" => "string"}},
          "required" => []
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - Query parameter with URL encoded space
  """
  def create_app_handle_query_params_query_parameter_with_url_encoded_space() do
    routes = [
      %{
        method: :get,
        path: "/query/basic",
        handler: &Handlers.handle_query_params_query_parameter_with_url_encoded_space/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"name" => %{"source" => "query", "type" => "string"}},
          "required" => ["name"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - Query parameter with URL encoded special characters
  """
  def create_app_handle_query_params_query_parameter_with_url_encoded_special_characters() do
    routes = [
      %{
        method: :get,
        path: "/query/basic",
        handler:
          &Handlers.handle_query_params_query_parameter_with_url_encoded_special_characters/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"name" => %{"source" => "query", "type" => "string"}},
          "required" => ["name"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - Query parameter with special characters - URL encoding
  """
  def create_app_handle_query_params_query_parameter_with_special_characters___url_encoding() do
    routes = [
      %{
        method: :get,
        path: "/test",
        handler:
          &Handlers.handle_query_params_query_parameter_with_special_characters___url_encoding/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "email" => %{"source" => "query", "type" => "string"},
            "special" => %{"source" => "query", "type" => "string"}
          },
          "required" => ["email", "special"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - Required integer query parameter - float value
  """
  def create_app_handle_query_params_required_integer_query_parameter___float_value() do
    routes = [
      %{
        method: :get,
        path: "/query/int",
        handler: &Handlers.handle_query_params_required_integer_query_parameter___float_value/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"query" => %{"source" => "query", "type" => "integer"}},
          "required" => ["query"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - Required integer query parameter - invalid type
  """
  def create_app_handle_query_params_required_integer_query_parameter___invalid_type() do
    routes = [
      %{
        method: :get,
        path: "/query/int",
        handler: &Handlers.handle_query_params_required_integer_query_parameter___invalid_type/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"query" => %{"source" => "query", "type" => "integer"}},
          "required" => ["query"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - Required integer query parameter - missing
  """
  def create_app_handle_query_params_required_integer_query_parameter___missing() do
    routes = [
      %{
        method: :get,
        path: "/query/int",
        handler: &Handlers.handle_query_params_required_integer_query_parameter___missing/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"query" => %{"source" => "query", "type" => "integer"}},
          "required" => ["query"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - Required integer query parameter - success
  """
  def create_app_handle_query_params_required_integer_query_parameter___success() do
    routes = [
      %{
        method: :get,
        path: "/query/int",
        handler: &Handlers.handle_query_params_required_integer_query_parameter___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"query" => %{"source" => "query", "type" => "integer"}},
          "required" => ["query"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - Required string query parameter - missing
  """
  def create_app_handle_query_params_required_string_query_parameter___missing() do
    routes = [
      %{
        method: :get,
        path: "/query",
        handler: &Handlers.handle_query_params_required_string_query_parameter___missing/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"query" => %{"source" => "query", "type" => "string"}},
          "required" => ["query"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - Required string query parameter - success
  """
  def create_app_handle_query_params_required_string_query_parameter___success() do
    routes = [
      %{
        method: :get,
        path: "/query",
        handler: &Handlers.handle_query_params_required_string_query_parameter___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"query" => %{"source" => "query", "type" => "string"}},
          "required" => ["query"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - String query param with max_length constraint - fail
  """
  def create_app_handle_query_params_string_query_param_with_max_length_constraint___fail() do
    routes = [
      %{
        method: :get,
        path: "/query/str-max-length",
        handler:
          &Handlers.handle_query_params_string_query_param_with_max_length_constraint___fail/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"source" => "query", "type" => "string", "maxLength" => 10}
          },
          "required" => ["name"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - String query param with min_length constraint - fail
  """
  def create_app_handle_query_params_string_query_param_with_min_length_constraint___fail() do
    routes = [
      %{
        method: :get,
        path: "/query/str-min-length",
        handler:
          &Handlers.handle_query_params_string_query_param_with_min_length_constraint___fail/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"source" => "query", "type" => "string", "minLength" => 3}
          },
          "required" => ["name"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - String query param with regex pattern - fail
  """
  def create_app_handle_query_params_string_query_param_with_regex_pattern___fail() do
    routes = [
      %{
        method: :get,
        path: "/query/pattern",
        handler: &Handlers.handle_query_params_string_query_param_with_regex_pattern___fail/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "code" => %{"source" => "query", "type" => "string", "pattern" => "^[0-9]{3,}$"}
          },
          "required" => ["code"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - String validation with regex - failure
  """
  def create_app_handle_query_params_string_validation_with_regex___failure() do
    routes = [
      %{
        method: :get,
        path: "/items/",
        handler: &Handlers.handle_query_params_string_validation_with_regex___failure/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "item_query" => %{
              "source" => "query",
              "type" => "string",
              "pattern" => "^fixedquery$"
            }
          },
          "required" => ["item_query"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - String validation with regex - success
  """
  def create_app_handle_query_params_string_validation_with_regex___success() do
    routes = [
      %{
        method: :get,
        path: "/items/",
        handler: &Handlers.handle_query_params_string_validation_with_regex___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "item_query" => %{
              "source" => "query",
              "type" => "string",
              "pattern" => "^fixedquery$"
            }
          },
          "required" => ["item_query"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - UUID query parameter - invalid format
  """
  def create_app_handle_query_params_uuid_query_parameter___invalid_format() do
    routes = [
      %{
        method: :get,
        path: "/query/uuid",
        handler: &Handlers.handle_query_params_uuid_query_parameter___invalid_format/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "item_id" => %{"source" => "query", "type" => "string", "format" => "uuid"}
          },
          "required" => ["item_id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: query_params - UUID query parameter - success
  """
  def create_app_handle_query_params_uuid_query_parameter___success() do
    routes = [
      %{
        method: :get,
        path: "/query/uuid",
        handler: &Handlers.handle_query_params_uuid_query_parameter___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "item_id" => %{"source" => "query", "type" => "string", "format" => "uuid"}
          },
          "required" => ["item_id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: rate_limit - Rate limit below threshold succeeds
  """
  def create_app_handle_rate_limit_rate_limit_below_threshold_succeeds() do
    routes = [
      %{
        method: :get,
        path: "/rate-limit/basic",
        handler: &Handlers.handle_rate_limit_rate_limit_below_threshold_succeeds/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: rate_limit - Rate limit exceeded returns 429
  """
  def create_app_handle_rate_limit_rate_limit_exceeded_returns_429() do
    routes = [
      %{
        method: :get,
        path: "/rate-limit/exceeded",
        handler: &Handlers.handle_rate_limit_rate_limit_exceeded_returns_429/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: request_id - Request ID header is preserved
  """
  def create_app_handle_request_id_request_id_header_is_preserved() do
    routes = [
      %{
        method: :get,
        path: "/request-id/preserved",
        handler: &Handlers.handle_request_id_request_id_header_is_preserved/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: request_id - Request ID is generated when not provided
  """
  def create_app_handle_request_id_request_id_is_generated_when_not_provided() do
    routes = [
      %{
        method: :get,
        path: "/request-id/generated",
        handler: &Handlers.handle_request_id_request_id_is_generated_when_not_provided/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: request_id - Request ID middleware can be disabled
  """
  def create_app_handle_request_id_request_id_middleware_can_be_disabled() do
    routes = [
      %{
        method: :get,
        path: "/request-id/disabled",
        handler: &Handlers.handle_request_id_request_id_middleware_can_be_disabled/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: request_timeout - Request completes before timeout
  """
  def create_app_handle_request_timeout_request_completes_before_timeout() do
    routes = [
      %{
        method: :get,
        path: "/timeouts/fast",
        handler: &Handlers.handle_request_timeout_request_completes_before_timeout/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: request_timeout - Request exceeds timeout
  """
  def create_app_handle_request_timeout_request_exceeds_timeout() do
    routes = [
      %{
        method: :get,
        path: "/timeouts/slow",
        handler: &Handlers.handle_request_timeout_request_exceeds_timeout/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: static_files - Static file server returns text file
  """
  def create_app_handle_static_files_static_file_server_returns_text_file() do
    routes = []

    config = %{
      static_files: [
        %{
          directory:
            "lib/e2e_elixir_app/static_assets/static_files_static_file_server_returns_text_file/public_0",
          route_prefix: "/public",
          cache_control: "public, max-age=60"
        }
      ]
    }

    {routes, config}
  end

  @doc """
  App factory for fixture: static_files - Static server returns index.html for directory
  """
  def create_app_handle_static_files_static_server_returns_index_html_for_directory() do
    routes = []

    config = %{
      static_files: [
        %{
          directory:
            "lib/e2e_elixir_app/static_assets/static_files_static_server_returns_index_html_for_directory/app_0",
          route_prefix: "/app"
        }
      ]
    }

    {routes, config}
  end

  @doc """
  App factory for fixture: status_codes - 19_413_payload_too_large
  """
  def create_app_handle_status_codes_19_413_payload_too_large() do
    routes = [
      %{
        method: :post,
        path: "/upload",
        handler: &Handlers.handle_status_codes_19_413_payload_too_large/1,
        request_schema: %{"type" => "object", "properties" => %{"data" => %{"type" => "string"}}}
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: status_codes - 200 OK - Success
  """
  def create_app_handle_status_codes_200_ok___success() do
    routes = [
      %{
        method: :get,
        path: "/status-test/{code}",
        handler: &Handlers.handle_status_codes_200_ok___success/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"code" => %{"source" => "path", "type" => "string"}},
          "required" => ["code"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: status_codes - 201 Created - Resource created
  """
  def create_app_handle_status_codes_201_created___resource_created() do
    routes = [
      %{
        method: :post,
        path: "/items/",
        handler: &Handlers.handle_status_codes_201_created___resource_created/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"name" => %{"type" => "string"}},
          "additionalProperties" => false,
          "required" => ["name"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: status_codes - 202 Accepted - Request accepted for processing
  """
  def create_app_handle_status_codes_202_accepted___request_accepted_for_processing() do
    routes = [
      %{
        method: :post,
        path: "/tasks/",
        handler: &Handlers.handle_status_codes_202_accepted___request_accepted_for_processing/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"task" => %{"type" => "string"}},
          "additionalProperties" => false,
          "required" => ["task"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: status_codes - 204 No Content - Success with no body
  """
  def create_app_handle_status_codes_204_no_content___success_with_no_body() do
    routes = [
      %{
        method: :delete,
        path: "/status-test/{code}",
        handler: &Handlers.handle_status_codes_204_no_content___success_with_no_body/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"code" => %{"source" => "path", "type" => "string"}},
          "required" => ["code"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: status_codes - 206 Partial Content
  """
  def create_app_handle_status_codes_206_partial_content() do
    routes = [
      %{
        method: :get,
        path: "/files/document.pdf",
        handler: &Handlers.handle_status_codes_206_partial_content/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: status_codes - 20_414_uri_too_long
  """
  def create_app_handle_status_codes_20_414_uri_too_long() do
    routes = [
      %{method: :get, path: "/data", handler: &Handlers.handle_status_codes_20_414_uri_too_long/1}
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: status_codes - 21_431_request_header_fields_too_large
  """
  def create_app_handle_status_codes_21_431_request_header_fields_too_large() do
    routes = [
      %{
        method: :get,
        path: "/data",
        handler: &Handlers.handle_status_codes_21_431_request_header_fields_too_large/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"X-Large-Header" => %{"source" => "header", "type" => "string"}},
          "required" => []
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: status_codes - 22_501_not_implemented
  """
  def create_app_handle_status_codes_22_501_not_implemented() do
    routes = [
      %{
        method: :trace,
        path: "/data",
        handler: &Handlers.handle_status_codes_22_501_not_implemented/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: status_codes - 23_503_service_unavailable
  """
  def create_app_handle_status_codes_23_503_service_unavailable() do
    routes = [
      %{
        method: :get,
        path: "/data",
        handler: &Handlers.handle_status_codes_23_503_service_unavailable/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: status_codes - 301 Moved Permanently - Permanent redirect
  """
  def create_app_handle_status_codes_301_moved_permanently___permanent_redirect() do
    routes = [
      %{
        method: :get,
        path: "/old-path",
        handler: &Handlers.handle_status_codes_301_moved_permanently___permanent_redirect/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: status_codes - 302 Found - Temporary redirect
  """
  def create_app_handle_status_codes_302_found___temporary_redirect() do
    routes = [
      %{
        method: :get,
        path: "/temp-redirect",
        handler: &Handlers.handle_status_codes_302_found___temporary_redirect/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: status_codes - 304 Not Modified - Cached content valid
  """
  def create_app_handle_status_codes_304_not_modified___cached_content_valid() do
    routes = [
      %{
        method: :get,
        path: "/status-test/{code}",
        handler: &Handlers.handle_status_codes_304_not_modified___cached_content_valid/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "code" => %{"source" => "path", "type" => "string"},
            "If-None-Match" => %{"source" => "header", "type" => "string"}
          },
          "required" => ["code"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: status_codes - 307 Temporary Redirect - Method preserved
  """
  def create_app_handle_status_codes_307_temporary_redirect___method_preserved() do
    routes = [
      %{
        method: :post,
        path: "/redirect-post",
        handler: &Handlers.handle_status_codes_307_temporary_redirect___method_preserved/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{},
          "additionalProperties" => false
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: status_codes - 400 Bad Request - Invalid request
  """
  def create_app_handle_status_codes_400_bad_request___invalid_request() do
    routes = [
      %{
        method: :post,
        path: "/items/",
        handler: &Handlers.handle_status_codes_400_bad_request___invalid_request/1,
        request_schema: %{"type" => "string"}
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: status_codes - 401 Unauthorized - Missing authentication
  """
  def create_app_handle_status_codes_401_unauthorized___missing_authentication() do
    routes = [
      %{
        method: :get,
        path: "/users/me",
        handler: &Handlers.handle_status_codes_401_unauthorized___missing_authentication/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: status_codes - 403 Forbidden - Insufficient permissions
  """
  def create_app_handle_status_codes_403_forbidden___insufficient_permissions() do
    routes = [
      %{
        method: :get,
        path: "/admin/users",
        handler: &Handlers.handle_status_codes_403_forbidden___insufficient_permissions/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: status_codes - 404 Not Found - Resource not found
  """
  def create_app_handle_status_codes_404_not_found___resource_not_found() do
    routes = [
      %{
        method: :get,
        path: "/status-test/{code}",
        handler: &Handlers.handle_status_codes_404_not_found___resource_not_found/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"code" => %{"source" => "path", "type" => "string"}},
          "required" => ["code"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: status_codes - 408 Request Timeout
  """
  def create_app_handle_status_codes_408_request_timeout() do
    routes = [
      %{
        method: :post,
        path: "/slow-endpoint",
        handler: &Handlers.handle_status_codes_408_request_timeout/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"data" => %{"type" => "string"}},
          "additionalProperties" => false,
          "required" => ["data"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: status_codes - 422 Unprocessable Entity - Validation error
  """
  def create_app_handle_status_codes_422_unprocessable_entity___validation_error() do
    routes = [
      %{
        method: :post,
        path: "/items/",
        handler: &Handlers.handle_status_codes_422_unprocessable_entity___validation_error/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"price" => %{"type" => "string"}, "name" => %{"type" => "string"}},
          "additionalProperties" => false,
          "required" => ["price", "name"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: status_codes - 429 Too Many Requests
  """
  def create_app_handle_status_codes_429_too_many_requests() do
    routes = [
      %{
        method: :get,
        path: "/api/resource",
        handler: &Handlers.handle_status_codes_429_too_many_requests/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: status_codes - 500 Internal Server Error - Server error
  """
  def create_app_handle_status_codes_500_internal_server_error___server_error() do
    routes = [
      %{
        method: :get,
        path: "/error",
        handler: &Handlers.handle_status_codes_500_internal_server_error___server_error/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: status_codes - 503 Service Unavailable - Server overload
  """
  def create_app_handle_status_codes_503_service_unavailable___server_overload() do
    routes = [
      %{
        method: :get,
        path: "/health",
        handler: &Handlers.handle_status_codes_503_service_unavailable___server_overload/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: streaming - Binary log download
  """
  def create_app_handle_streaming_binary_log_download() do
    routes = [
      %{
        method: :get,
        path: "/stream/logfile",
        handler: &Handlers.handle_streaming_binary_log_download/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: streaming - Chunked CSV export
  """
  def create_app_handle_streaming_chunked_csv_export() do
    routes = [
      %{
        method: :get,
        path: "/stream/csv-report",
        handler: &Handlers.handle_streaming_chunked_csv_export/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: streaming - Stream JSON lines
  """
  def create_app_handle_streaming_stream_json_lines() do
    routes = [
      %{
        method: :get,
        path: "/stream/json-lines",
        handler: &Handlers.handle_streaming_stream_json_lines/1
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: url_encoded - 13_array_field_success
  """
  def create_app_handle_url_encoded_13_array_field_success() do
    routes = [
      %{
        method: :post,
        path: "/register",
        handler: &Handlers.handle_url_encoded_13_array_field_success/1,
        request_schema: %{
          "type" => "object",
          "required" => ["tags"],
          "properties" => %{
            "tags" => %{"type" => "array", "items" => %{"type" => "string"}, "minItems" => 1}
          }
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: url_encoded - 14_nested_object_bracket_notation
  """
  def create_app_handle_url_encoded_14_nested_object_bracket_notation() do
    routes = [
      %{
        method: :post,
        path: "/profile",
        handler: &Handlers.handle_url_encoded_14_nested_object_bracket_notation/1,
        request_schema: %{
          "type" => "object",
          "required" => ["user"],
          "properties" => %{
            "user" => %{
              "type" => "object",
              "required" => ["name", "email"],
              "properties" => %{
                "name" => %{"type" => "string", "minLength" => 1},
                "email" => %{"type" => "string", "format" => "email"},
                "age" => %{"type" => "integer", "minimum" => 0}
              }
            }
          }
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: url_encoded - 15_special_characters_field_names
  """
  def create_app_handle_url_encoded_15_special_characters_field_names() do
    routes = [
      %{
        method: :post,
        path: "/data",
        handler: &Handlers.handle_url_encoded_15_special_characters_field_names/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "user-name" => %{"type" => "string"},
            "contact.email" => %{"type" => "string", "format" => "email"}
          }
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: url_encoded - 16_minlength_validation_failure
  """
  def create_app_handle_url_encoded_16_minlength_validation_failure() do
    routes = [
      %{
        method: :post,
        path: "/users",
        handler: &Handlers.handle_url_encoded_16_minlength_validation_failure/1,
        request_schema: %{
          "type" => "object",
          "required" => ["username"],
          "properties" => %{"username" => %{"type" => "string", "minLength" => 3}}
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: url_encoded - 17_pattern_validation_failure
  """
  def create_app_handle_url_encoded_17_pattern_validation_failure() do
    routes = [
      %{
        method: :post,
        path: "/accounts",
        handler: &Handlers.handle_url_encoded_17_pattern_validation_failure/1,
        request_schema: %{
          "type" => "object",
          "required" => ["account_id"],
          "properties" => %{"account_id" => %{"type" => "string", "pattern" => "^ACC-[0-9]{6}$"}}
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: url_encoded - 18_integer_minimum_validation_failure
  """
  def create_app_handle_url_encoded_18_integer_minimum_validation_failure() do
    routes = [
      %{
        method: :post,
        path: "/products",
        handler: &Handlers.handle_url_encoded_18_integer_minimum_validation_failure/1,
        request_schema: %{
          "type" => "object",
          "required" => ["quantity"],
          "properties" => %{"quantity" => %{"type" => "integer", "minimum" => 1}}
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: url_encoded - 19_array_minitems_validation_failure
  """
  def create_app_handle_url_encoded_19_array_minitems_validation_failure() do
    routes = [
      %{
        method: :post,
        path: "/tags",
        handler: &Handlers.handle_url_encoded_19_array_minitems_validation_failure/1,
        request_schema: %{
          "type" => "object",
          "required" => ["tags"],
          "properties" => %{
            "tags" => %{"type" => "array", "items" => %{"type" => "string"}, "minItems" => 2}
          }
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: url_encoded - 20_format_email_validation_failure
  """
  def create_app_handle_url_encoded_20_format_email_validation_failure() do
    routes = [
      %{
        method: :post,
        path: "/subscribe",
        handler: &Handlers.handle_url_encoded_20_format_email_validation_failure/1,
        request_schema: %{
          "type" => "object",
          "required" => ["email"],
          "properties" => %{"email" => %{"type" => "string", "format" => "email"}}
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: url_encoded - 21_integer_type_coercion_failure
  """
  def create_app_handle_url_encoded_21_integer_type_coercion_failure() do
    routes = [
      %{
        method: :post,
        path: "/products",
        handler: &Handlers.handle_url_encoded_21_integer_type_coercion_failure/1,
        request_schema: %{
          "type" => "object",
          "required" => ["price"],
          "properties" => %{"price" => %{"type" => "integer"}}
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: url_encoded - 22_additional_properties_strict_failure
  """
  def create_app_handle_url_encoded_22_additional_properties_strict_failure() do
    routes = [
      %{
        method: :post,
        path: "/settings",
        handler: &Handlers.handle_url_encoded_22_additional_properties_strict_failure/1,
        request_schema: %{
          "type" => "object",
          "required" => ["theme"],
          "properties" => %{"theme" => %{"type" => "string", "enum" => ["light", "dark"]}},
          "additionalProperties" => false
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: url_encoded - Boolean field conversion
  """
  def create_app_handle_url_encoded_boolean_field_conversion() do
    routes = [
      %{
        method: :post,
        path: "/form/",
        handler: &Handlers.handle_url_encoded_boolean_field_conversion/1,
        request_schema: %{
          "type" => "object",
          "required" => ["username"],
          "properties" => %{
            "username" => %{"type" => "string"},
            "subscribe" => %{"type" => "boolean"}
          }
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: url_encoded - Empty string value
  """
  def create_app_handle_url_encoded_empty_string_value() do
    routes = [
      %{
        method: :post,
        path: "/form/",
        handler: &Handlers.handle_url_encoded_empty_string_value/1,
        request_schema: %{
          "type" => "object",
          "required" => ["username"],
          "properties" => %{
            "username" => %{"type" => "string"},
            "description" => %{"type" => "string"}
          }
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: url_encoded - Multiple values for same field
  """
  def create_app_handle_url_encoded_multiple_values_for_same_field() do
    routes = [
      %{
        method: :post,
        path: "/form/tags",
        handler: &Handlers.handle_url_encoded_multiple_values_for_same_field/1,
        request_schema: %{
          "type" => "object",
          "required" => ["tags"],
          "properties" => %{"tags" => %{"type" => "array", "items" => %{"type" => "string"}}}
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: url_encoded - Numeric field type conversion
  """
  def create_app_handle_url_encoded_numeric_field_type_conversion() do
    routes = [
      %{
        method: :post,
        path: "/form/",
        handler: &Handlers.handle_url_encoded_numeric_field_type_conversion/1,
        request_schema: %{
          "type" => "object",
          "required" => ["username"],
          "properties" => %{"username" => %{"type" => "string"}, "age" => %{"type" => "integer"}}
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: url_encoded - OAuth2 password grant flow
  """
  def create_app_handle_url_encoded_oauth2_password_grant_flow() do
    routes = [
      %{
        method: :post,
        path: "/token",
        handler: &Handlers.handle_url_encoded_oauth2_password_grant_flow/1,
        request_schema: %{
          "type" => "object",
          "required" => ["username", "password", "grant_type"],
          "properties" => %{
            "username" => %{"type" => "string"},
            "password" => %{"type" => "string"},
            "grant_type" => %{"type" => "string"},
            "scope" => %{"type" => "string"}
          }
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: url_encoded - Optional field missing - success
  """
  def create_app_handle_url_encoded_optional_field_missing___success() do
    routes = [
      %{
        method: :post,
        path: "/register/",
        handler: &Handlers.handle_url_encoded_optional_field_missing___success/1,
        request_schema: %{
          "type" => "object",
          "required" => ["username", "password"],
          "properties" => %{
            "username" => %{"type" => "string"},
            "password" => %{"type" => "string"},
            "email" => %{"type" => ["string", "null"], "format" => "email"}
          }
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: url_encoded - Pattern validation - fail
  """
  def create_app_handle_url_encoded_pattern_validation___fail() do
    routes = [
      %{
        method: :post,
        path: "/form/validated",
        handler: &Handlers.handle_url_encoded_pattern_validation___fail/1,
        request_schema: %{
          "type" => "object",
          "required" => ["username"],
          "properties" => %{"username" => %{"type" => "string", "pattern" => "^[a-z0-9_]+$"}}
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: url_encoded - Required field missing - validation error
  """
  def create_app_handle_url_encoded_required_field_missing___validation_error() do
    routes = [
      %{
        method: :post,
        path: "/login/",
        handler: &Handlers.handle_url_encoded_required_field_missing___validation_error/1,
        request_schema: %{
          "type" => "object",
          "required" => ["username", "password"],
          "properties" => %{
            "username" => %{"type" => "string"},
            "password" => %{"type" => "string"}
          }
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: url_encoded - Simple form submission - success
  """
  def create_app_handle_url_encoded_simple_form_submission___success() do
    routes = [
      %{
        method: :post,
        path: "/login/",
        handler: &Handlers.handle_url_encoded_simple_form_submission___success/1,
        request_schema: %{
          "type" => "object",
          "required" => ["username", "password"],
          "properties" => %{
            "username" => %{"type" => "string"},
            "password" => %{"type" => "string"}
          }
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: url_encoded - Special characters encoding
  """
  def create_app_handle_url_encoded_special_characters_encoding() do
    routes = [
      %{
        method: :post,
        path: "/form/",
        handler: &Handlers.handle_url_encoded_special_characters_encoding/1,
        request_schema: %{
          "type" => "object",
          "required" => ["name"],
          "properties" => %{
            "name" => %{"type" => "string"},
            "description" => %{"type" => "string"}
          }
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: url_encoded - String max_length validation - fail
  """
  def create_app_handle_url_encoded_string_max_length_validation___fail() do
    routes = [
      %{
        method: :post,
        path: "/form/validated",
        handler: &Handlers.handle_url_encoded_string_max_length_validation___fail/1,
        request_schema: %{
          "type" => "object",
          "required" => ["username"],
          "properties" => %{"username" => %{"type" => "string", "maxLength" => 20}}
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: url_encoded - String min_length validation - fail
  """
  def create_app_handle_url_encoded_string_min_length_validation___fail() do
    routes = [
      %{
        method: :post,
        path: "/form/validated",
        handler: &Handlers.handle_url_encoded_string_min_length_validation___fail/1,
        request_schema: %{
          "type" => "object",
          "required" => ["username"],
          "properties" => %{"username" => %{"type" => "string", "minLength" => 3}}
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: validation_errors - 09_multiple_validation_errors
  """
  def create_app_handle_validation_errors_09_multiple_validation_errors() do
    routes = [
      %{
        method: :post,
        path: "/users",
        handler: &Handlers.handle_validation_errors_09_multiple_validation_errors/1,
        request_schema: %{
          "type" => "object",
          "required" => ["name", "email", "age"],
          "properties" => %{
            "name" => %{"type" => "string", "minLength" => 3},
            "email" => %{"type" => "string", "format" => "email"},
            "age" => %{"type" => "integer", "minimum" => 18}
          }
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: validation_errors - 10_nested_error_path
  """
  def create_app_handle_validation_errors_10_nested_error_path() do
    routes = [
      %{
        method: :post,
        path: "/profiles",
        handler: &Handlers.handle_validation_errors_10_nested_error_path/1,
        request_schema: %{
          "type" => "object",
          "required" => ["profile"],
          "properties" => %{
            "profile" => %{
              "type" => "object",
              "required" => ["contact"],
              "properties" => %{
                "contact" => %{
                  "type" => "object",
                  "required" => ["email"],
                  "properties" => %{"email" => %{"type" => "string", "format" => "email"}}
                }
              }
            }
          }
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: validation_errors - Array item validation error
  """
  def create_app_handle_validation_errors_array_item_validation_error() do
    routes = [
      %{
        method: :post,
        path: "/items/",
        handler: &Handlers.handle_validation_errors_array_item_validation_error/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"type" => "string"},
            "price" => %{"type" => "number"},
            "tags" => %{"type" => "array", "items" => %{"type" => "string"}}
          },
          "additionalProperties" => false,
          "required" => ["name", "price", "tags"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: validation_errors - Array max_items constraint violation
  """
  def create_app_handle_validation_errors_array_max_items_constraint_violation() do
    routes = [
      %{
        method: :post,
        path: "/items/",
        handler: &Handlers.handle_validation_errors_array_max_items_constraint_violation/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"type" => "string"},
            "price" => %{"type" => "number"},
            "tags" => %{"type" => "array", "items" => %{"type" => "string"}, "maxItems" => 10}
          },
          "additionalProperties" => false,
          "required" => ["name", "price", "tags"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: validation_errors - Array min_items constraint violation
  """
  def create_app_handle_validation_errors_array_min_items_constraint_violation() do
    routes = [
      %{
        method: :post,
        path: "/items/",
        handler: &Handlers.handle_validation_errors_array_min_items_constraint_violation/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"type" => "string"},
            "price" => %{"type" => "number"},
            "tags" => %{"type" => "array", "items" => %{}, "minItems" => 1}
          },
          "additionalProperties" => false,
          "required" => ["name", "price", "tags"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: validation_errors - Body field type error - string for float
  """
  def create_app_handle_validation_errors_body_field_type_error___string_for_float() do
    routes = [
      %{
        method: :post,
        path: "/items/",
        handler: &Handlers.handle_validation_errors_body_field_type_error___string_for_float/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"name" => %{"type" => "string"}, "price" => %{"type" => "number"}},
          "additionalProperties" => false,
          "required" => ["name", "price"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: validation_errors - Header validation error
  """
  def create_app_handle_validation_errors_header_validation_error() do
    routes = [
      %{
        method: :get,
        path: "/items/",
        handler: &Handlers.handle_validation_errors_header_validation_error/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "q" => %{"source" => "query", "type" => "string"},
            "x-token" => %{"source" => "header", "type" => "string"}
          },
          "required" => ["q", "x-token"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: validation_errors - Invalid UUID format
  """
  def create_app_handle_validation_errors_invalid_uuid_format() do
    routes = [
      %{
        method: :get,
        path: "/items/{item_id}",
        handler: &Handlers.handle_validation_errors_invalid_uuid_format/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "item_id" => %{"source" => "path", "type" => "string", "format" => "uuid"}
          },
          "required" => ["item_id"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: validation_errors - Invalid boolean value
  """
  def create_app_handle_validation_errors_invalid_boolean_value() do
    routes = [
      %{
        method: :get,
        path: "/items/",
        handler: &Handlers.handle_validation_errors_invalid_boolean_value/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "q" => %{"source" => "query", "type" => "string"},
            "is_active" => %{"source" => "query", "type" => "boolean"}
          },
          "required" => ["q", "is_active"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: validation_errors - Invalid datetime format
  """
  def create_app_handle_validation_errors_invalid_datetime_format() do
    routes = [
      %{
        method: :post,
        path: "/items/",
        handler: &Handlers.handle_validation_errors_invalid_datetime_format/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"type" => "string"},
            "price" => %{"type" => "number"},
            "created_at" => %{"type" => "string", "format" => "date-time"}
          },
          "additionalProperties" => false,
          "required" => ["name", "price", "created_at"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: validation_errors - Invalid enum value
  """
  def create_app_handle_validation_errors_invalid_enum_value() do
    routes = [
      %{
        method: :get,
        path: "/models/{model_name}",
        handler: &Handlers.handle_validation_errors_invalid_enum_value/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "model_name" => %{
              "source" => "path",
              "type" => "string",
              "enum" => ["alexnet", "resnet", "lenet"]
            }
          },
          "required" => ["model_name"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: validation_errors - Malformed JSON body
  """
  def create_app_handle_validation_errors_malformed_json_body() do
    routes = [
      %{
        method: :post,
        path: "/items/",
        handler: &Handlers.handle_validation_errors_malformed_json_body/1,
        request_schema: %{"type" => "string"}
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: validation_errors - Missing required body field
  """
  def create_app_handle_validation_errors_missing_required_body_field() do
    routes = [
      %{
        method: :post,
        path: "/items/",
        handler: &Handlers.handle_validation_errors_missing_required_body_field/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{"name" => %{"type" => "string"}, "price" => %{"type" => "string"}},
          "additionalProperties" => false,
          "required" => ["name", "price"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: validation_errors - Missing required query parameter
  """
  def create_app_handle_validation_errors_missing_required_query_parameter() do
    routes = [
      %{
        method: :get,
        path: "/items/",
        handler: &Handlers.handle_validation_errors_missing_required_query_parameter/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"q" => %{"source" => "query", "type" => "string"}},
          "required" => ["q"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: validation_errors - Multiple validation errors
  """
  def create_app_handle_validation_errors_multiple_validation_errors() do
    routes = [
      %{
        method: :post,
        path: "/items/",
        handler: &Handlers.handle_validation_errors_multiple_validation_errors/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"type" => "string", "minLength" => 3},
            "price" => %{"type" => "integer", "exclusiveMinimum" => 0},
            "quantity" => %{"type" => "integer"}
          },
          "additionalProperties" => false,
          "required" => ["name", "price", "quantity"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: validation_errors - Nested object validation error
  """
  def create_app_handle_validation_errors_nested_object_validation_error() do
    routes = [
      %{
        method: :post,
        path: "/items/",
        handler: &Handlers.handle_validation_errors_nested_object_validation_error/1,
        request_schema: %{
          "type" => "object",
          "properties" => %{
            "name" => %{"type" => "string"},
            "price" => %{"type" => "number"},
            "seller" => %{
              "type" => "object",
              "properties" => %{
                "name" => %{"type" => "string", "minLength" => 3},
                "address" => %{
                  "type" => "object",
                  "properties" => %{
                    "city" => %{"type" => "string", "minLength" => 3},
                    "zip_code" => %{"type" => "string", "minLength" => 5}
                  },
                  "additionalProperties" => false,
                  "required" => ["city", "zip_code"]
                }
              },
              "additionalProperties" => false,
              "required" => ["name", "address"]
            }
          },
          "additionalProperties" => false,
          "required" => ["name", "price", "seller"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: validation_errors - Numeric constraint violation - gt (greater than)
  """
  def create_app_handle_validation_errors_numeric_constraint_violation___gt__greater_than_() do
    routes = [
      %{
        method: :get,
        path: "/items/",
        handler:
          &Handlers.handle_validation_errors_numeric_constraint_violation___gt__greater_than_/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "q" => %{"source" => "query", "type" => "string"},
            "price" => %{"source" => "query", "type" => "number"}
          },
          "required" => ["q", "price"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: validation_errors - Numeric constraint violation - le (less than or equal)
  """
  def create_app_handle_validation_errors_numeric_constraint_violation___le__less_than_or_equal_() do
    routes = [
      %{
        method: :get,
        path: "/items/",
        handler:
          &Handlers.handle_validation_errors_numeric_constraint_violation___le__less_than_or_equal_/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "q" => %{"source" => "query", "type" => "string"},
            "limit" => %{"source" => "query", "type" => "integer", "maximum" => 100}
          },
          "required" => ["q", "limit"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: validation_errors - Query param type error - string provided for int
  """
  def create_app_handle_validation_errors_query_param_type_error___string_provided_for_int() do
    routes = [
      %{
        method: :get,
        path: "/items/",
        handler:
          &Handlers.handle_validation_errors_query_param_type_error___string_provided_for_int/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "q" => %{"source" => "query", "type" => "string"},
            "skip" => %{"source" => "query", "type" => "integer"}
          },
          "required" => ["q", "skip"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: validation_errors - String max_length constraint violation
  """
  def create_app_handle_validation_errors_string_max_length_constraint_violation() do
    routes = [
      %{
        method: :get,
        path: "/items/",
        handler: &Handlers.handle_validation_errors_string_max_length_constraint_violation/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"q" => %{"source" => "query", "type" => "string", "maxLength" => 50}},
          "required" => ["q"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: validation_errors - String min_length constraint violation
  """
  def create_app_handle_validation_errors_string_min_length_constraint_violation() do
    routes = [
      %{
        method: :get,
        path: "/items/",
        handler: &Handlers.handle_validation_errors_string_min_length_constraint_violation/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{"q" => %{"source" => "query", "type" => "string", "minLength" => 3}},
          "required" => ["q"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end

  @doc """
  App factory for fixture: validation_errors - String regex pattern mismatch
  """
  def create_app_handle_validation_errors_string_regex_pattern_mismatch() do
    routes = [
      %{
        method: :get,
        path: "/items/",
        handler: &Handlers.handle_validation_errors_string_regex_pattern_mismatch/1,
        parameter_schema: %{
          "type" => "object",
          "properties" => %{
            "q" => %{"source" => "query", "type" => "string", "pattern" => "^[a-zA-Z0-9_-]+$"}
          },
          "required" => ["q"]
        }
      }
    ]

    config = %{}
    {routes, config}
  end
end
