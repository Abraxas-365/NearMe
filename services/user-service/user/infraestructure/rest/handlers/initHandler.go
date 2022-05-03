package handlers

import (
	"user-service/user/application"

	"github.com/gofiber/fiber/v2"
)

type UserHandler interface {
	CreateUser(c *fiber.Ctx) error
}

type userHandler struct {
	userApp application.UserApplication
}

func NewUserHandler(app application.UserApplication) UserHandler {
	return &userHandler{
		app,
	}
}
