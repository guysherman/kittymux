package sessions

import (
	"os"
	"testing"

	"github.com/guysherman/kittymux/settings"
	. "github.com/smartystreets/goconvey/convey"
)

func TestSessionDao(t *testing.T) {
	Convey("serialize then deserialize yields equivalent object", t, func() {
		testFileName := "test_session_dao_test.json"
		sd := &SessionDao{}

		session := Session{
			Title:       "Test Session",
			ShortcutKey: "a",
			Windows: []Window{
				{
					Title:       "Test Window",
					ShortcutKey: "b",
					ForegroundProcess: ProcessHandle{
						Args: []string{"a", "b", "c"},
						Cwd:  "/foo",
					},
					Cwd: "/foo",
				},
			},
		}

		err := sd.Write(session, testFileName)
		So(err, ShouldBeNil)

		result, err := sd.Read(testFileName)
		So(err, ShouldBeNil)
		So(result, ShouldResemble, session)

		os.Remove(testFileName)
	})

	Convey("Generates correct state dir", t, func() {
		Convey("when KITTYMUX_STATE_DIR is set", func() {
			expectedStateDir := "/home/kitty/kittymux_state"
			os.Setenv("KITTYMUX_STATE_DIR", expectedStateDir)
			stateDir := settings.GetStateDir()
			So(stateDir, ShouldEqual, expectedStateDir)

			os.Unsetenv("KITTYMUX_STATE_DIR")
		})

		Convey("when XDG_STATE_HOME is set", func() {
			expectedStateDir := "/home/kitty/state/kittymux"
			os.Setenv("XDG_STATE_HOME", "/home/kitty/state")
			stateDir := settings.GetStateDir()
			So(stateDir, ShouldEqual, expectedStateDir)

			os.Unsetenv("XDG_STATE_HOME")
		})

		Convey("when none is set", func() {
			expectedStateDir := "~/.local/state/kittymux"
			stateDir := settings.GetStateDir()
			So(stateDir, ShouldEqual, expectedStateDir)
		})
	})
}
