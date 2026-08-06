```go
package main

import (
	"encoding/json"
	"sync"
	"testing"
	"time"
)

func processFile(fileID int, done *sync.WaitGroup, results *sync.Map) {
	defer done.Done()
	time.Sleep(10 * time.Millisecond)
	results.Store(fileID, "processed")
}

func uploadHandler(results *sync.Map, wg *sync.WaitGroup) func(req []byte) ([]byte, error) {
	return func(req []byte) ([]byte, error) {
		var request map[string]interface{}
		if err := json.Unmarshal(req, &request); err != nil {
			return nil, err
		}

		fileID := int(request["file_id"].(float64))

		wg.Add(1)
		go processFile(fileID, wg, results)

		return json.Marshal(map[string]interface{}{
			"status":  "processing",
			"file_id": fileID,
		})
	}
}

func TestUploadHandlerSchedulesBackgroundWork(t *testing.T) {
	var wg sync.WaitGroup
	var results sync.Map

	handler := uploadHandler(&results, &wg)

	req, _ := json.Marshal(map[string]int{"file_id": 123})
	respBytes, err := handler(req)
	if err != nil {
		t.Fatalf("handler returned error: %v", err)
	}

	var resp map[string]interface{}
	if err := json.Unmarshal(respBytes, &resp); err != nil {
		t.Fatalf("unmarshal: %v", err)
	}
	if resp["status"] != "processing" {
		t.Fatalf("status: got %v want %q", resp["status"], "processing")
	}

	wg.Wait()

	value, ok := results.Load(123)
	if !ok {
		t.Fatal("expected file 123 to have been processed")
	}
	if value != "processed" {
		t.Errorf("result: got %v want %q", value, "processed")
	}
}
```
