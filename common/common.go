package common

import (
	"errors"
	"fmt"
	"io/ioutil"
	"net/http"
	"os"
	"strings"
	"time"
)

func GetInput(year string, day string) {
	// Create file sessionCookie with session Cookie
	// from https://github.com/jonathanpaulson/AdventOfCode/blob/master/get_input.py
	// # You can find SESSION by using Chrome tools:
	// # 1) Go to https://adventofcode.com/2022/day/1/input
	// # 2) right-click -> inspect -> click "Network".
	// # 3) Refresh
	// # 4) Click click
	// # 5) Click cookies
	// # 6) Grab the value for session. Fill it in.
	b_session, err := os.ReadFile("./common/sessionCookie")
	if err != nil {
		fmt.Println(err)
		fmt.Println("Most likely missing sessionCookie file inside common, exiting")
		panic(err)
	}
	session := string(b_session)
	folder_path := fmt.Sprintf("./%v/inputs", year)
	if _, err := os.Stat(folder_path); errors.Is(err, os.ErrNotExist) {
		fmt.Println("Folder doesn't exists. Creating")
		_ = os.MkdirAll(folder_path, os.ModePerm)
	} else {
		fmt.Println("Folder already exists. Skipping creation")
	}
	file_name := fmt.Sprintf("%v/%v-%v.in", folder_path, year, day)
	if _, err := os.Stat(file_name); err != nil {
		fmt.Printf("File %v doesn't exist, creating\n", file_name)
		client := &http.Client{}
		url := fmt.Sprintf("https://adventofcode.com/%v/day/%v/input", year, day)
		req, _ := http.NewRequest(http.MethodGet, url, nil)
		req.AddCookie(&http.Cookie{Name: "session", Value: session})
		resp, _ := client.Do(req)
		body, _ := ioutil.ReadAll(resp.Body)
		resp.Body.Close()

		file, fileErr := os.Create(file_name)
		if fileErr != nil {
			fmt.Println(fileErr)
			panic(fileErr)
		}
		fmt.Fprint(file, strings.TrimSuffix(strings.TrimSpace(string(body)), "\n"))
	} else {
		fmt.Printf("File %v already exists, skipping creation\n", file_name)
	}
}


func Timer(name string) func() {
    start := time.Now()
    return func() {
        fmt.Printf("%s took %v\n", name, time.Since(start))
    }
}