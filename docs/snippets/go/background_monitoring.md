```go
package main

import (
	"encoding/json"
	"log"
	"sync"
	"time"
	spikard "github.com/xberg-io/spikard/packages/go"
)

type TaskMonitor struct {
	tasks map[int]string
	mu    sync.Mutex
}

func (tm *TaskMonitor) Start(taskID int) {
	tm.mu.Lock()
	tm.tasks[taskID] = "running"
	tm.mu.Unlock()
	log.Printf("Task %d started", taskID)
}

func (tm *TaskMonitor) Complete(taskID int) {
	tm.mu.Lock()
	tm.tasks[taskID] = "completed"
	tm.mu.Unlock()
	log.Printf("Task %d completed", taskID)
}

func doTask(taskID int, monitor *TaskMonitor) {
	monitor.Start(taskID)
	time.Sleep(100 * time.Millisecond)
	monitor.Complete(taskID)
}

func main() {
	app, _ := spikard.NewApp()
	defer app.Close()

	monitor := &TaskMonitor{tasks: make(map[int]string)}

	app.Post(func(req []byte) ([]byte, error) {
		var request map[string]interface{}
		if err := json.Unmarshal(req, &request); err != nil {
			return nil, err
		}

		taskID := int(request["task_id"].(float64))

		go doTask(taskID, monitor)

		return json.Marshal(map[string]interface{}{
			"status":  "queued",
			"task_id": taskID,
		})
	}, "/jobs")

	app.Run()
}
```
