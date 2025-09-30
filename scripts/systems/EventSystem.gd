extends Node
class_name EventSystem

signal push_notification(text: String, level: int, sticky: bool)

func advise(msg:String, level:int=0, sticky:bool=false):
	emit_signal("push_notification", msg, level, sticky)
	EventBus.emit_signal("notify", msg, level)
