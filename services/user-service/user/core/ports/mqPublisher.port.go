package ports

import (
	"user-service/user/core/events"
)

type UserMQPublisher interface {
	PublishEvent(events.Event) error
	PublishEvents(events.Events) error
}
