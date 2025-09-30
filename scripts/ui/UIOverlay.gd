extends CanvasLayer
class_name UIOverlay

func _ready():
	var env = get_tree().current_scene.get_node_or_null("EnvironmentSimulator")
	if env:
		env.connect("env_updated", Callable(self, "_on_env"))
	EventBus.connect("notify", Callable(self, "_on_notify"))

func _on_env(t:float,h:float,u:float):
	$Margin/VBox/HBox/Temp.text = "T: %.1f °C" % t
	$Margin/VBox/HBox/Hum.text  = "H: %.0f %%" % h
	$Margin/VBox/HBox/UVI.text  = "UVI: %.1f" % u

func _on_notify(msg:String, lvl:int):
	$Margin/VBox/Notifications.text = msg

func _on_Feed_pressed(): get_tree().current_scene.get_node("Reptile").feed()
func _on_Drink_pressed(): get_tree().current_scene.get_node("Reptile").drink()
func _on_Save_pressed():
	var terrarium = get_tree().current_scene
	SaveManager.save_slot(terrarium.slot, terrarium.get_state())
func _on_Exit_pressed(): get_tree().change_scene_to_file("res://scenes/MainMenu.tscn")
