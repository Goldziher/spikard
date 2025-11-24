```ruby
require "spikard"

app = Spikard::App.new

app.post "/upload" do |_params, _query, body|
  file = body["file"]
  { filename: file[:filename], size: file[:tempfile].size }
end
```
