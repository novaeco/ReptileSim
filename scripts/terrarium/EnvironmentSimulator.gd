extends Node
class_name EnvironmentSimulator

@export var preset: Dictionary = {}
var temperature_c: float = 25.0
var humidity_pct: float = 80.0
var uvi: float = 1.5

signal env_updated(temp_c: float, humidity_pct: float, uvi: float)

func _ready()->void:
	EventBus.connect("sim_tick", Callable(self, "_on_sim_tick"))

func configure_from_preset(preset_dict:Dictionary):
	preset = preset_dict

func _on_sim_tick(delta_sim: float) -> void:
	var heater_w := float(preset.get("heater_power_w", 15))
	var heat_gain := (heater_w * 0.02)
	var vent := float(preset.get("ventilation_factor", 0.5))
	var loss := (temperature_c - 22.0) * 0.01 * (0.8 + vent)
	temperature_c = clampf(temperature_c + (heat_gain - loss) * (delta_sim / 60.0), 18.0, 35.0)

	var substrate := float(preset.get("substrate_moisture", 0.6))
	var plant := float(preset.get("plant_density", 0.3))
	var evap_loss := maxf(0.0, (temperature_c - 20.0) * 0.02) * (delta_sim / 60.0)
	var refill := (substrate * 0.01 + plant * 0.005) * (delta_sim / 60.0)
	humidity_pct = clampf(humidity_pct - evap_loss + refill - vent * 0.01 * (delta_sim / 60.0), 30.0, 100.0)

	uvi = clampf(float(preset.get("uv_virtual_intensity", 1.0)) * 2.0, 0.0, 5.0)
	emit_signal("env_updated", temperature_c, humidity_pct, uvi)
