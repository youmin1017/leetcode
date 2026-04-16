package main

func main() {
}

func gcdOfStrings(s1 string, s2 string) string {
	if s1+s2 != s2+s1 {
		return ""
	}

	g := gcd(len(s1), len(s2))
	return s1[:g]
}

func gcd(a int, b int) int {
	if b == 0 {
		return a
	}
	return gcd(b, a%b)
}
