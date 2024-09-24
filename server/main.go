package main

import (
	"encoding/json"
	"io"
	"log"
	"net/http"
	"strings"

	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
)

type Entry struct {
	CodeFile                string `json:"code_file"`
	CodeFunc                string `json:"code_func"`
	CodeLine                string `json:"code_line"`
	Message                 string `json:"message"`
	Priority                string `json:"priority"`
	SyslogFacility          string `json:"syslog_facility"`
	SyslogIdentifier        string `json:"syslog_identifier"`
	TID                     string `json:"tid"`
	AuditLoginUID           string `json:"_audit_loginuid"`
	AuditSession            string `json:"_audit_session"`
	BootID                  string `json:"_boot_id"`
	CapEffective            string `json:"_cap_effective"`
	Cmdline                 string `json:"_cmdline"`
	Comm                    string `json:"_comm"`
	Exe                     string `json:"_exe"`
	GID                     string `json:"_gid"`
	Hostname                string `json:"_hostname"`
	MachineID               string `json:"_machine_id"`
	PID                     string `json:"_pid"`
	RuntimeScope            string `json:"_runtime_scope"`
	SourceRealtimeTimestamp string `json:"_source_realtime_timestamp"`
	SystemdCgroup           string `json:"_systemd_cgroup"`
	SystemdOwnerUID         string `json:"_systemd_owner_uid"`
	SystemdSlice            string `json:"_systemd_slice"`
	SystemdUnit             string `json:"_systemd_unit"`
	SystemdUserSlice        string `json:"_systemd_user_slice"`
	SystemdUserUnit         string `json:"_systemd_user_unit"`
	Transport               string `json:"_transport"`
	UID                     string `json:"_uid"`
}

var entries = map[string][]Entry{}

func sendLogs(c echo.Context) error {
	id := c.Param("id")

	data, err := io.ReadAll(c.Request().Body)
	if err != nil {
		return err
	}

	data = data[1 : len(data)-1]
	data = []byte(strings.ReplaceAll(string(data), "\\", ""))

	var entry Entry
	if err := json.Unmarshal(data, &entry); err != nil {
		return err
	}

	entries[id] = append(entries[id], entry)

	return c.NoContent(http.StatusOK)
}

type Settings struct {
	Priorities []string `json:"priorities"`
}

func getSettings(c echo.Context) error {
	// id := c.Param("id") // Get the device ID from the URL

	settings := Settings{
		Priorities: []string{
			"4",
		},
	}

	return c.JSON(http.StatusOK, settings)
}

func registerHeartbeat(c echo.Context) error {
	// id := c.Param("id") // Get the device ID from the URL

	return nil
}

func main() {
	server := echo.New()

	server.Use(middleware.Logger())
	server.Use(middleware.Recover())

	server.POST("/api/v1/device/:id/logs", sendLogs)
	server.GET("/api/v1/device/:id/settings", getSettings)
	server.POST("/api/v1/device/:id/heartbeat", registerHeartbeat)

	log.Fatal(server.Start(":8080"))
}
