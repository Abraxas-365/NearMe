package events

import "github.com/google/uuid"

type BookingDeleted struct {
	ID uuid.UUID `bson:"_id,omitempty" json:"id"`
}

func (e BookingDeleted) Name() string {
	return "event.booking.deleted"
}

func (e BookingDeleted) Exchange() string {
	return "Booking"
}
func (e BookingDeleted) Routing() string {
	return "deleted"
}
