```ruby
require "spikard"

app.post "/upload" do |request|
  file = request[:body]["file"]
  { filename: file[:filename], size: file[:tempfile].size }
end
```
