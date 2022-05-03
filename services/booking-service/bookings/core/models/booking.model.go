package models

import (
	"time"

	"github.com/google/uuid"
)

type Booking struct {
	ID            uuid.UUID    `bson:"_id,omitempty" json:"id"`
	Nightclub     uuid.UUID    `bson:"nightclub" json:"nightclub"`
	UserBooking   uuid.UUID    `bson:"user_booking" json:"user_booking"`
	GuessesNumber int          `bson:"guesses_number" json:"guesses_number"`
	QrCode        string       `bson:"qr_code" json:"qr_code"`
	Guesses       []*uuid.UUID `bson:"guesses" json:"guesses"`
	Created       time.Time    `bson:"created" json:"created"`
	BookingDate   time.Time    `bson:"booking_date" json:"booking"`
}

func (b *Booking) SetUserBooking(userID uuid.UUID) {
	b.UserBooking = userID
}
