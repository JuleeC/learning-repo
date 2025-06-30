package main

import (
	"fmt"

	"package/packages/even"
)

// capital letter = gloabl scope
// small letter = local scope
// underscore = local scope
func main() {
	fmt.Println("packages")
	fmt.Println(even.Even(2))
	// this doesnt work because odd is not in the packages scope
	// fmt.Println(even.odd(2))
}
