package repository

import (
	"user-service/user/core/events"
	"user-service/user/core/models"
)

func (r *mongoRepository) CreateUser(user models.User) (events.Event, error) {
	return nil, nil
}
