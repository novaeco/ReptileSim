extends Node
class_name EventBus

signal notify(msg : String, level : int)
signal sim_tick(delta_sim : float)
signal save_requested(slot : int)
signal load_requested(slot : int)

func info(t:String): emit_signal("notify", t, 0)
func warn(t:String): emit_signal("notify", t, 1)
func error(t:String): emit_signal("notify", t, 2)
