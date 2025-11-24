```ruby
require "json"

app.get "/events" do |_request|
  Enumerator.new do |y|
    3.times do |i|
      y << "data: #{JSON.dump({ tick: i })}\n\n"
      sleep 0.1
    end
  end
end
```
