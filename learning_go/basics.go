// build with go build basics.go
// run with go run basics.go
// comment
// package main so its can be ecxecuted alone.
package main

import "fmt"

// declaring variables
var a int
var b bool
var c string

// if you want to declalre more vars you can do this
var (
	d int
	e bool
	f string
)

func main()  {
	fmt.Printf("Hello World")
	fmt.Printf("testing if commiting works")
	//it is not necessary to declare a type
	//you can do this also but only in a function
	a1 := 10
	b2 := true
	c3 := "hello"
}
