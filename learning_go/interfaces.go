package main

import "fmt"

// in other words, interfaces has overloaded things that it means
// a interface has a set of methods that is defined by a type
func main() {
	fmt.Println("interfaces")
}

type S struct {
	i int
}

func (s S) Get() int {
	return s.i
}

func (s S) Set(i int) {
	s.i = i
}

// we can also define a interface with a set of methods
type I interface {
	Get() int
	Set(int)
}

func f1(p I) {
	fmt.Println(p.Get())
	p.Set(10)
}
