package settings

import (
	"fmt"
	"log"
	"os"

	"github.com/guysherman/kittymux/kitty"
)

type QuickNavDatabase struct {
	QuickNavs map[string][]QuickNavHandle
	dao       IQuickNavDao `json:"-"`
}

type QuickNavHandle struct {
	EntryId   int
	EntryType kitty.WindowListEntryType
}

func NewQuickNavDatabase(dao IQuickNavDao) QuickNavDatabase {
	db, err := dao.Read(quickNavDatabasePath())
	if err != nil {
		log.Fatal(err)
		os.Exit(-9)
	}

	db.dao = dao
	return db
}

func quickNavDatabasePath() string {
	stateDir := GetStateDir()
	quicknavsPath := fmt.Sprintf("%s/quicknavs.json", stateDir)
	return quicknavsPath
}

func (d QuickNavDatabase) SetShortcut(key string, handle QuickNavHandle) QuickNavDatabase {
	// Each window should have only one shortcut key,
	// so find it and remove it from the database
	d = d.RemoveHandle(handle)

	// Then add it to the correct key
	d = d.addHandleToKey(key, handle)

	return d
}

func (d QuickNavDatabase) RemoveHandle(handle QuickNavHandle) QuickNavDatabase {
	// Find the key and index for the handle
	existingKey, existingIndex := d.findHandle(handle)

	// Then remove it
	if existingKey != "" && existingIndex > -1 {
		originalHandles := d.QuickNavs[existingKey]
		// If the handle we are about to remove is the only one for the key
		// we can just delete the whole key
		if len(originalHandles) == 1 {
			delete(d.QuickNavs, existingKey)
		} else {
			// Otherwise we splice the handle out of the array for that key
			newHandles := originalHandles[0:existingIndex]
			newHandles = append(newHandles, originalHandles[existingIndex+1:]...)
			d.QuickNavs[existingKey] = newHandles
		}
	}

	// It seems safe to always write the database out to a file
	d.dao.Write(d, quickNavDatabasePath())

	return d
}

func (d QuickNavDatabase) findHandle(handle QuickNavHandle) (string, int) {
	existingIndex := -1
	existingKey := ""

	// We operate on the loop invariant that a given handle appears at most once
	// in the entire map[string][]QuickNavHandle, thus when we find an instance,
	// we can bail out and deal with it
	for k, v := range d.QuickNavs {
		for i, h := range v {
			if h.EntryId == handle.EntryId && h.EntryType == handle.EntryType {
				existingIndex = i
				existingKey = k
				break
			}
		}

		if existingIndex != -1 {
			break
		}
	}
	return existingKey, existingIndex
}

func (d QuickNavDatabase) addHandleToKey(key string, handle QuickNavHandle) QuickNavDatabase {
	keyHandles := d.QuickNavs[key]
	keyHandles = append(keyHandles, handle)
	d.QuickNavs[key] = keyHandles

	// It seems safe to always write the database out to a file
	d.dao.Write(d, quickNavDatabasePath())

	return d
}

func (d QuickNavDatabase) ShortcutsByEntryId() map[string]string {
	result := map[string]string{}

	for shortcut, handles := range d.QuickNavs {
		for _, handle := range handles {
			entryType := ""
			switch handle.EntryType {
			case kitty.OsWindow:
				entryType = "o"
				break
			case kitty.Tab:
				entryType = "t"
				break
			case kitty.Window:
				entryType = "w"
				break
			}
			entryId := fmt.Sprintf("%s:%d", entryType, handle.EntryId)
			result[entryId] = shortcut
		}
	}
	return result
}
