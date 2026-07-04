```go
package main

import (
	"encoding/json"
	"fmt"
	spikard "github.com/xberg-io/spikard/packages/go"
)

type CreateItemRequest struct {
	Name string `json:"name"`
}

type ErrorResponse struct {
	Error  string `json:"error"`
	Detail string `json:"detail"`
}

func main() {
	app, _ := spikard.NewApp()
	defer app.Close()

	app.Post(func(req []byte) ([]byte, error) {
		var request CreateItemRequest
		if err := json.Unmarshal(req, &request); err != nil {
			errResp := ErrorResponse{
				Error:  "validation_error",
				Detail: fmt.Sprintf("Invalid request: %v", err),
			}
			respJSON, _ := json.Marshal(errResp)
			return respJSON, fmt.Errorf("validation failed")
		}

		if request.Name == "" {
			errResp := ErrorResponse{
				Error:  "validation_error",
				Detail: "name field is required",
			}
			respJSON, _ := json.Marshal(errResp)
			return respJSON, fmt.Errorf("validation failed")
		}

		return json.Marshal(request)
	}, "/items")

	app.Run()
}
```
