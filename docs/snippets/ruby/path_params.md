```ruby
app = Spikard::App.new

app.get("/orders/:order_id") do |params, query, _body|
  {
    id: params[:order_id].to_i,
    details: query["details"] == "true",
  }
end
```
