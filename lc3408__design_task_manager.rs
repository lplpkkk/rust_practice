use std::collectins::{BinaryHeap,HashMap};
use std::cmp::Ordering;

struct Task{
	priority:i32,
	task_id:i32,
}

struct TaskManager{
	task_info:HashMap<i32,(i32,i32)>,
	heap:BinaryHeap<Task>,
}

impl TaskManager{
	fn new(tasks:Vec<Vec<i32>>->Self{
		let mut task_info=HashMap::new();
		let mut heap=BinaryHeap::new();
		
		for task in tasks{
			let user_id=task[0];
			..
			..

			task_info.insert(task_id,(piority,user_id));
			heap.push(Task{priority,task_id});
		}

		fn add(&mut self, task_id:i32, priority:i32){
			self.task_info.insert(task_id,
			self.heap.push()
		}

		fn edit(&mut self, task_id:i32, new_p:i32){
			if let Some(info)=self.task_info.get_mut(&task_id){
				info.0=new_priority;
				self.heap.push(Task{new_prio
			}
		}
	}
}






while let Some(task) = self.heap.pop(){
	if let Some(&(priorit,user_id)) = self.task_info.get(&task.task_id){
		if priorty=task.prioirty{
			return
		}
	}
}


