```go
package main

import (
	"encoding/json"
	spikard "github.com/xberg-io/spikard/packages/go"
)

func main() {
	app, _ := spikard.NewApp()
	defer app.Close()

	app.Get(func(req []byte) ([]byte, error) {
		events := []map[string]interface{}{
			{"event": "message", "data": "Hello"},
			{"event": "message", "data": "World"},
		}
		return json.Marshal(events)
	}, "/events")

	app.Run()
}
```
