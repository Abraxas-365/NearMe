package handlers

import (
	"user-service/user/core/models"

	"github.com/gofiber/fiber/v2"
)

func (h *userHandler) Login(c fiber.Ctx) error {
	body := struct {
		Email    models.Email    `json:"email"`
		Password models.Password `json:"password"`
	}{}
	if err := c.BodyParser(&body); err != nil {
		return fiber.ErrBadRequest
	}
	logedUser, err := h.userApp.Login(body.Email, body.Password)
	if err != nil {
		return c.Status(500).SendString(err.Error())
	}
	return c.Status(fiber.StatusOK).JSON(logedUser)

}
