extends Node3D
class_name Reptile

@export var display_name:String = "Tribolonotus"
var hunger: float = 0.2
var thirst: float = 0.2
var stress: float = 0.2
var health: float = 0.9
var growth: float = 0.0
var species: Dictionary

func _ready()->void:
	var terrarium := get_parent()
	species = terrarium.species
	EventBus.connect("sim_tick", Callable(self, "_on_sim_tick"))

func _on_sim_tick(delta_sim: float) -> void:
	var thirst_rate := float(species.get("thirst_rate_per_hour",0.6))/3600.0
	var hunger_rate := float(species.get("hunger_rate_per_hour",0.6))/3600.0
	thirst = clampf(thirst + thirst_rate*delta_sim, 0.0, 1.0)
	hunger = clampf(hunger + hunger_rate*delta_sim, 0.0, 1.0)

	var env := get_parent().get_node("EnvironmentSimulator")
	var temp := env.temperature_c
	var hum := env.humidity_pct
	var uvi := env.uvi

	var t_range := _target_temp_range()
	var h_range := species.get("humidity_pct_range",[70,90])
	var u_target := float(species.get("uvb_uvi_target",2.0))

	var t_pen := _penalty(temp, t_range[0], t_range[1])
	var h_pen := _penalty(hum, h_range[0], h_range[1])
	var u_pen := absf(uvi - u_target) * 0.02

	stress = clampf(stress + 0.2*(t_pen+h_pen+u_pen)*(delta_sim/60.0) - 0.05*(delta_sim/60.0), 0.0, 1.0)
	health = clampf(health - (0.3*hunger + 0.3*thirst + 0.5*stress)*(delta_sim/3600.0), 0.0, 1.0)

	if health > 0.6 and hunger < 0.5 and thirst < 0.5 and stress < 0.5:
		growth += float(species.get("growth_rate_per_day", 0.0015)) * (delta_sim/86400.0)
		growth = clampf(growth, 0.0, 1.0)

func _target_temp_range()->Array:
	var hour = Time.time_of_day
	var day := (hour >= 8.0 and hour <= 20.0)
	return species.get(day ? "temp_c_day_range" : "temp_c_night_range", [24.0,28.0])

func _penalty(x:float, a:float, b:float)->float:
	if x < a: return (a-x)*0.02
	if x > b: return (x-b)*0.02
	return 0.0

func drink():
	thirst = maxf(0.0, thirst - 0.5)

func feed():
	hunger = maxf(0.0, hunger - 0.5)

func get_state()->Dictionary:
	return {
		"display_name": display_name,
		"hunger": hunger,
		"thirst": thirst,
		"stress": stress,
		"health": health,
		"growth": growth
	}

func set_state(d:Dictionary)->void:
	if d.has("display_name"): display_name = d["display_name"]
	if d.has("hunger"): hunger = float(d["hunger"])
	if d.has("thirst"): thirst = float(d["thirst"])
	if d.has("stress"): stress = float(d["stress"])
	if d.has("health"): health = float(d["health"])
	if d.has("growth"): growth = float(d["growth"])
