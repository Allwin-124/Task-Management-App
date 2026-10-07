const API_BASE_URL = 'http://127.0.0.1:3000';

async function request(endpoint, options = {}) {
  const url = `${API_BASE_URL}${endpoint}`;
  const headers = {
    'Content-Type': 'application/json',
    ...(options.token ? { Authorization: `Bearer ${options.token}` } : {}),
    ...options.headers,
  };

  const config = {
    method: options.method || 'GET',
    headers,
    ...(options.body ? { body: JSON.stringify(options.body) } : {}),
  };

  const response = await fetch(url, config);
  const data = await response.json().catch(() => ({}));

  if (!response.ok) {
    const errorMsg = data.error || data.message || `Request failed with status ${response.status}`;
    const error = new Error(errorMsg);
    error.status = response.status;
    error.data = data;
    throw error;
  }

  return data;
}

export const api = {
  // Seed Users
  seedUsers: () => request('/seed/users', { method: 'POST' }),

  // Auth & 2FA
  login: (email, password) =>
    request('/auth/login', { method: 'POST', body: { email, password } }),

  getLatestEmailLog: () => request('/dev/email-logs/latest', { method: 'GET' }),

  verify2FA: (loginChallengeId, code) =>
    request('/auth/verify-2fa', {
      method: 'POST',
      body: { login_challenge_id: loginChallengeId, code },
    }),

  // Tasks
  createTask: (token, taskData) =>
    request('/tasks', { method: 'POST', token, body: taskData }),

  listTasks: (token) => request('/tasks', { method: 'GET', token }),

  assignTasks: (token, taskIds, userEmail) =>
    request('/tasks/assign', {
      method: 'POST',
      token,
      body: { task_ids: taskIds, user_email: userEmail },
    }),

  viewMyTasks: (token) =>
    request('/tasks/view-my-tasks', { method: 'GET', token }),
};
