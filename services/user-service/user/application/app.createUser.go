package application

import "user-service/user/core/models"

func (a *userApplication) Create(newUser models.User) (models.UserPublic, error) {
	/*Check if user can be created*/
	if err := a.userService.CanCreateUser(newUser); err != nil {
		return models.UserPublic{}, err
	}

	/*Create user*/
	event, err := a.userRepo.CreateUser(*newUser.New())
	if err != nil {
		return models.UserPublic{}, err
	}
	/*send event to queue*/
	a.mqPublisher.PublishEvent(event)

	/*Login user*/
	logedUser, err := a.Login(newUser.Email, newUser.Password)
	if err != nil {
		return models.UserPublic{}, err
	}

	return logedUser, nil

}
