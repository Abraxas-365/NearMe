package repository

import (
	"context"
	"user-service/user/core/events"
	"user-service/user/core/models"
)

func (r *mongoRepository) CreateUser(user models.User) (events.Event, error) {
	ctx, cancel := context.WithTimeout(context.Background(), r.timeout)
	defer cancel()
	collection := r.client.Database(r.database).Collection(r.collection)
	_, err := collection.InsertOne(ctx, user)
	if err != nil {
		return nil, err
	}

	return events.UserCreated{
		ID:       user.ID,
		UserName: string(user.Name),
	}, nil
}
