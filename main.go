package main

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"
	"time"

	"github.com/CityScope/CS_CityIO_Backend/models"
	"github.com/google/logger"
	"github.com/labstack/echo"
	"github.com/labstack/echo/middleware"
)

var port string = "8080"
var prefix string = "/api"

var tables map[string]interface{} = make(map[string]interface{})

// GET "/"
func getFrontend(c echo.Context) error {
	logger.Info("GET frontend")
	return c.Redirect(http.StatusMovedPermanently, "http://cityscope.media.mit.edu/CS_CityIO_Frontend/")
}

// GET "/api"
func getAPI(c echo.Context) error {
	logger.Info("GET request " + prefix)
	return c.JSON(http.StatusOK, "OK")
}

// GET "/api/tables/list"
func listTables(c echo.Context) error {
	logger.Info("GET /table/list")
	tableList := make([]string, 0, len(tables))
	baseUrl := "https://cityio.media.mit.edu/api/table/"
	for k := range tables {
		tableList = append(tableList, baseUrl+k)
	}
	return c.JSON(http.StatusOK, tableList)
}

// GET "api/table/:tableName"
func getTable(c echo.Context) error {
	tableName := c.Param("tableName")
	logger.Infof("GET /table/%v", tableName)
	table, ok := tables[tableName]

	if ok {
		logger.Infof("  found table: %v", tableName)
		return c.JSON(http.StatusOK, table)
	} else {
		logger.Infof("  could not find table: %v", tableName)
		return c.JSON(http.StatusNotFound,
			map[string]string{"status": "table not found"})
	}
}

// GET "api/table/clear/:tableName"
func clearTable(c echo.Context) error {
	tableName := c.Param("tableName")
	logger.Infof("GET /table/clear/%v", tableName)
	//TODO: do we want to delete it? perhaps inactivate it?
	delete(tables, tableName)
	return c.JSON(http.StatusOK,
		map[string]string{"status": "deleted " + tableName})
}

// POST "api/table/update/:tableName"
func postTable(c echo.Context) error {

	data := make(map[string]interface{})
	tableName := c.Param("tableName")
	logger.Infof("POST /table/update/%v", tableName)

	err := json.NewDecoder(c.Request().Body).Decode(&data)

	if err != nil {
		log.Printf("error: %v\n", err.Error())
		logger.Errorf("error decoding table data from json: %v\n", err.Error())
	}

	byteData, _ := json.Marshal(data)

	table := models.Table{}

	err = json.Unmarshal(byteData, &table)
	if err != nil {
		log.Printf("[%v]: invalid type: %v\n", tableName, err.Error())
		tables[tableName] = data
	} else {
		log.Printf("[%v]: valid type \n", tableName)

		hash := table.Hash()
		update := true

		// don't update when the hash is the same
		if lastTable, ok := tables[tableName]; ok {
			lt, yep := lastTable.(models.Table)
			if yep && hash == lt.Meta.Id {
				update = false
			}
		}

		if update {
			table.Qualify(hash)
			tables[tableName] = table
		}
	}

	logger.Info("POST SUCCESS")
	return c.JSON(http.StatusOK,
		map[string]string{tableName: "done"})
}

func main() {

	t := time.Now()
	time := fmt.Sprintf("%d-%02d-%02d_%02d:%02d:%02d",
		t.Year(),
		t.Month(),
		t.Day(),
		t.Hour(),
		t.Minute(),
		t.Second())

	logPath := fmt.Sprintf("./log/cityio_%v.log", time)
	lf, err := os.OpenFile(logPath, os.O_CREATE|os.O_WRONLY|os.O_APPEND, 0660)

	if err != nil {
		logger.Fatalf("Failed to open log file: %v", err)
	}

	defer lf.Close()
	defer logger.Init("Logger", false, false, lf).Close()

	//////////////////////////////////////////////

	router := echo.New()

	tables = make(map[string]interface{})

	router.Use(middleware.CORSWithConfig(middleware.CORSConfig{
		AllowOrigins: []string{"*"},
		AllowHeaders: []string{
			echo.HeaderOrigin,
			echo.HeaderContentType,
			echo.HeaderAccept},
		AllowMethods: []string{echo.GET, echo.POST},
	}))

	// Frontend redirects to city IO frontend
	router.GET("/", getFrontend)

	// API endpoints

	router.GET(prefix, getAPI)

	router.POST(prefix+"/table/update/:tableName", postTable)

	router.GET(prefix+"/table/clear/:tableName", clearTable)

	router.GET(prefix+"/table/:tableName", getTable)

	router.GET(prefix+"/tables/list", listTables)

	logger.Info("server started")
	logger.Fatal(router.Start(":" + port))
}
