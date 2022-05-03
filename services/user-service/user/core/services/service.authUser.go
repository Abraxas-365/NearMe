package service

import (
	"user-service/internal/auth"
	"user-service/user/core/models"
)

func (r *userService) Auth(email models.Email, password models.Password) (models.UserPublic, error) {

	user, err := r.userRepo.GetUserByEmail(email)
	if err != nil {
		return models.UserPublic{}, ErrUserNotFound
	}
	if !password.EqualTo(user.Password) {
		return models.UserPublic{}, ErrUnauthorized
	}
	publicUser := user.ToPublic()

	publicUser.Token, err = auth.GereteToken(string(user.Email), user.ID)
	if err != nil {
		return models.UserPublic{}, err
	}

	return publicUser, nil

}
