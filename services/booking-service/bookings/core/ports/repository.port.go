package ports

import (
	"booking-service/bookings/core/events"
	"booking-service/bookings/core/models"

	"github.com/google/uuid"
)

type BookingRepository interface {
	Create(new models.Booking) (events.Event, error)
	Delete(userBookingId uuid.UUID, bookingId uuid.UUID) error
	PutQr(bookingId uuid.UUID, qrPath string) error
}
