package main

import (
	"encoding/json"
	"github.com/CityScope/CS_CityIO_Backend/models"
	"github.com/labstack/echo"
	"log"
	"net/http"
)

func main() {
	port := "8080"
	prefix := "/api"
	router := echo.New()
	tables := make(map[string]interface{})

	///////////////////////////////////////
	router.GET("/", func(c echo.Context) error {
		return c.Redirect(http.StatusMovedPermanently, "http://cityscope.media.mit.edu/CS_CityIO_Frontend/")
	})
	///////////////////////////////////////
	router.POST(prefix+"/table/update/:tableName", func(c echo.Context) error {
		data := make(map[string]interface{})
		tableName := c.Param("tableName")

		err := json.NewDecoder(c.Request().Body).Decode(&data)

		if err != nil {
			log.Printf("error: %v\n", err.Error())
		}

		byteData, _ := json.Marshal(data)

		table := models.Table{}

		err = json.Unmarshal(byteData, &table)
		if err != nil {
			log.Printf("[%v]: unvalid: %v\n", tableName, err.Error())
			tables[tableName] = data
		} else {
			log.Printf("[%v]: valid type \n", tableName)
			table.UpdateTimeStamp()
			tables[tableName] = table
		}

		return c.JSON(http.StatusOK,
			map[string]string{"tableName": "done"})

	})
	///////////////////////////////////////
	router.GET(prefix+"/table/clear/:tableName", func(c echo.Context) error {
		tableName := c.Param("tableName")

		//TODO: do we want to delete it? perhaps inactivate it?
		delete(tables, tableName)
		return c.JSON(http.StatusOK,
			map[string]string{"status": "deleted " + tableName})
	})
	///////////////////////////////////////
	router.GET(prefix+"/table/:tableName", func(c echo.Context) error {
		tableName := c.Param("tableName")
		table, ok := tables[tableName]
		if ok {
			return c.JSON(http.StatusOK, table)
		} else {
			return c.JSON(http.StatusNotFound,
				map[string]string{"status": "table not found"})
		}
	})
	///////////////////////////////////////
	router.GET(prefix+"/tables/list", func(c echo.Context) error {
		tableList := make([]string, 0, len(tables))
		for k := range tables {
			tableList = append(tableList, k)
		}
		return c.JSON(http.StatusOK, tableList)
	})
	log.Fatal(router.Start(":" + port))
}
