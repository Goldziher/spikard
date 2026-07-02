```ruby
PathParamsSchema = Dry::Schema.Params do
  required(:user_id).filled(:string, format?: /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/)
  required(:post_id).filled(:integer, gt?: 0)
end

app.get("/users/:user_id/posts/:post_id") do |params, _query, _body|
  result = PathParamsSchema.call(params)

  halt 400, result.errors.to_h if result.failure?

  {
    user_id: result[:user_id],
    post_id: result[:post_id],
    title: "Sample Post"
  }
end
```
