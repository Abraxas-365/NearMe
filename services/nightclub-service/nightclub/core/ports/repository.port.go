package repository

import (
	"nightclub-service/nightclub/core/events"
	"nightclub-service/nightclub/core/models"

	"github.com/google/uuid"
)

type NightclubRepository interface {
	Create(new models.Nightclub) (events.Event, error)
	AddPromotor(promotorId uuid.UUID) error
}
