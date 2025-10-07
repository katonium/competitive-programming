// https://atcoder.jp/contests/abc409/tasks/abc409_a
package main

import (
	"fmt"
)

func main() {
	var n int
	fmt.Scanf("%d", &n)
	var t, a string
	fmt.Scanf("%s", &t)
	fmt.Scanf("%s", &a)
	for i := 0; i < n; i++ {
		if t[i] == 'o' && a[i] == 'o' {
			fmt.Printf("Yes\n")
			return
		}
	}
	fmt.Printf("No\n")
}
