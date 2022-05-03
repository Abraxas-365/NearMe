package application

import "user-service/user/core/models"

func (a *userApplication) Create(new models.User) (models.UserPublic, error) {
	/*Check if user can be created*/
	if err := a.userService.CanCreateUser(new); err != nil {
		return models.UserPublic{}, err
	}

	/*Create user*/
	event, err := a.userRepo.CreateUser(new)
	if err != nil {
		return models.UserPublic{}, err
	}
	/*send event to queue*/
	a.mqPublisher.PublishEvent(event)

	/*Login user*/
	logedUser, err := a.Login(new.Email, new.Password)
	if err != nil {
		return models.UserPublic{}, err
	}

	return logedUser, nil

}
