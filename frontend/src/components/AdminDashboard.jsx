import React, { useState, useEffect, useCallback } from 'react';
import { api } from '../services/api';

export function AdminDashboard({ token, onError, onSuccess }) {
  const [tasks, setTasks] = useState([]);
  const [loading, setLoading] = useState(false);
  
  // Task form state
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [priority, setPriority] = useState('medium');
  const [assigneeEmail, setAssigneeEmail] = useState('jamesbond@example.com');
  const [selectedTaskIds, setSelectedTaskIds] = useState([]);

  const loadTasks = useCallback(async () => {
    setLoading(true);
    try {
      const data = await api.listTasks(token);
      setTasks(data);
    } catch (err) {
      onError(err.message || 'Failed to fetch tasks');
    } finally {
      setLoading(false);
    }
  }, [token, onError]);

  useEffect(() => {
    loadTasks();
  }, [loadTasks]);

  const handleCreateTask = async (e) => {
    e?.preventDefault();
    if (!title.trim()) return;

    try {
      await api.createTask(token, { title, description, priority, status: 'todo' });
      setTitle('');
      setDescription('');
      onSuccess(`Created task "${title}"`);
      loadTasks();
    } catch (err) {
      onError(err.message || 'Failed to create task');
    }
  };

  const handleQuickCreate5Tasks = async () => {
    setLoading(true);
    try {
      const sampleTasks = [
        { title: 'Infiltrate Enemy Compound', description: 'Gather intelligence on secret facility', priority: 'high' },
        { title: 'Disable Security Grid', description: 'Bypass laser perimeter and mainframe', priority: 'high' },
        { title: 'Exfiltrate VIP Target', description: 'Escort agent safely to helicopter extraction', priority: 'medium' },
        { title: 'Decode Encrypted Transmissions', description: 'Process intercept raw signal logs', priority: 'medium' },
        { title: 'File Mission debrief Report', description: 'Submit paperwork to MI6 headquarters', priority: 'low' },
      ];

      for (const t of sampleTasks) {
        await api.createTask(token, { ...t, status: 'todo' });
      }

      onSuccess('Successfully created 5 tasks for Admin!');
      loadTasks();
    } catch (err) {
      onError(err.message || 'Failed to create sample tasks');
    } finally {
      setLoading(false);
    }
  };

  const handleToggleSelectTask = (id) => {
    setSelectedTaskIds((prev) =>
      prev.includes(id) ? prev.filter((tId) => tId !== id) : [...prev, id]
    );
  };

  const handleSelectFirst3 = () => {
    const first3 = tasks.slice(0, 3).map((t) => t.id);
    setSelectedTaskIds(first3);
  };

  const handleAssignTasks = async () => {
    if (selectedTaskIds.length === 0) {
      onError('Please select at least one task to assign');
      return;
    }

    try {
      const res = await api.assignTasks(token, selectedTaskIds, assigneeEmail);
      onSuccess(`Assigned ${res.assigned_count} tasks to ${assigneeEmail}! Task cache invalidated.`);
      setSelectedTaskIds([]);
      loadTasks();
    } catch (err) {
      onError(err.message || 'Failed to assign tasks');
    }
  };

  return (
    <div className="grid-2">
      {/* Left Column: Create Tasks */}
      <div>
        <div className="card">
          <div className="card-title">
            <span>➕ Create New Task</span>
            <span className="badge badge-admin">Admin Only</span>
          </div>

          <form onSubmit={handleCreateTask}>
            <div className="form-group">
              <label>Task Title</label>
              <input
                type="text"
                className="form-control"
                placeholder="e.g. Inspect Security Protocols"
                value={title}
                onChange={(e) => setTitle(e.target.value)}
                required
              />
            </div>

            <div className="form-group">
              <label>Description</label>
              <textarea
                className="form-control"
                rows={3}
                placeholder="Detailed instructions for assigned staff..."
                value={description}
                onChange={(e) => setDescription(e.target.value)}
              />
            </div>

            <div className="form-group">
              <label>Priority</label>
              <select
                className="form-control"
                value={priority}
                onChange={(e) => setPriority(e.target.value)}
              >
                <option value="high">High</option>
                <option value="medium">Medium</option>
                <option value="low">Low</option>
              </select>
            </div>

            <div style={{ display: 'flex', gap: '10px' }}>
              <button type="submit" className="btn btn-primary" style={{ flex: 1 }}>
                Create Task
              </button>
              <button
                type="button"
                className="btn btn-secondary"
                onClick={handleQuickCreate5Tasks}
                title="Automatically creates 5 sample tasks"
              >
                ⚡ Quick Create 5
              </button>
            </div>
          </form>
        </div>
      </div>

      {/* Right Column: Manage & Assign Tasks */}
      <div>
        <div className="card">
          <div className="card-title">
            <span>📋 System Tasks ({tasks.length})</span>
            <button className="btn btn-secondary btn-sm" onClick={loadTasks}>
              Refresh
            </button>
          </div>

          {tasks.length > 0 && (
            <div style={{ marginBottom: '16px', background: 'rgba(15, 23, 42, 0.4)', padding: '12px', borderRadius: '8px' }}>
              <div className="form-group" style={{ marginBottom: '10px' }}>
                <label>Assignee Email</label>
                <input
                  type="email"
                  className="form-control"
                  value={assigneeEmail}
                  onChange={(e) => setAssigneeEmail(e.target.value)}
                />
              </div>

              <div style={{ display: 'flex', gap: '8px' }}>
                <button
                  className="btn btn-secondary btn-sm"
                  onClick={handleSelectFirst3}
                >
                  Select First 3
                </button>
                <button
                  className="btn btn-primary btn-sm"
                  style={{ flex: 1 }}
                  onClick={handleAssignTasks}
                  disabled={selectedTaskIds.length === 0}
                >
                  Assign {selectedTaskIds.length} Task(s) to {assigneeEmail}
                </button>
              </div>
            </div>
          )}

          {loading ? (
            <p style={{ color: 'var(--text-secondary)' }}>Loading tasks...</p>
          ) : tasks.length === 0 ? (
            <div style={{ textAlign: 'center', padding: '24px 0', color: 'var(--text-secondary)' }}>
              No tasks created yet. Click <strong>Quick Create 5</strong> above to generate tasks.
            </div>
          ) : (
            <div className="table-container">
              <table>
                <thead>
                  <tr>
                    <th style={{ width: '40px' }}>Select</th>
                    <th>Title</th>
                    <th>Priority</th>
                    <th>Assigned To</th>
                  </tr>
                </thead>
                <tbody>
                  {tasks.map((task) => (
                    <tr key={task.id}>
                      <td>
                        <input
                          type="checkbox"
                          checked={selectedTaskIds.includes(task.id)}
                          onChange={() => handleToggleSelectTask(task.id)}
                        />
                      </td>
                      <td>
                        <div style={{ fontWeight: '600' }}>{task.title}</div>
                        <div style={{ fontSize: '0.75rem', color: 'var(--text-secondary)' }}>
                          {task.description}
                        </div>
                      </td>
                      <td>
                        <span className={`badge badge-priority-${task.priority}`}>
                          {task.priority}
                        </span>
                      </td>
                      <td>
                        {task.assigned_to ? (
                          <span style={{ fontSize: '0.85rem', color: '#34d399' }}>
                            {task.assigned_to}
                          </span>
                        ) : (
                          <span style={{ fontSize: '0.85rem', color: 'var(--text-secondary)' }}>
                            Unassigned
                          </span>
                        )}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
