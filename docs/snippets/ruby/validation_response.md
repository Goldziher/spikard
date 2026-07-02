```ruby
UserSchema = Dry::Schema.JSON do
  required(:id).filled(:string)
  required(:email).filled(:string, format?: /@/)
  required(:age).filled(:integer)
end

UserListResponseSchema = Dry::Schema.JSON do
  required(:users).array(:hash) do
    required(:id).filled(:string)
    required(:email).filled(:string, format?: /@/)
    required(:age).filled(:integer)
  end
  required(:total).filled(:integer)
  required(:page).filled(:integer)
end

app.get("/users") do |_params, _query, _body|
  users = [
    { id: "usr_1", email: "alice@example.com", age: 30 },
    { id: "usr_2", email: "bob@example.com", age: 25 }
  ]

  response = {
    users: users,
    total: users.length,
    page: 1
  }

  # Validate response before returning
  result = UserListResponseSchema.call(response)

  if result.failure?
    halt 500, { error: "Response validation failed", details: result.errors.to_h }
  end

  result.to_h
end

# Example: catch validation errors
app.get("/invalid") do |_params, _query, _body|
  response = { id: "usr_1", email: "test@example.com" }
  # Missing 'age' field

  result = UserSchema.call(response)

  if result.failure?
    halt 500, { error: "Response validation failed", details: result.errors.to_h }
  end

  result.to_h
end
```
