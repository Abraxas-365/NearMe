package handlers

import (
	"user-service/user/core/models"

	"github.com/gofiber/fiber/v2"
)

func (h *userHandler) Create(c *fiber.Ctx) error {
	newUser := models.User{}

	if err := c.BodyParser(&newUser); err != nil {
		return fiber.ErrBadRequest
	}
	user, err := h.userApp.Create(newUser)
	if err != nil {
		return c.Status(500).SendString(err.Error())
	}
	return c.Status(fiber.StatusOK).JSON(user)

}
