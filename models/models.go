package models

import (
	"crypto/sha256"
	"encoding/json"
	"fmt"
	"math/rand"
	"time"
)

// Table struct
type Table struct {
	Meta    Meta        `json:"meta"`
	Header  Header      `json:"header"`
	Grid    []Cell      `json:"grid"`
	Objects interface{} `json:"objects"`
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

func CreateSampleTable() Table {
	t := Table{}
	t.CreateEmptyMeta()
	t.Header.CreateSampleHeader()
	for i := 0; i < 4; i++ {
		t.Grid = append(t.Grid, CreateSampleCell())
	}

	t.Objects = "this is a sample table for testing purposes"

	return t
}

// Meta struct
// data will be (over) written by the server
type Meta struct {
	Id        string `json:"id"`
	Timestamp int    `json:"timestamp"`
	Apiv      string `json:"apiv"`
}

func (t *Table) Qualify(hash string) {
	t.UpdateTimeStamp()
	t.Meta.Apiv = "2.1.0"
	t.Meta.Id = hash
}

// hash of the Grid, Header, Objects
func (t *Table) Hash() string {
	headerBytes, _ := json.Marshal(t.Header)
	gridBytes, _ := json.Marshal(t.Grid)
	objectsBytes, _ := json.Marshal(t.Objects)
	bytes := append(append(headerBytes[:], gridBytes[:]...), objectsBytes[:]...)
	hashed := sha256.Sum256(bytes)
	return fmt.Sprintf("%64x", hashed)
}

// Header has info that is unlikely to chage
type Header struct {
	Name    string      `json:"name"`
	Spatial Spatial     `json:"spatial"`
	Owner   Owner       `json:"owner"`
	Block   []string    `json:"block"`
	Mapping interface{} `json:"mapping"`
}

func (h *Header) CreateSampleHeader() {
	h.Name = "sample_table"
	h.Spatial.CreateSampleSpatialData(2, 2)
	h.Owner.CreateSampleOwner()
	h.Block = []string{"type", "rot"}

	m := make(map[int]string)
	m[0] = "RS"
	m[1] = "RM"
	m[2] = "RL"
	m[3] = "OS"
	m[4] = "OM"
	m[5] = "OL"

	h.Mapping = m
}

// Owner is the info to addres the ownership
type Owner struct {
	Name      string `json:"name"`
	Title     string `json:"title"`
	Institute string `json:"institute"`
}

func (o *Owner) CreateSampleOwner() {
	o.Name = "Yasushi Sakai"
	o.Title = "Research Assistant"
	o.Institute = "MIT Media Lab"
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

func (s *Spatial) CreateSampleSpatialData(r byte, c byte) {
	s.Nrows = r
	s.Ncols = c
	s.PhysicalLongitude = -71.08768
	s.PhysicalLatitude = 42.3608
	s.Longitude = -71.08768
	s.Latitude = 42.3608
	s.CellSize = 10.0
	s.Rotation = 0.0
}

// Cell is data for each grid cell, we don't
// know what will be inside prior
type Cell interface{}

func CreateSampleCell() Cell {
	return []int{rand.Intn(4), 90}
}

// OldTable is a table with formats before 2.0.0
type OldTable interface{}
