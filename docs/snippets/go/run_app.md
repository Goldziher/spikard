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
		return json.Marshal(map[string]string{"message": "Hello, World!"})
	}, "/")

	app.Run()
}
```
