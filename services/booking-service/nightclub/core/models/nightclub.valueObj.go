package models

import "github.com/google/uuid"

type Promotors []*uuid.UUID

func (p Promotors) IsPromotor(userId uuid.UUID) bool {
	for _, elem := range p {
		if *elem == userId {
			return true
		}
	}
	return false

}
