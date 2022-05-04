package ports

import (
	"booking-service/user/core/models"

	"github.com/google/uuid"
)

type UserRepository interface {
	Create(models.User) error
	IsUserInDb(userId uuid.UUID) bool
	Delete(userId uuid.UUID) error
}
