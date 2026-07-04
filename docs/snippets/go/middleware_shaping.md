```go
package main

import (
	"encoding/json"
	"fmt"
	"sync"
	"time"
	spikard "github.com/xberg-io/spikard/packages/go"
)

type RateLimiter struct {
	limit    int
	window   time.Duration
	requests map[string][]time.Time
	mu       sync.Mutex
}

func NewRateLimiter(limit int, window time.Duration) *RateLimiter {
	return &RateLimiter{
		limit:    limit,
		window:   window,
		requests: make(map[string][]time.Time),
	}
}

func (rl *RateLimiter) Allow(clientID string) bool {
	rl.mu.Lock()
	defer rl.mu.Unlock()

	now := time.Now()
	cutoff := now.Add(-rl.window)

	var recent []time.Time
	for _, t := range rl.requests[clientID] {
		if t.After(cutoff) {
			recent = append(recent, t)
		}
	}

	if len(recent) >= rl.limit {
		return false
	}

	rl.requests[clientID] = append(recent, now)
	return true
}

func main() {
	app, _ := spikard.NewApp()
	defer app.Close()

	limiter := NewRateLimiter(10, time.Minute)

	handler := func(req []byte) ([]byte, error) {
		if !limiter.Allow("client1") {
			return nil, fmt.Errorf("rate limit exceeded")
		}
		return json.Marshal(map[string]string{"status": "ok"})
	}

	app.Get(handler, "/api/data")

	app.Run()
}
```
