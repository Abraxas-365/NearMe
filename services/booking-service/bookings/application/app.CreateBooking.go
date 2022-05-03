package application

import (
	"booking-service/bookings/core/models"
)

func (a *bookingApplication) Create(new models.Booking) error {
	//Check if user can book
	if err := a.bookingService.CanBooked(new.UserBooking, new.Nightclub); err != nil {
		return err
	}
	// safe in db
	event, err := a.bookingRepo.Create(new)
	if err != nil {
		return err
	}
	//publish event
	a.mqPublisher.PublishEvent(event)
	return nil
}
