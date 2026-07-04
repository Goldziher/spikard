```go
package main

import (
	"encoding/json"
	spikard "github.com/xberg-io/spikard/packages/go"
)

type Order struct {
	ID       int    `json:"id"`
	Item     string `json:"item"`
	Quantity int    `json:"quantity"`
}

func main() {
	app, _ := spikard.NewApp()
	defer app.Close()

	app.Post(func(req []byte) ([]byte, error) {
		var order Order
		if err := json.Unmarshal(req, &order); err != nil {
			return nil, err
		}
		return json.Marshal(order)
	}, "/orders")

	app.Run()
}
```
