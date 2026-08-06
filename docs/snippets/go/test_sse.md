```go
package main

import (
	"encoding/json"
	"testing"
)

func notificationsHandler(req []byte) ([]byte, error) {
	events := []map[string]interface{}{
		{"event": "message", "data": map[string]int{"count": 0}},
		{"event": "message", "data": map[string]int{"count": 1}},
		{"event": "message", "data": map[string]int{"count": 2}},
	}
	return json.Marshal(events)
}

func TestNotificationsHandlerEmitsThreeEvents(t *testing.T) {
	respBytes, err := notificationsHandler(nil)
	if err != nil {
		t.Fatalf("handler returned error: %v", err)
	}

	var events []map[string]interface{}
	if err := json.Unmarshal(respBytes, &events); err != nil {
		t.Fatalf("unmarshal: %v", err)
	}

	if len(events) != 3 {
		t.Fatalf("event count: got %d want %d", len(events), 3)
	}

	first := events[0]["data"].(map[string]interface{})
	if first["count"] != float64(0) {
		t.Errorf("first event count: got %v want %v", first["count"], 0)
	}

	last := events[2]["data"].(map[string]interface{})
	if last["count"] != float64(2) {
		t.Errorf("last event count: got %v want %v", last["count"], 2)
	}
}
```
