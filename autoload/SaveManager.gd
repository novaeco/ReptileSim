extends Node
class_name SaveManager

const SAVE_DIR := "user://saves"
const META_FILE := "user://saves/index.json"

var _index := {
	"slots": [],
	"max_slots": 25
}

func _ready() -> void:
	DirAccess.make_dir_recursive_absolute(SAVE_DIR)
	_load_index()

func _load_index():
	if FileAccess.file_exists(META_FILE):
		_index = JSON.parse_string(FileAccess.get_file_as_string(META_FILE)) or _index
	else:
		_save_index()

func _save_index():
	FileAccess.open(META_FILE, FileAccess.WRITE).store_string(JSON.stringify(_index, "\t"))

func list_slots() -> Array:
	return _index["slots"]

func save_slot(slot:int, data:Dictionary) -> void:
	var path := "%s/slot_%02d.json" % [SAVE_DIR, slot]
	FileAccess.open(path, FileAccess.WRITE).store_string(JSON.stringify(data, "\t"))
	var found := false
	for s in _index["slots"]:
		if s.get("slot",-1) == slot:
			found = true
			s["name"] = data.get("name","Terrarium")
			s["species"] = data.get("species_id","")
	if not found:
		_index["slots"].append({
			"slot": slot,
			"name": data.get("name","Terrarium"),
			"species": data.get("species_id","")
		})
	_save_index()
	EventBus.info("Sauvegardé slot %d" % slot)

func load_slot(slot:int) -> Dictionary:
	var path := "%s/slot_%02d.json" % [SAVE_DIR, slot]
	if not FileAccess.file_exists(path):
		return {}
	return JSON.parse_string(FileAccess.get_file_as_string(path)) or {}
