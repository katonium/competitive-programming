// https://atcoder.jp/contests/abc408/tasks/abc408_a
package main

import (
	"fmt"
)

func main() {
	var n, s int
	fmt.Scanf("%d %d", &n, &s)

	tprev := 0
	for i := 0; i < n; i++ {
		var ti int
		_, err := fmt.Scan(&ti)

		if err != nil {
			panic(err)
		}

		if ti-tprev > s {
			fmt.Printf("No\n")
			return
		}
		tprev = ti
	}

	fmt.Printf("Yes\n")
}
