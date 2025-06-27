package main

import (
	"fmt"
)
//functions are declared with func
//syntax is : func(p type)name(parameters) return_type {}
// functions can be declared in any order because the compiler executes after scanning the whole file

//scope is the region of code that a variable is valid
// vars that are declared outisde of a function are global
// vars that are declared inside a function are local

var r int
var r1 bool

//we can assign the value of a function to a variable
func main() {
	fmt.Println("Hello World!")
  r2 := rec(10)
	fmt.Printf("%v",r2)
	r3 := func(i int ) {
		if i == 0 {
			fmt.Println("done")
		}
	}
	r3(55)
	// funcs as values are often used in maps
	var xs = map[int]func() int{
    1: func() int { return 10 },
    2: func() int { return 20 },
    3: func() int { return 30 },
	}
	fmt.Printf("%v\n",xs[1]())

}
func rec(n int) int {
	if n == 4 {
		return 1
	}
	//recursion
	rec(n - 1)
	return 0
}
