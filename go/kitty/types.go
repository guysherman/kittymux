package kitty

type KittyOsWindow struct {
	Id                 int
	Is_focused         bool
	Platform_window_id int
	Tabs               []KittyTab
}

type KittyTab struct {
	Id                    int
	Is_focused            bool
	Layout                string
	Title                 string
	Windows               []KittyWindow
	Active_window_history []int
}

type KittyWindow struct {
	Id                   int
	Is_focused           bool
	Is_self              bool
	Lines                int
	Pid                  int
	Title                string
	Cwd                  string
	Cmdline              []string
	Env                  map[string]string
	Foreground_processes []KittyForegroundProcessHandle
}

type KittyForegroundProcessHandle struct {
	Pid     int
	Cwd     string
	Cmdline []string
}
