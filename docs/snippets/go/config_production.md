```go
package main

import (
	"encoding/json"
	spikard "github.com/xberg-io/spikard/packages/go"
)

func main() {
	app, _ := spikard.NewApp()
	defer app.Close()

	host := "0.0.0.0"
	port := uint16(443)
	config := &spikard.ServerConfig{
		Host: &host,
		Port: &port,
	}
	app.Config(config)

	app.Get(func(req []byte) ([]byte, error) {
		return json.Marshal(map[string]string{"status": "ok"})
	}, "/health")

	app.Run()
}
```
