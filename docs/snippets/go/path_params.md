```go
package main

import (
	"encoding/json"
	spikard "github.com/xberg-io/spikard/packages/go"
)

type PathRequest struct {
	Params map[string]interface{} `json:"params"`
}

type UserResponse struct {
	ID   int    `json:"id"`
	Name string `json:"name"`
}

func main() {
	app, _ := spikard.NewApp()
	defer app.Close()

	app.Get(func(req []byte) ([]byte, error) {
		var pathReq PathRequest
		if err := json.Unmarshal(req, &pathReq); err != nil {
			return nil, err
		}

		id := int(pathReq.Params["id"].(float64))
		response := UserResponse{ID: id, Name: "Alice"}
		return json.Marshal(response)
	}, "/users/{id}")

	app.Run()
}
```
