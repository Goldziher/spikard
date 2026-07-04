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
		data := []map[string]interface{}{
			{"chunk": 1, "content": "part1"},
			{"chunk": 2, "content": "part2"},
			{"chunk": 3, "content": "part3"},
		}
		return json.Marshal(data)
	}, "/stream")

	app.Run()
}
```
