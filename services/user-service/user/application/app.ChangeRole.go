package application

import (
	"user-service/user/core/models"

	"github.com/google/uuid"
)

func (a *userApplication) ChangeRole(userId uuid.UUID, role models.Role) error {
	if err := a.userRepo.ChangeRole(userId, string(role)); err != nil {
		return err
	}
	return nil
}
