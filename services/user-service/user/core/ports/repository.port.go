package ports

import (
	"user-service/user/core/events"
	"user-service/user/core/models"

	"github.com/google/uuid"
)

type UserRepository interface {
	CreateUser(models.User) (events.Event, error)
	IsUserExist(email string) bool
	GetUserByEmail(email string) (models.User, error)
	ChangeRole(userId uuid.UUID, role string) error
}
