```ruby
require "spikard"
require "json"

app = Spikard::App.new

app.get "/events" do |_params, _query, _body|
  Enumerator.new do |y|
    3.times do |i|
      y << "data: #{JSON.dump({ tick: i })}\n\n"
      sleep 0.1
    end
  end
end
```
