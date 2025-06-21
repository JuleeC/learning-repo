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
//or
var x,y,z int

//for decalring floats you need to use either float32 or float64
var f_value float32 = 3.14231

//string are immutable and cant be changed
var s string = "hello"
// s[0] = 'a' is not working

//Runes is int32 and it used for iterating over characters in a string
//go has support for complex numbers
func main()  {
	fmt.Printf("Hello World")
	fmt.Printf("testing if commiting works")
	//it is not necessary to declare a type
	//you can do this also but only in a function
	a1 := 10
	b2 := true
	c3 := "hello"
}
