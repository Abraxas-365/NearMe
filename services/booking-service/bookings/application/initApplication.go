package application

import (
	"booking-service/bookings/core/models"
	"booking-service/bookings/core/ports"
	"booking-service/bookings/core/service"

	"github.com/google/uuid"
)

type BookingApplication interface {
	Create(new models.Booking) error
	Delete(userBookingId uuid.UUID, bookingId uuid.UUID) error
	PutQr(bookingId uuid.UUID, qrPath string) error
}

type bookingApplication struct {
	bookingRepo    ports.BookingRepository
	bookingService service.BookingService
	mqPublisher    ports.UserMQPublisher
}

func NewBookingApplication(bookingRepo ports.BookingRepository, bookingService service.BookingService, mqPublisher ports.UserMQPublisher) BookingApplication {
	return &bookingApplication{
		bookingRepo,
		bookingService,
		mqPublisher,
	}
}
