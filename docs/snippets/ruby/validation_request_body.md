```ruby
CreateUserSchema = Dry::Schema.Params do
  required(:email).filled(:string, format?: /@/)
  required(:age).filled(:integer, gteq?: 18)
  required(:username).filled(:string, format?: /^[a-zA-Z0-9_]+$/)
end

app.post("/users") do |_params, _query, body|
  result = CreateUserSchema.call(body)

  if result.failure?
    halt 400, result.errors.to_h
  end

  {
    id: "usr_123",
    email: result[:email],
    age: result[:age],
    username: result[:username]
  }
end
```
