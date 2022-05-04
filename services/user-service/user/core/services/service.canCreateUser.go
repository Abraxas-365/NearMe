package service

import "user-service/user/core/models"

func (s *userService) CanCreateUser(new models.User) error {
	// TODO add the user validations
	if err := new.Validate(); err != nil {
	}
	if s.userRepo.IsUserInDb(string(new.Email)) {
		return ErrorUserExists
	}
	return nil
}
