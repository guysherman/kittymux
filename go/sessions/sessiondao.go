package sessions

import (
	"encoding/json"
	"fmt"
	"os"
)

type ISessionDao interface {
	Read(filepath string) (Session, error)
	Write(session Session, filepath string) error
}

type SessionDao struct{}

func (s *SessionDao) Read(filepath string) (Session, error) {
	bytes, err := os.ReadFile(filepath)
	if err != nil {
		return Session{}, err
	}

	var result Session

	err = json.Unmarshal(bytes, &result)
	if err != nil {
		return Session{}, err
	}

	return result, nil
}

func (s *SessionDao) Write(session Session, filepath string) error {
	bytes, err := json.Marshal(session)
	if err != nil {
		return err
	}

	file, err := os.Create(filepath)
	if err != nil {
		return err
	}

	_, err = file.Write(bytes)
	if err != nil {
		return err
	}

	err = file.Close()
	if err != nil {
		return err
	}

	return nil
}

func GetStateDir() string {
	if os.Getenv("KITTYMUX_STATE_DIR") != "" {
		return os.Getenv("KITTYMUX_STATE_DIR")
	} else if os.Getenv("XDG_STATE_HOME") != "" {
		return fmt.Sprintf("%s/kittymux", os.Getenv("XDG_STATE_HOME"))
	} else {
		return "~/.local/state/kittymux"
	}
}
