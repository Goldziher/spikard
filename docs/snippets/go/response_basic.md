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
		response := map[string]interface{}{
			"status": "created",
			"id":     123,
		}
		return json.Marshal(response)
	}, "/items")

	app.Run()
}
```
