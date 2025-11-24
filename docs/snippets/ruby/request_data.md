```ruby
app = Spikard::App.new

app.post("/orders/:order_id") do |params, query, body|
  {
    **body,
    id: params[:order_id].to_i,
    verbose: query["verbose"] == "true",
  }
end
```
