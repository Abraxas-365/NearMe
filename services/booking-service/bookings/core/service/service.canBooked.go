package service

import "github.com/google/uuid"

func (s *bookingService) CanBooked(userId uuid.UUID, nightclubId uuid.UUID) error {
	nightclub, err := s.nightclubApp.GetNightclub(nightclubId)
	if err != nil {
		return err
	}
	//check if the nightclub need a promotor
	if !nightclub.NeedPromotor {
		//if no need promotor
		return nil
	}
	//if needs a promotor check if user is promotor
	if !nightclub.Promotors.IsPromotor(userId) {
		// if not promotor
		return ErrorUnauthorized
	}
	// if promotor
	return nil
}
