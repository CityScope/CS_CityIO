package main

import (
	"bytes"
	"encoding/json"
	"github.com/CityScope/CS_CityIO_Backend/models"
	"github.com/labstack/echo"
	"github.com/stretchr/testify/assert"
	"net/http"
	"net/http/httptest"
	"testing"
)

// isolated tests for each handler functions

func TestSimpleGet(t *testing.T) {
	e := echo.New()
	req := httptest.NewRequest(http.MethodGet, "/", nil)
	rec := httptest.NewRecorder()

	c := e.NewContext(req, rec)

	if assert.NoError(t, getAPI(c)) {
		assert.Equal(t, http.StatusOK, rec.Code)
		assert.Equal(t, "\"OK\"", rec.Body.String())
	}
}

func TestGetFrontend(t *testing.T) {
	e := echo.New()
	req := httptest.NewRequest(http.MethodGet, "/", nil)
	rec := httptest.NewRecorder()

	c := e.NewContext(req, rec)

	if assert.NoError(t, getFrontend(c)) {
		assert.Equal(t, http.StatusMovedPermanently, rec.Code)
	}
}

func TestGetListTables(t *testing.T) {
	e := echo.New()
	req := httptest.NewRequest(http.MethodGet, "/", nil)
	rec := httptest.NewRecorder()

	c := e.NewContext(req, rec)

	listTables(c)

	var tables map[string]interface{}

	err := json.Unmarshal(rec.Body.Bytes(), &tables)

	if err == nil {
		assert.Equal(t, &tables, "[]")
	}
}

func TestPostTable(t *testing.T) {
	e := echo.New()

	// make a sample table
	table := models.CreateSampleTable()
	byteData, _ := json.Marshal(table)

	// make new request
	req := httptest.NewRequest(
		http.MethodPost,
		"/api/table/update/:tableName",
		bytes.NewReader(byteData))

	rec := httptest.NewRecorder()

	c := e.NewContext(req, rec)
	c.SetPath("api/table/update/:tableName")
	c.SetParamNames("tableName")
	c.SetParamValues("test_table")

	postTable(c)

	result := make(map[string]string)
	err := json.Unmarshal(rec.Body.Bytes(), &result)

	if err == nil {
		assert.Equal(t, result["test_table"], "done")
	}
}

func TestGetTable(t *testing.T) {
	table := models.CreateSampleTable()

	tables[table.Header.Name] = table

	e := echo.New()
	req := httptest.NewRequest(http.MethodGet, "/", nil)
	rec := httptest.NewRecorder()

	c := e.NewContext(req, rec)
	c.SetPath("api/table/:tableName")
	c.SetParamNames("tableName")
	c.SetParamValues(table.Header.Name)

	getTable(c)

	result := models.Table{}

	err := json.Unmarshal(rec.Body.Bytes(), &result)
	delete(tables, "test_table")

	if err == nil {
		assert.Equal(t, result.Header.Name, table.Header.Name)
	}

}

func TestClearTable(t *testing.T) {
	table := models.CreateSampleTable()
	tables[table.Header.Name] = table

	e := echo.New()
	req := httptest.NewRequest(http.MethodGet, "/", nil)
	rec := httptest.NewRecorder()

	c := e.NewContext(req, rec)
	c.SetPath("api/table/clear/:tableName")
	c.SetParamNames("tableName")
	c.SetParamValues(table.Header.Name)

	clearTable(c)

	assert.Equal(t, len(tables), 0)
}

// one example pass of
// post, list, get, clear

func TestSinglePass(t *testing.T) {

	table := models.CreateSampleTable()
	byteData, _ := json.Marshal(table)

	// POST a table
	e := echo.New()
	req := httptest.NewRequest(http.MethodPost, "/", bytes.NewReader(byteData))
	rec := httptest.NewRecorder()

	c := e.NewContext(req, rec)
	c.SetPath("api/table/update/:tableName")
	c.SetParamNames("tableName")
	c.SetParamValues(table.Header.Name)

	postTable(c)

	postResult := make(map[string]string)
	err := json.Unmarshal(rec.Body.Bytes(), &postResult)

	if err != nil {
		assert.Equal(t, postResult[table.Header.Name], "done")
	}

	// list
	req = httptest.NewRequest(http.MethodGet, "/", nil)
	rec = httptest.NewRecorder()

	c = e.NewContext(req, rec)

	listTables(c)

	var listResult []string

	err = json.Unmarshal(rec.Body.Bytes(), &listResult)

	if err != nil {
		assert.Equal(t, listResult[0], table.Header.Name)
	}

	// GET a table
	req = httptest.NewRequest(http.MethodGet, "/", nil)
	rec = httptest.NewRecorder()

	c = e.NewContext(req, rec)
	c.SetPath("api/table/:tableName")
	c.SetParamNames("tableName")
	c.SetParamValues(table.Header.Name)

	getTable(c)

	getResult := models.Table{}

	err = json.Unmarshal(rec.Body.Bytes(), &getResult)

	if err != nil {
		assert.Equal(t, getResult.Header.Name, table.Header.Name)
	}

	// clear a table
	req = httptest.NewRequest(http.MethodGet, "/", nil)
	rec = httptest.NewRecorder()

	c = e.NewContext(req, rec)
	c.SetPath("api/table/clear/:tableName")
	c.SetParamNames("tableName")
	c.SetParamValues(table.Header.Name)

	clearTable(c)

	clearResult := []string{}

	err = json.Unmarshal(rec.Body.Bytes(), &clearResult)

	if err != nil {
		assert.Equal(t, len(clearResult), 0)
	}
}
