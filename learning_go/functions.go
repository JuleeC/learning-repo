package main

import (
	"fmt"
)

// functions are declared with func
// syntax is : func(p type)name(parameters) return_type {}
// functions can be declared in any order because the compiler executes after scanning the whole file

// scope is the region of code that a variable is valid
// vars that are declared outisde of a function are global
// vars that are declared inside a function are local

var (
	r  int
	r1 bool
)

// we can assign the value of a function to a variable
func main() {
	fmt.Println("Hello World!")
	r2 := rec(10)
	fmt.Printf("%v\n", r2)
	r3 := func(i int) {
		if i == 0 {
			fmt.Println("done")
		}
	}
	r3(55)
	// funcs as values are often used in maps
	xs := map[int]func() int{
		1: func() int { return 10 },
		2: func() int { return 20 },
		3: func() int { return 30 },
	}
	fmt.Printf("%v\n", xs[1]())

	//because functions are values we can pass them as arguments and use them as callbacks
	//func callback(y int, f func(int)) {
	//  f(y)
	//}

	// variadic arguments are used to pass a variable number of arguments to a function
	// func myfunc(args ...int) {}
	// if we want to it iterate over the arguments we can use a for loop
	// for _, v := range args {}
	// when we pass a variadic argument to a function we can use the ... operator
	// slice tricks also works with variadic arguments
}

// deferred code is executed after the function returns
// after returning anything the defer code is executed
// if many things are deferred, the defer code is executed in the reverse order. its like a stack
func defertest(i int) bool {
	defer fmt.Println("defer")
	if i == 0 {
		return false
	}
	return true
}

// we can also defer a function
// the output isnt 0, its 1
func fm() (ret int) {
	defer func() {
		ret++
		fmt.Println("defer")
	}()
	return 0
}

func rec(n int) int {
	if n == 4 {
		return 1
	}
	// recursion
	rec(n - 1)
	return 0
}
