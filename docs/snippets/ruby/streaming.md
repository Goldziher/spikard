```ruby
require "json"

app.get "/stream" do |_request|
  Enumerator.new do |y|
    3.times do |i|
      y << JSON.dump({ tick: i }) + "\n"
      sleep 0.1
    end
  end
end
```
