package logging

import (
	"time"
)

// LogMessage is the simplified event payload for a log message
type LogMessage struct {
	ID         string
	Time       time.Time
	Level      string
	Message    string `json:"msg"`
	Attributes []Attr
}

type Attr struct {
	Key   string
	Value string
}
