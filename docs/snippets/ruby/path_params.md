```ruby
App.get("/orders/:order_id") do |ctx|
  {
    id: ctx.params[:order_id].to_i,
    details: ctx.query["details"] == "true",
  }
end
```
