package application

import (
	"user-service/user/core/models"
)

func (a *userApplication) Login(email models.Email, password models.Password) (models.UserPublic, error) {
	user, err := a.userService.Auth(email, password)
	if err != nil {
		return models.UserPublic{}, err
	}
	return user, nil

}
