package main

import (
	"fmt"
)
//functions are declared with func
//syntax is : func(p type)name(parameters) return_type {}
// functions can be declared in any order because the compiler executes after scanning the whole file
func main() {
	fmt.Println("Hello World!")
}
func rec(n int) int {
	if n == 0 {
		return 1
	}
	rec(n - 1)
	return 0

}
