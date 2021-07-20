package main

import "fmt"

func main() {
	var r,pi float64
	pi=3.14159
	fmt.Scan(&r)
	fmt.Printf("VOLUME = %.3f\n", (4/3.0)*pi*r*r*r)
}
