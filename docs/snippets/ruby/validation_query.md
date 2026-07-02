```ruby
ListUsersQuery = Dry::Schema.Params do
  optional(:page).filled(:integer, gteq?: 1)
  optional(:limit).filled(:integer, gteq?: 1, lteq?: 100)
  optional(:sort_by).filled(:string, included_in?: %w[name email created_at])
  optional(:min_age).filled(:integer, gteq?: 0, lteq?: 120)
end

app.get("/users") do |_params, query, _body|
  result = ListUsersQuery.call(query)

  halt 400, result.errors.to_h if result.failure?

  {
    page: result[:page] || 1,
    limit: result[:limit] || 10,
    users: []
  }
end
```
