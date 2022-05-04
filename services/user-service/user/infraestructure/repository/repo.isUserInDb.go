package repository

import (
	"context"
	"user-service/user/core/models"

	"go.mongodb.org/mongo-driver/bson"
)

//TODO create function
func (r *mongoRepository) IsUserInDb(email string) bool {
	ctx, cancel := context.WithTimeout(context.Background(), r.timeout)
	defer cancel()
	collection := r.client.Database(r.database).Collection(r.collection)
	check := new(models.User)
	filter := bson.M{"email": email}
	if err := collection.FindOne(ctx, filter).Decode(&check); err != nil {
		return false
	}
	return true
}
