package main

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func main() {
	server := gin.Default()

	server.GET("/events", getEvents) // GET, POST, PUT, PATCH, DELETE

	// Server run at port 8080
	server.Run(":8080")
}

func getEvents(context *gin.Context) {
	context.JSON(http.StatusOK, gin.H{"message": "Hello World!"})
}
