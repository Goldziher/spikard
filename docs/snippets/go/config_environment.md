```go
package main

import (
	"encoding/json"
	"os"
	"strconv"
	spikard "github.com/xberg-io/spikard/packages/go"
)

func main() {
	app, _ := spikard.NewApp()
	defer app.Close()

	host := os.Getenv("SERVER_HOST")
	if host == "" {
		host = "127.0.0.1"
	}

	portStr := os.Getenv("SERVER_PORT")
	if portStr == "" {
		portStr = "8080"
	}
	portNum, _ := strconv.ParseUint(portStr, 10, 16)
	port := uint16(portNum)

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
