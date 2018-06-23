package main

import (
	"encoding/json"
	"fmt"
	"net/http"

	"github.com/CityScope/CS_CityIO_Backend/models"
	"github.com/gin-gonic/gin"
)

func main() {

	prefix := "/dev"
	router := gin.Default()
	tables := make(map[string]interface{})

	///////////////////////////////////////
	router.POST(prefix+"/api/table/update/:tableName", func(c *gin.Context) {
		var data interface{}

		if err := c.ShouldBindJSON(&data); err == nil {
			// go table.UpdateTimeStamp()
			byteData, _ := json.Marshal(data)

			table := models.Table{}

			err := json.Unmarshal(byteData, &table)

			// if reflect.DeepEqual(table, by)

			tableName := c.Param("tableName")
			if err != nil {
				fmt.Println(err)
				tables[tableName] = data
			} else {
				table.UpdateTimeStamp()
				tables[tableName] = table
			}

		} else {
			fmt.Println(err)
			c.JSON(http.StatusBadRequest, gin.H{"error": err})
		}
		// tableName := c.Param("tableName")
		c.JSON(http.StatusOK, gin.H{"tableName": "done"})

	})

	///////////////////////////////////////
	router.GET(prefix+"/api/table/:tableName", func(c *gin.Context) {
		tableName := c.Param("tableName")
		table, ok := tables[tableName]
		if ok {
			c.JSON(http.StatusOK, table)
		} else {
			c.JSON(http.StatusNotFound, gin.H{"status": "table not found"})
		}
	})

	///////////////////////////////////////
	router.GET(prefix+"/api/table/clear/:tableName", func(c *gin.Context) {
		tableName := c.Param("tableName")

		//TODO: do we want to delete it? perhaps inactivate it?
		delete(tables, tableName)
		c.JSON(http.StatusOK, "deleted "+tableName+".")
	})

	///////////////////////////////////////
	router.GET(prefix+"/api/tables/list", func(c *gin.Context) {
		tableList := make([]string, 0, len(tables))
		for k := range tables {
			tableList = append(tableList, k)
		}

		c.JSON(http.StatusOK, tableList)
	})

	router.Run(":8081")
}
