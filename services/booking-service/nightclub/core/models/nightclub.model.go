package models

import "github.com/google/uuid"

type Nightclub struct {
	ID           uuid.UUID `bson:"_id,omitempty" json:"id"`
	Name         string    `bson:"name" json:"name"`
	NeedPromotor bool      `bson:"need_promotor" json:"need_promotor"`
	Promotors    Promotors `bson:"promotors,omitempty" json:"promotor"`
}
