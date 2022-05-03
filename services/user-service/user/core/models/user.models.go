package models

import (
	"time"

	"github.com/google/uuid"
)

type User struct {
	ID       uuid.UUID `bson:"_id,omitempty" json:"id"`
	Name     Name      `bson:"name" json:"name,omitempty"`
	Password Password  `bson:"password" json:"password,omitempty"`
	Email    Email     `bson:"email" json:"email,omitempty"`
	Created  time.Time `bson:"created" json:"created"`
	Edit     time.Time `bson:"edited" json:"edited"`
}

//All users start as normal users
func (u *User) New() {
	u.ID = uuid.New()
	u.Password = u.Password.Encrypt()
	u.Created = time.Now()
	u.Edit = time.Now()
}

func (u *User) Validate() error {
	if err := u.Email.Validate(); err != nil {
		return err
	}
	if err := u.Password.Validate(); err != nil {
		return err
	}
	if err := u.Name.Validate(); err != nil {
		return err
	}
	return nil
}

func (u *User) ToPublic() UserPublic {
	userPublic := UserPublic{
		ID:   u.ID,
		Name: u.Name,
	}
	return userPublic
}

/*User for public exposure*/

type UserPublic struct {
	ID    uuid.UUID `bson:"_id,omitempty" json:"id"`
	Name  Name      `bson:"name" json:"name,omitempty"`
	Role  Role      `bson:"role" json:"role"`
	Token string    `bson:"token" json:"token"`
}
