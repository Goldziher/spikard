```go
package main

import (
	"encoding/json"
	"fmt"
	spikard "github.com/xberg-io/spikard/packages/go"
)

type PathRequest struct {
	Params map[string]interface{} `json:"params"`
}

func main() {
	app, _ := spikard.NewApp()
	defer app.Close()

	app.Get(func(req []byte) ([]byte, error) {
		var pathReq PathRequest
		if err := json.Unmarshal(req, &pathReq); err != nil {
			return nil, fmt.Errorf("invalid request: %w", err)
		}

		id, ok := pathReq.Params["id"].(float64)
		if !ok {
			return nil, fmt.Errorf("id parameter must be a number")
		}
		if id <= 0 {
			return nil, fmt.Errorf("id must be greater than 0")
		}

		return json.Marshal(map[string]interface{}{"id": int(id)})
	}, "/items/{id}")

	app.Run()
}
```
