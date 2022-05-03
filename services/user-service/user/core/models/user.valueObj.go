package models

import "errors"

//Value Name
type Name string

func (n Name) Validate() error {
	// TODO rules to the validation
	return nil
}

//Value Password
type Password string

func (p Password) Validate() error {
	// TODO rules to the validation
	return nil
}

func (p Password) Encrypt() Password {
	// TODO Encrypt function
	return p
}

func (p Password) EqualTo(other Password) bool {
	return p == other
}

//Value Email

type Email string

func (e Email) Validate() error {
	// TODO rules to the validation
	return nil
}

//Value Role

/*ROLES:
  - U = normal user
  - P = promotor
  - S = staf
*/
type Role string

func (r Role) Validate() error {
	// TODO rules to the validation
	return nil
}

func (r Role) Asign(role string) (Role, error) {
	switch role {
	case "U":
		return Role(role), nil
	case "P":
		return Role(role), nil
	case "S":
		return Role(role), nil

	}
	return Role(""), errors.New("Invalid role")
}
