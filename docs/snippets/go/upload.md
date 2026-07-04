```go
package main

import (
	"encoding/json"
	spikard "github.com/xberg-io/spikard/packages/go"
)

type FileRequest struct {
	Filename string `json:"filename"`
	Size     int    `json:"size"`
}

func main() {
	app, _ := spikard.NewApp()
	defer app.Close()

	app.Post(func(req []byte) ([]byte, error) {
		var file FileRequest
		if err := json.Unmarshal(req, &file); err != nil {
			return nil, err
		}

		return json.Marshal(map[string]interface{}{
			"filename": file.Filename,
			"size":     file.Size,
			"status":   "uploaded",
		})
	}, "/upload")

	app.Run()
}
```
