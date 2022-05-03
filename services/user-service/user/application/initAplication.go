package application

import (
	"user-service/user/core/models"
	"user-service/user/core/ports"
	service "user-service/user/core/services"
)

type UserApplication interface {
	Create(models.User) (models.UserPublic, error)
	Login(email models.Email, password models.Password) (models.UserPublic, error)
}

type userApplication struct {
	userRepo    ports.UserRepository
	mqPublisher ports.UserMQPublisher
	userService service.UserService
}

func newUserApplication(userRepo ports.UserRepository, userService service.UserService, mqPublisher ports.UserMQPublisher) UserApplication {
	userApplication := &userApplication{
		userRepo,
		mqPublisher,
		userService,
	}
	return userApplication
}
