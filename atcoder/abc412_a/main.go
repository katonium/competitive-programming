// https://atcoder.jp/contests/abc412/tasks/abc412_a
package main

import "fmt"

func main() {
	var n int
	fmt.Scanf("%d", &n)
	moreThanExpected := 0
	for i := 0; i < n; i++ {
		var a int16
		var b int16
		fmt.Scanf("%d %d", &a, &b)

		if a < b {
			moreThanExpected++
		}
	}
	fmt.Printf("%d\n", moreThanExpected)
}
