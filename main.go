package main

import (
	"fmt"
	"io"
	"os"
	"regexp"
	"strings"
)

const bq = "`"

var patterns = []string{
	`(?:"|'|=|` + bq + `)(((?:[a-zA-Z]{1,10}://|//)[^"'/]{1,}\.[a-zA-Z]{2,}[^"']{0,})|((?:/|\.\./|\./)[^` + bq + `"'><,;| *()(%%$^/\\\[\]][^` + bq + `"'><,;|()]{1,})|([a-zA-Z0-9_\-/]{1,}/[a-zA-Z0-9_\-/.]{1,}\.(?:[a-zA-Z]{1,4}|action)(?:[\?|#][^"|']{0,}|))|([a-zA-Z0-9_\-/]{1,}/[a-zA-Z0-9_\-/]{3,}(?:[\?|#][^"|']{0,}|))|([a-zA-Z0-9_\-]{1,}\.(?:php|asp|aspx|jsp|json|action|html|js|txt|xml)(?:[\?|#][^"|']{0,}|)))(?:"|'| |` + bq + `)|(?:src=["']|href=["'])(([^'"]*))(?:"|'| |` + bq + `)`,
}

func main() {
	// Read input
	content, err := io.ReadAll(os.Stdin)

	if err != nil {
		fmt.Fprintf(os.Stderr, "Can't read stdin: %v\n", err)
		os.Exit(1)
	}

	printed := make(map[string]bool)

	for _, p := range patterns {
		re := regexp.MustCompile(p)
		matches := re.FindAllStringSubmatch(string(content), -1)

		for _, m := range matches {
			lin := strings.TrimSpace(m[1])
			if lin == "" {
				lin = strings.TrimSpace(m[8])
			}
			if lin != "" && !printed[lin] {
				fmt.Println(lin)
				printed[lin] = true
			}
		}

	}

}
