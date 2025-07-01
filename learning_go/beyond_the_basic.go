package main

import (
	"fmt"
)

// go pointers isnt arithmetic like in c, its more of an reference
// when passing a pointer to a function, the function can modify the value
// assigning a pointer to a variable without a value ist called nil pointer
// go has a garbage collector that frees up memory that is no longer in use
// but you can also manually free up memory with the free function or make  function
// sometimes zero values isnt good enough so initializing a constructor is a good idea
// go allows you to define your own types
type foo int

// creating more sophisticated types
// each item in a struct is called a field
// field names can also be exported
type bar struct {
	a int
	b string
}

func main() {
	fmt.Println("beyond the basics")
	// creating a object
	a := new(bar)
	a.a = 10
	a.b = "hello"
	fmt.Println("%v\n", a.a)
	fmt.Println("%s\n", a.b)
	// or if you want to print the whole object
	fmt.Println("%v\n", a)
}
