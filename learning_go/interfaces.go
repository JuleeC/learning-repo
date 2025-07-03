package main

import "fmt"

// in other words, interfaces has overloaded things that it means
// a interface has a set of methods that is defined by a type
func main() {
	fmt.Println("interfaces")
	f1(&S{10})
	f1(&R{10})
}

type S struct {
	i int
}
type R struct {
	i int
}

func (p *R) Get() int {
	return p.i
}

func (p *R) Set(i int) {
	p.i = i
}

func (s *S) Get() int {
	return s.i
}

func (s *S) Set(i int) {
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
	// we can only use .type in a switch statement
	// we can also use a type assertion to check if the type is the same
	switch t := p.(type) {
	case *S:
		fmt.Println(t.Get())
	case *R:
		fmt.Println(t.Get())

	}
}
