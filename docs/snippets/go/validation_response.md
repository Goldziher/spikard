```go
package main

import (
	"encoding/json"
	spikard "github.com/xberg-io/spikard/packages/go"
)

type Product struct {
	ID    int    `json:"id"`
	Name  string `json:"name"`
	Price float64 `json:"price"`
}

func main() {
	app, _ := spikard.NewApp()
	defer app.Close()

	app.Get(func(req []byte) ([]byte, error) {
		product := Product{
			ID:    1,
			Name:  "Widget",
			Price: 29.99,
		}
		return json.Marshal(product)
	}, "/products/1")

	app.Run()
}
```
