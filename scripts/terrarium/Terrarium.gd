extends Node3D
class_name Terrarium

@export var species_id: StringName = &"tribolonotus_gracilis"
@export var slot:int = 0

var species:Dictionary
var env_sim:EnvironmentSimulator

func _ready()->void:
	# Permettre au menu de passer le choix via Settings.data["new_game"]
	if Settings.data.has("new_game"):
		var ng = Settings.data["new_game"]
		species_id = StringName(ng.get("species_id", species_id))
		slot = int(ng.get("slot", slot))
		Settings.data.erase("new_game")

	species = DataStore.get_species(species_id)
	env_sim = $EnvironmentSimulator
	env_sim.configure_from_preset(_load_preset())
	EventBus.info("Terrarium prêt: %s" % species.get("name","Unknown"))

func _load_preset()->Dictionary:
	var raw = FileAccess.get_file_as_string("res://data/terrarium_presets/basic_60x45x45.json")
	return JSON.parse_string(raw) or {}

func get_state()->Dictionary:
	return {
		"name":"Terrarium %02d" % slot,
		"species_id":String(species_id),
		"env": {
			"temperature_c": env_sim.temperature_c,
			"humidity_pct": env_sim.humidity_pct,
			"uvi": env_sim.uvi
		},
		"reptile": $Reptile.get_state()
	}

func set_state(d:Dictionary)->void:
	if d.has("reptile"):
		$Reptile.set_state(d["reptile"])
