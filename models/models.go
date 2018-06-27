package models

import (
	"crypto/sha256"
	"fmt"
	"time"
)

// Table struct
type Table struct {
	Meta    Meta        `json:"meta"`
	Header  Header      `json:"header"`
	Grid    []Cell      `json:"grid"`
	Objects interface{} `json:"objects"`
}

// Meta struct
// data will be (over) written by the server
type Meta struct {
	Id        string `json:"id"`
	Timestamp int    `json:"timestamp"`
	Apiv      string `json:"apiv"`
}

// helper to create Empty Meta data
// func CreateEmptyMeta() Meta {
// 	now := time.Now()
// 	ts := int(now.UnixNano() / 1000000)
// 	return Meta{"", ts, "2.0.0"}
// }

func (t *Table) CreateEmptyMeta() {
	now := time.Now()
	ts := int(now.UnixNano() / 1000000)
	t.Meta = Meta{"", ts, "2.0.0"}
}

func (t *Table) UpdateTimeStamp() {
	t.Meta.Timestamp = int(time.Now().UnixNano() / 1000000)
}

func (t *Table) QualifyTableData() {
	t.UpdateTimeStamp()
	t.Meta.Apiv = "2.1.0"
	t.HashData()
}

func (m *Meta) HashData(data string) {
	headerM := json.Marshall(t.Header)

	hash := sha256.Sum256([]byte(data))
	m.Id = fmt.Sprintf("%64x", hash)
}

// Header has info that is unlikely to chage
type Header struct {
	Name    string      `json:"name"`
	Spatial Spatial     `json:"spatial"`
	Owner   Owner       `json:"owner"`
	Block   []string    `json:"block"`
	Mapping interface{} `json:"mapping"`
}

// Owner is the info to addres the ownership
type Owner struct {
	Name      string `json:"name"`
	Title     string `json:"title"`
	Institute string `json:"institute"`
}

type Spatial struct {
	Nrows             byte    `json:"nrows"`
	Ncols             byte    `json:"ncols"`
	PhysicalLongitude float64 `json:"physical_longitude"`
	PhysicalLatitude  float64 `json:"physical_latitude"`
	Longitude         float64 `json:"longitude"`
	Latitude          float64 `json:"latitude"`
	CellSize          float64 `json:"cellSize"`
	Rotation          float64 `json:"rotation"`
}

// Cell is data for each grid cell, we don't
// know what will be inside prior
type Cell interface{}

// OldTable is a table with formats before 2.0.0
type OldTable interface{}
