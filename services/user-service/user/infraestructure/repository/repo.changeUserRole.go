package repository

import (
	"github.com/google/uuid"
)

func (r *mongoRepository) ChangeRole(userId uuid.UUID, role string) error {
	return nil
}
