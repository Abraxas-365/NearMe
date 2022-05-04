package application

import (
	"booking-service/nightclub/core/models"
	"booking-service/nightclub/core/ports"

	"github.com/google/uuid"
)

type NightclubApplication interface {
	Create(models.Nightclub) error
	IsNightclubInDb(nightclubId uuid.UUID) bool
	Delete(nightclubId uuid.UUID) error
	GetNightclub(nightclubId uuid.UUID) (models.Nightclub, error)
}

type nightclubApp struct {
	nightclubRepo ports.NightclubRepository
}
