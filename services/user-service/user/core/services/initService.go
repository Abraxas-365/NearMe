package service

import (
	"errors"
	"user-service/user/core/models"
	"user-service/user/core/ports"
)

var (
	ErrorUserExists = errors.New("User already exists")
	ErrUserNotFound = errors.New("User not found")
	ErrUnauthorized = errors.New("Unauthorized")
)

type UserService interface {
	Auth(email models.Email, password models.Password) (models.UserPublic, error)
	CanCreateUser(new models.User) error
}

type userService struct {
	userRepo ports.UserRepository
}

func NewUserService(userRepo ports.UserRepository) UserService {
	return &userService{
		userRepo,
	}
}
