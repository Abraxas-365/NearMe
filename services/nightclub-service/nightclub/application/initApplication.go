package application

import (
	"nightclub-service/nightclub/core/events"
	"nightclub-service/nightclub/core/models"

	"github.com/google/uuid"
)

type NightclubApplication interface {
	Create(new models.Nightclub) (events.Event, error)
	AddPromotor(promotorId uuid.UUID) error
	DeletePromotor(promotorId uuid.UUID) error
	DeleteNightclub(promotorId uuid.UUID) error
}
