package handlers

import (
	"user-service/user/core/models"

	"github.com/gofiber/fiber/v2"
)

func (h *userHandler) CreateUser(c *fiber.Ctx) error {
	newUser := models.User{}

	if err := c.BodyParser(&newUser); err != nil {
		return fiber.ErrBadRequest
	}
}
