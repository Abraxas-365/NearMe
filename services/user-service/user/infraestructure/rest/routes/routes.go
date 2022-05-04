package routes

import (
	"user-service/user/infraestructure/rest/handlers"

	"github.com/gofiber/fiber/v2"
)

func UsersRoute(app *fiber.App, handler handlers.UserHandler) {
	users := app.Group("/api/users")
	/*Login user*/
	users.Post("/login", handler.Login)
	/*Register user*/
	users.Post("/register", handler.Create)
}
