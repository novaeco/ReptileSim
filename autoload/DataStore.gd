extends Node
class_name DataStore

var species_db : Dictionary = {}

func _ready() -> void:
	_load_species()

func _load_species():
	var dir := DirAccess.open("res://data/species")
	if dir:
		dir.list_dir_begin()
		while true:
			var f = dir.get_next()
			if f == "": break
			if f.ends_with(".json"):
				var raw = FileAccess.get_file_as_string("res://data/species/%s" % f)
				var obj = JSON.parse_string(raw)
				if typeof(obj) == TYPE_DICTIONARY and obj.has("id"):
					species_db[obj.id] = obj

func get_species(id:StringName) -> Dictionary:
	return species_db.get(id, {})

func list_species_ids() -> Array:
	return species_db.keys()
