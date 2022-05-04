package mqpublisher

import (
	"user-service/internal/rabbit"
	"user-service/user/core/ports"
)

type mqPublisher struct {
	publisher rabbit.MQueue
}

func NewMQPublisher(mq rabbit.MQueue) ports.UserMQPublisher {
	return &mqPublisher{
		mq,
	}
}
