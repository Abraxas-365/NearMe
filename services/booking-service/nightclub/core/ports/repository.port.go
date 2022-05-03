package ports

import (
	"booking-service/nightclub/core/models"

	"github.com/google/uuid"
)

type NightclubRepository interface {
	GetNightclub(nightclubId uuid.UUID) (models.Nightclub, error)
}
