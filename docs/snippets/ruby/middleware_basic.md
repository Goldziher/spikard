```ruby
app = Spikard::App.new

app.on_request do |request|
  puts "#{request[:method]} #{request[:path]}"
  request
end
```
