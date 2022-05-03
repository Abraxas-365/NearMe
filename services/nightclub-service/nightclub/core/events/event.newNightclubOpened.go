package events

import "github.com/google/uuid"

type NewNightclubOpened struct {
	ID       uuid.UUID `bson:"_id,omitempty" json:"id"`
	UserName string    `bson:"name" json:"name,omitempty"`
	Role     string    `bson:"role" json:"role"`
}

func (e NewNightclubOpened) Name() string {
	return "event.nightclub.new_opened"
}

func (e NewNightclubOpened) Exchange() string {
	return "Nightclub"
}
func (e NewNightclubOpened) Routing() string {
	return "new_opened"
}
