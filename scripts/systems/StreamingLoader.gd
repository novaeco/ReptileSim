extends Node
class_name StreamingLoader

func load_async(path:String, callback:Callable):
	ResourceLoader.load_threaded_request(path)
	await get_tree().process_frame
	while true:
		var status := ResourceLoader.load_threaded_get_status(path)
		if status == ResourceLoader.THREAD_LOAD_LOADED:
			var res := ResourceLoader.load_threaded_get(path)
			callback.call(res)
			return
		elif status == ResourceLoader.THREAD_LOAD_FAILED:
			push_warning("Load failed: %s" % path)
			callback.call(null)
			return
		await get_tree().create_timer(0.01).timeout
