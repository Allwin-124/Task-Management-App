import React, { useState, useEffect, useCallback } from 'react';
import { api } from '../services/api';

export function StaffDashboard({ user, token, onError, onSuccess }) {
  const [tasks, setTasks] = useState([]);
  const [cacheHit, setCacheHit] = useState(null);
  const [summaryCount, setSummaryCount] = useState(0);
  const [loading, setLoading] = useState(false);
  const [forbiddenMessage, setForbiddenMessage] = useState(null);

  const fetchMyTasks = useCallback(async () => {
    setLoading(true);
    setForbiddenMessage(null);
    try {
      const data = await api.viewMyTasks(token);
      setTasks(data.tasks || []);
      setCacheHit(data.cache?.hit);
      setSummaryCount(data.summary?.total_assigned_tasks || 0);
      
      const hitStatus = data.cache?.hit ? 'CACHE HIT (true)' : 'CACHE MISS (false - fetched from DB)';
      onSuccess(`Fetched assigned tasks. Status: ${hitStatus}`);
    } catch (err) {
      onError(err.message || 'Failed to fetch assigned tasks');
    } finally {
      setLoading(false);
    }
  }, [token, onError, onSuccess]);

  useEffect(() => {
    fetchMyTasks();
  }, [fetchMyTasks]);

  const handleTestCreateTaskForbidden = async () => {
    setForbiddenMessage(null);
    try {
      await api.createTask(token, {
        title: 'Forbidden Task Attempt',
        description: 'James Bond should not be able to create tasks',
        priority: 'high',
      });
      onError('UNEXPECTED: Task creation succeeded when it should have been blocked!');
    } catch (err) {
      if (err.status === 403 || err.message.toLowerCase().includes('forbidden')) {
        const msg = `⛔ 403 Forbidden: Staff user (${user.email}) is blocked from creating tasks!`;
        setForbiddenMessage(msg);
        onError(msg);
      } else {
        onError(`Error: ${err.message}`);
      }
    }
  };

  return (
    <div style={{ maxWidth: '800px', margin: '0 auto' }}>
      {/* Caching & Role Header Card */}
      <div className="card">
        <div className="card-title">
          <div>
            <span>🎯 Staff Dashboard — {user.email}</span>
            <div style={{ fontSize: '0.8rem', fontWeight: 'normal', color: 'var(--text-secondary)', marginTop: '4px' }}>
              Assigned Tasks View (Database + In-Memory Cache)
            </div>
          </div>
          {cacheHit !== null && (
            <span className={`badge ${cacheHit ? 'badge-cache-hit' : 'badge-cache-miss'}`}>
              {cacheHit ? '⚡ CACHE HIT (hit: true)' : '🔄 CACHE MISS (hit: false)'}
            </span>
          )}
        </div>

        <div style={{ display: 'flex', gap: '12px', flexWrap: 'wrap', marginTop: '16px' }}>
          <button className="btn btn-primary" onClick={fetchMyTasks} disabled={loading}>
            {loading ? 'Fetching...' : '🔄 Fetch Tasks Again (Test Cache Hit)'}
          </button>
          <button className="btn btn-danger" onClick={handleTestCreateTaskForbidden}>
            🛡️ Attempt Create Task (Test 403 Forbidden)
          </button>
        </div>

        {forbiddenMessage && (
          <div className="alert alert-error" style={{ marginTop: '16px' }}>
            <span>{forbiddenMessage}</span>
          </div>
        )}
      </div>

      {/* Assigned Tasks List */}
      <div className="card">
        <div className="card-title">
          <span>Assigned Mission Tasks ({summaryCount})</span>
        </div>

        {loading ? (
          <p style={{ color: 'var(--text-secondary)' }}>Loading assigned tasks...</p>
        ) : tasks.length === 0 ? (
          <div style={{ textAlign: 'center', padding: '32px 0', color: 'var(--text-secondary)' }}>
            No tasks assigned to <strong>{user.email}</strong> yet.<br />
            <span style={{ fontSize: '0.85rem' }}>
              (Log in as Admin to assign tasks to James Bond, then switch back here!)
            </span>
          </div>
        ) : (
          <div className="table-container">
            <table>
              <thead>
                <tr>
                  <th>Task Title & Description</th>
                  <th>Priority</th>
                  <th>Status</th>
                  <th>Assigned To</th>
                </tr>
              </thead>
              <tbody>
                {tasks.map((task) => (
                  <tr key={task.id}>
                    <td>
                      <div style={{ fontWeight: '600' }}>{task.title}</div>
                      <div style={{ fontSize: '0.8rem', color: 'var(--text-secondary)' }}>
                        {task.description}
                      </div>
                    </td>
                    <td>
                      <span className={`badge badge-priority-${task.priority}`}>
                        {task.priority}
                      </span>
                    </td>
                    <td>
                      <span className="badge badge-staff">{task.status}</span>
                    </td>
                    <td>
                      <span style={{ color: '#34d399', fontSize: '0.85rem' }}>
                        {task.assigned_to}
                      </span>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>
    </div>
  );
}
