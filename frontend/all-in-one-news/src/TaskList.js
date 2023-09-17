// src/TaskList.js
import React, { useState, useEffect } from 'react';
import axios from 'axios';

function TaskList() {
  const [tasks, setTasks] = useState([]);

  useEffect(() => {
    axios.get('http://127.0.0.1:8000/latest')
      .then(response => {
        setTasks(response.data);
         console.log(response.data); // 在这里打印更新后的数据
      })
      .catch(error => {
        console.error('获取数据时出错:', error);
      });
  }, []);

  return (
    <div>
      <h1>任务列表</h1>
      <ul>
        {
          tasks.map(task => (

          <li key={task.id}>{task.title}</li>
        ))}
      </ul>
    </div>
  );
}

export default TaskList;
