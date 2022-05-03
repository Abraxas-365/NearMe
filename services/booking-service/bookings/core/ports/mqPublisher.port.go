package ports

import (
	"booking-service/bookings/core/events"
)

type UserMQPublisher interface {
	PublishEvent(events.Event) error
	PublishEvents(events.Events) error
}
