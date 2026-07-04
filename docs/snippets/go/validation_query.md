```go
package main

import (
	"encoding/json"
	"fmt"
	"strconv"
	spikard "github.com/xberg-io/spikard/packages/go"
)

type QueryRequest struct {
	Query map[string]interface{} `json:"query"`
}

func main() {
	app, _ := spikard.NewApp()
	defer app.Close()

	app.Get(func(req []byte) ([]byte, error) {
		var queryReq QueryRequest
		if err := json.Unmarshal(req, &queryReq); err != nil {
			return nil, fmt.Errorf("invalid request: %w", err)
		}

		pageRaw, ok := queryReq.Query["page"]
		if !ok {
			pageRaw = float64(1)
		}
		page := int(pageRaw.(float64))

		if page < 1 {
			return nil, fmt.Errorf("page must be at least 1")
		}

		return json.Marshal(map[string]interface{}{"page": page})
	}, "/items")

	app.Run()
}
```
