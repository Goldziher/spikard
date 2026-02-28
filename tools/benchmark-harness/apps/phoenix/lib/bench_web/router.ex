defmodule BenchWeb.Router do
  use Phoenix.Router

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/", BenchWeb do
    pipe_through :api

    get "/health", HealthController, :index

    # Raw JSON body endpoints
    post "/json/small", JsonController, :small
    post "/json/medium", JsonController, :medium
    post "/json/large", JsonController, :large
    post "/json/very-large", JsonController, :very_large

    # Raw multipart endpoints
    post "/multipart/small", MultipartController, :small
    post "/multipart/medium", MultipartController, :medium
    post "/multipart/large", MultipartController, :large

    # Raw URL-encoded endpoints
    post "/urlencoded/simple", UrlencodedController, :simple
    post "/urlencoded/complex", UrlencodedController, :complex

    # Raw path parameter endpoints
    get "/path/simple/:id", PathController, :simple
    get "/path/multiple/:user_id/:post_id", PathController, :multiple
    get "/path/deep/:org/:team/:project/:resource/:id", PathController, :deep
    get "/path/int/:id", PathController, :int_param
    get "/path/uuid/:uuid", PathController, :uuid_param
    get "/path/date/:date", PathController, :date_param

    # Raw query parameter endpoints
    get "/query/few", QueryController, :few
    get "/query/medium", QueryController, :medium
    get "/query/many", QueryController, :many
  end

  scope "/validated", BenchWeb do
    pipe_through :api

    # Validated JSON body endpoints
    post "/json/small", ValidatedJsonController, :small
    post "/json/medium", ValidatedJsonController, :medium
    post "/json/large", ValidatedJsonController, :large
    post "/json/very-large", ValidatedJsonController, :very_large

    # Validated multipart endpoints
    post "/multipart/small", ValidatedMultipartController, :small
    post "/multipart/medium", ValidatedMultipartController, :medium
    post "/multipart/large", ValidatedMultipartController, :large

    # Validated URL-encoded endpoints
    post "/urlencoded/simple", ValidatedUrlencodedController, :simple
    post "/urlencoded/complex", ValidatedUrlencodedController, :complex

    # Validated path parameter endpoints
    get "/path/simple/:id", ValidatedPathController, :simple
    get "/path/multiple/:user_id/:post_id", ValidatedPathController, :multiple
    get "/path/deep/:org/:team/:project/:resource/:id", ValidatedPathController, :deep
    get "/path/int/:id", ValidatedPathController, :int_param
    get "/path/uuid/:uuid", ValidatedPathController, :uuid_param
    get "/path/date/:date", ValidatedPathController, :date_param

    # Validated query parameter endpoints
    get "/query/few", ValidatedQueryController, :few
    get "/query/medium", ValidatedQueryController, :medium
    get "/query/many", ValidatedQueryController, :many
  end
end
