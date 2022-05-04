package events

import "github.com/google/uuid"

type BookingBooked struct {
	ID       uuid.UUID `bson:"_id,omitempty" json:"id"`
	UserName string    `bson:"name" json:"name,omitempty"`
}

func (e BookingBooked) Name() string {
	return "event.booking.booked"
}

func (e BookingBooked) Exchange() string {
	return "Booking"
}
func (e BookingBooked) Routing() string {
	return "booked"
}
