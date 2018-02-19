package main

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
)

var tables []string

func handler(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, "hello, %s", r.URL.Path[1:])
}

func tableListHandler(w http.ResponseWriter, r *http.Request) {

	data, _ := json.Marshal(tables)

	fmt.Println(data)

	w.Header().Set("Content-Type", "application/json")
	w.Write(data)
}

func main() {

	tables = append(tables, "fake-table")
	tables = append(tables, "virtual-table")
	tables = append(tables, "dull-table")

	fmt.Printf("starting server")
	http.HandleFunc("/api/table/list/", tableListHandler)

	log.Fatal(http.ListenAndServe(":8080", nil))
}
