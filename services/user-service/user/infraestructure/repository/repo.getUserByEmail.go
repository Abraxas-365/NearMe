package repository

import (
	"context"
	"user-service/user/core/models"

	"go.mongodb.org/mongo-driver/bson"
)

func (r *mongoRepository) GetUserByEmail(email string) (models.User, error) {
	ctx, cancel := context.WithTimeout(context.Background(), r.timeout)
	defer cancel()
	collection := r.client.Database(r.database).Collection(r.collection)
	user := models.User{}
	filter := bson.M{"email": email}
	if err := collection.FindOne(ctx, filter).Decode(&user); err != nil {
		return models.User{}, err
	}

	return user, nil
}
