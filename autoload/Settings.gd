extends Node
class_name Settings

const FILE := "user://settings.json"
var data := {
	"graphics": {"render_scale": 0.75, "msaa": 2, "fullscreen": true}
}

func _ready()->void:
	if FileAccess.file_exists(FILE):
		data = JSON.parse_string(FileAccess.get_file_as_string(FILE)) or data
	_apply()

func _apply():
	DisplayServer.window_set_mode(
		data.graphics.fullscreen ? DisplayServer.WINDOW_MODE_FULLSCREEN : DisplayServer.WINDOW_MODE_WINDOWED
	)
