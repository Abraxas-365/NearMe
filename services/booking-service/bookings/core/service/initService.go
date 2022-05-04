package service

import (
	"booking-service/nightclub/application"
	"errors"

	"github.com/google/uuid"
)

var (
	ErrorUnauthorized = errors.New("Unauthorized")
)

type BookingService interface {
	CanBooked(userId uuid.UUID, nightclubId uuid.UUID) error
}

type bookingService struct {
	nightclubApp application.NightclubApplication
}

func NewNightclubService(nightclubApp application.NightclubApplication) BookingService {
	return &bookingService{
		nightclubApp,
	}
}
