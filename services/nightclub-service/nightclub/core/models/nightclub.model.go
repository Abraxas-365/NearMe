package models

import "github.com/google/uuid"

type Nightclub struct {
	ID           uuid.UUID    `bson:"_id,omitempty" json:"id"`
	Name         Name         `bson:"name" json:"name"`
	Password     Password     `bson:"password" json:"password"`
	NeedPromotor bool         `bson:"need_promotor" json:"need_promotor"`
	Promotors    []*uuid.UUID `bson:"promotors,omitempty" json:"promotor"`
	// TODO working days
}

type NightclubPublic struct {
	ID           uuid.UUID    `bson:"_id,omitempty" json:"id"`
	Name         Name         `bson:"name" json:"name"`
	NeedPromotor bool         `bson:"need_promotor" json:"need_promotor"`
	Promotors    []*uuid.UUID `bson:"promotors,omitempty" json:"promotor"`
}
