// https://atcoder.jp/contests/abc418/tasks/abc418_a
package main

import (
	"fmt"
)

func main() {
	var n string
	var s string
	fmt.Scanf("%s", &n)
	fmt.Scanf("%s", &s)

	if len(s) < 3 {
		// It's not a tea!
		fmt.Printf("No\n")
		return
	}
	if shouldTea := s[len(s)-3:]; shouldTea != "tea" {
		// It's not a tea!
		fmt.Printf("No\n")
		return
	}
	fmt.Printf("Yes\n")
}
