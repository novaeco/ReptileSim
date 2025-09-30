extends Node
class_name Time

@export var time_scale: float = 60.0
var time_of_day: float = 12.0 # heures [0..24)

func _process(delta: float) -> void:
	var delta_sim := delta * time_scale
	time_of_day = fmod(time_of_day + delta_sim / 3600.0, 24.0)
	EventBus.emit_signal("sim_tick", delta_sim)
