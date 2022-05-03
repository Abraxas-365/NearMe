package service

import (
	"booking-service/nightclub/core/ports"
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
	nightclubRepo ports.NightclubRepository
}

func NewNightclubService(nightclubRepo ports.NightclubRepository) BookingService {
	return &bookingService{
		nightclubRepo,
	}
}
