```ruby
app = Spikard::App.new

app.get("/health") { |_params, _query, _body| { status: "ok" } }
```
