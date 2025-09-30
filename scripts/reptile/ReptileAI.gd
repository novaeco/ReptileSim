extends Node
class_name ReptileAI

@export var speed: float = 0.5

func _ready()->void:
	EventBus.connect("sim_tick", Callable(self, "_on_tick"))
	randomize()

func _on_tick(delta_sim:float)->void:
	var reptile:Node3D = get_parent() as Node3D
	if not reptile: return
	var dir := Vector3(randf()-0.5, 0, randf()-0.5).normalized()
	reptile.translate(dir * speed * (delta_sim/60.0))
