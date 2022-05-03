package models

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
