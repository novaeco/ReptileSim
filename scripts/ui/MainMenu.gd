extends Control
class_name MainMenu

func _ready():
	$VBoxContainer/SpeciesOption.clear()
	for sid in DataStore.list_species_ids():
		$VBoxContainer/SpeciesOption.add_item(String(sid))
	if $VBoxContainer/SpeciesOption.item_count > 0:
		$VBoxContainer/SpeciesOption.select(0)

func _on_NewGame_pressed():
	var sid := $VBoxContainer/SpeciesOption.get_item_text($VBoxContainer/SpeciesOption.get_selected_id())
	var slot := int($VBoxContainer/SlotSpin.value)
	Settings.data["new_game"] = {"species_id": sid, "slot": slot}
	get_tree().change_scene_to_file("res://scenes/Terrarium.tscn")

func _on_LoadGame_pressed():
	EventBus.emit_signal("load_requested", int($VBoxContainer/SlotSpin.value))
