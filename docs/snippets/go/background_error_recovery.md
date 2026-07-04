```go
package main

import (
	"encoding/json"
	"fmt"
	"log"
	"time"
	spikard "github.com/xberg-io/spikard/packages/go"
)

func processWithRetry(taskID int, maxRetries int) {
	var err error
	for attempt := 1; attempt <= maxRetries; attempt++ {
		err = doWork(taskID)
		if err == nil {
			log.Printf("Task %d completed", taskID)
			return
		}

		if attempt < maxRetries {
			backoff := time.Duration(attempt*attempt) * time.Second
			log.Printf("Task %d failed, retrying in %v: %v", taskID, backoff, err)
			time.Sleep(backoff)
		}
	}

	log.Printf("Task %d failed after %d attempts: %v", taskID, maxRetries, err)
}

func doWork(taskID int) error {
	return nil
}

func main() {
	app, _ := spikard.NewApp()
	defer app.Close()

	app.Post(func(req []byte) ([]byte, error) {
		var request map[string]interface{}
		if err := json.Unmarshal(req, &request); err != nil {
			return nil, err
		}

		taskID := int(request["task_id"].(float64))

		go processWithRetry(taskID, 3)

		return json.Marshal(map[string]interface{}{
			"status":  "queued",
			"task_id": taskID,
		})
	}, "/tasks")

	app.Run()
}
```
