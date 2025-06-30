package main

import (
	"fmt"

	"package/packages/even"
)

// capital letter = gloabl scope
// small letter = local scope
// underscore = local scope
// go want to use CamelCase over words with underscores
// go packaage names are lowercase
// writing go test is customary and should be done
// go test
func main() {
	i := 10
	fmt.Println("packages")
	fmt.Printf("is %d  even? %v", i, even.Even(i))
	// this doesnt work because odd is not in the packages scope
	// fmt.Println(even.odd(2))
}
