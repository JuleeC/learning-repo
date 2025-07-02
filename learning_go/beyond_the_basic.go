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
type Mutex struct{}

func (m *Mutex) Lock()   {}
func (m *Mutex) Unlock() {}

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
	// we can call the methods like this
	// if we dont point to bar, its fine because go handles the pointer afterwards for us
	var a1 *bar
	a1.doSomething(10)
	// or like this
	doS(10, a1)
	// mutex struct exmaple
	type NewMutex Mutex // we can create a new type from an existing type
	// newmutex is the same as mutex but it doenst have any methods of the mutex type
	type PrintableMutex struct{ Mutex }
	// but printablemutex has all the methods of the mutex type
	// in go its called embedding
}
func (b *bar) doSomething(i int) {}

// its the same
func doS(i int, b *bar) {}
