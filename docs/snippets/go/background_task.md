```go
package main

import (
	"encoding/json"
	"fmt"
	"log"
	"time"
	spikard "github.com/xberg-io/spikard/packages/go"
)

func processFile(fileID int) {
	time.Sleep(100 * time.Millisecond)
	log.Printf("File %d processed", fileID)
}

func main() {
	app, _ := spikard.NewApp()
	defer app.Close()

	app.Post(func(req []byte) ([]byte, error) {
		var request map[string]interface{}
		if err := json.Unmarshal(req, &request); err != nil {
			return nil, err
		}

		fileID := int(request["file_id"].(float64))

		go processFile(fileID)

		return json.Marshal(map[string]interface{}{
			"status":  "processing",
			"file_id": fileID,
		})
	}, "/upload")

	app.Run()
}
```
