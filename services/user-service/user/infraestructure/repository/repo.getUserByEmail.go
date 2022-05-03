package repository

import "user-service/user/core/models"

func (r *mongoRepository) GetUserByEmail(email string) (models.User, error) {
	return models.User{}, nil
}
