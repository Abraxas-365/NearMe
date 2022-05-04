package ports

import (
	"booking-service/nightclub/core/models"

	"github.com/google/uuid"
)

type NightclubRepository interface {
	GetNightclub(nightclubId uuid.UUID) (models.Nightclub, error)
	Create(models.Nightclub) error
	IsNightclubInDb(nightclubId uuid.UUID) bool
	Delete(nightclubId uuid.UUID) error
}
