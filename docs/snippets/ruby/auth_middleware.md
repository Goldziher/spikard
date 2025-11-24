```ruby
app.use do |ctx, next_middleware|
  token = ctx.headers["authorization"]
  if token != "Bearer dev-token"
    { error: "unauthorized" }
  else
    next_middleware.call
  end
end
```
