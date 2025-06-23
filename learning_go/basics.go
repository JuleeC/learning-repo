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
// we can assume that errrors are a type just like ints and floats
func main()  {
	fmt.Printf("Hello World\n")
	fmt.Printf("testing if commiting works\n")
	//it is not necessary to declare a type
	//you can do this also but only in a function
	a1 := 10
//	b2 := true
//	c3 := "hello"

	// if statements
	if a1 > 10 {
		fmt.Printf("a1 is greater than 10\n")
	} else {
		fmt.Printf("a1 is less than 10\n")
	}
	//if and switch statements can have an initalization
	if err := doSomething(); err != 0 {
		fmt.Printf("value")
	}
	//go uses 3 types if loops
	// for init; condition; post 
	// for condition
	// for {} --endless loop

	// with break you can break out of a loop
	// we can break also labels
	J: for i := 0; i < 10; i++ {
			for j := 0; j < 10; j++ {
					if j == 5 {
						break J
					}
					fmt.Printf("i is %d and j is %d\n", i, j)
					}
			}
			// continue is used to skip the rest of the loop and works also with labels

			// range is used to iterate over arrays and slices
			list := []string {"a", "b", "c"}
			for k, v := range list {
				fmt.Printf("k is %d and v is %d \n", k, v)
			}
			//switch is used to compare a value against a list of values
			switch {
			case a1 > 10:
				fmt.Printf("a1 is greater than 10\n")
			case a1 < 10:
				fmt.Printf("a1 is less than 10\n")
				//if we want to skip a case we can use fallthrough
			case a1 == 10: fallthrough
				//default is used if none of the cases match
			default:
				fmt.Printf("a1 is equal to 10\n")
			}
			//also a varient
			switch a1 {
				case 0,1:
					fmt.Printf("a1 is 0 or 1\n")
				}
				//arrays
				var arr [3]int
				arr[0] = 1
				arr[1] = 2

				//arrays are values so if you assign another array to it it copies the values and it will be the same array
}

//goto is used to jump to a label
func label_goto_example()  {
	i := 0
	Here:
  	fmt.Println(i)		
		i++
		goto Here
	}

func doSomething() int32 {
	return 10
}





