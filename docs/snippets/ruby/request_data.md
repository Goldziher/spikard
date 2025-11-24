```ruby
App.post("/orders/:order_id") do |ctx|
  order = ctx.json
  {
    **order,
    id: ctx.params[:order_id].to_i,
    request_id: ctx.headers["x-request-id"],
    verbose: ctx.query["verbose"] == "true",
  }
end
```
