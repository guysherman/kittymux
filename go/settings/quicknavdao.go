package settings

import (
	"encoding/json"
	"log"
	"os"
)

type IQuickNavDao interface {
	Read(filepath string) (QuickNavDatabase, error)
	Write(quickNavs QuickNavDatabase, filepath string) error
}

type QuickNavDao struct{}

func (qnd *QuickNavDao) Read(filepath string) (QuickNavDatabase, error) {
	bytes, err := os.ReadFile(filepath)
	if err != nil {
		if e, ok := err.(*os.PathError); ok && e.Err.Error() == "no such file or directory" {
			log.Print("WARN: No quicknavs found")
			return QuickNavDatabase{
				QuickNavs: map[string][]QuickNavHandle{},
			}, e
		} else if err != nil {
			log.Fatal(err)
			os.Exit(-8)
		}
	}

	db := QuickNavDatabase{
		QuickNavs: map[string][]QuickNavHandle{},
	}
	err = json.Unmarshal(bytes, &db)
	if err != nil {
		log.Fatal(err)
	}
	return db, nil
}

func (qnd *QuickNavDao) Write(quickNavs QuickNavDatabase, filepath string) error {
	bytes, err := json.Marshal(quickNavs)
	if err != nil {
		log.Fatal(err)
		os.Exit(-5)
	}

	f, err := os.Create(filepath)
	if err != nil {
		log.Fatal(err)
		os.Exit(-6)
	}
	_, err = f.Write(bytes)
	if err != nil {
		log.Fatal(err)
		os.Exit(-7)
	}

	f.Close()
	return nil
}
