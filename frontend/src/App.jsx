import React, { useState } from 'react';
import { Header } from './components/Header';
import { AuthScreen } from './components/AuthScreen';
import { AdminDashboard } from './components/AdminDashboard';
import { StaffDashboard } from './components/StaffDashboard';
import { api } from './services/api';

export default function App() {
  const [user, setUser] = useState(null); // { email, role, token }
  const [activeTab, setActiveTab] = useState('main'); // 'main' or 'staff'
  const [errorMsg, setErrorMsg] = useState(null);
  const [successMsg, setSuccessMsg] = useState(null);
  const [isSeeding, setIsSeeding] = useState(false);

  const showError = (msg) => {
    setErrorMsg(msg);
    setTimeout(() => setErrorMsg(null), 6000);
  };

  const showSuccess = (msg) => {
    setSuccessMsg(msg);
    setTimeout(() => setSuccessMsg(null), 6000);
  };

  const handleSeedUsers = async () => {
    setIsSeeding(true);
    try {
      const res = await api.seedUsers();
      showSuccess(`Users seeded successfully! (Admin: admin@example.com, James Bond: jamesbond@example.com)`);
    } catch (err) {
      showError(err.message || 'Failed to seed users');
    } finally {
      setIsSeeding(false);
    }
  };

  const handleLogout = () => {
    setUser(null);
    showSuccess('Logged out successfully.');
  };

  return (
    <div className="app-container">
      <Header
        user={user}
        onLogout={handleLogout}
        onSeedUsers={handleSeedUsers}
        isSeeding={isSeeding}
      />

      {/* Notifications */}
      {errorMsg && (
        <div className="alert alert-error">
          <span>{errorMsg}</span>
          <button
            onClick={() => setErrorMsg(null)}
            style={{ background: 'none', border: 'none', color: '#fca5a5', cursor: 'pointer' }}
          >
            ✕
          </button>
        </div>
      )}

      {successMsg && (
        <div className="alert alert-success">
          <span>{successMsg}</span>
          <button
            onClick={() => setSuccessMsg(null)}
            style={{ background: 'none', border: 'none', color: '#6ee7b7', cursor: 'pointer' }}
          >
            ✕
          </button>
        </div>
      )}

      {/* Auth Screen or Dashboard */}
      {!user ? (
        <AuthScreen
          onAuthSuccess={(authData) => {
            setUser(authData);
            setActiveTab(authData.role === 'admin' ? 'admin' : 'staff');
          }}
          onError={showError}
          onSuccess={showSuccess}
        />
      ) : (
        <div>
          {/* Admin view switch tabs */}
          {user.role === 'admin' && (
            <div style={{ display: 'flex', gap: '10px', marginBottom: '20px' }}>
              <button
                className={`btn ${activeTab === 'admin' ? 'btn-primary' : 'btn-secondary'}`}
                onClick={() => setActiveTab('admin')}
              >
                🛠️ Admin Task Control Panel
              </button>
              <button
                className={`btn ${activeTab === 'staff' ? 'btn-primary' : 'btn-secondary'}`}
                onClick={() => setActiveTab('staff')}
              >
                👤 View My Assigned Tasks
              </button>
            </div>
          )}

          {user.role === 'admin' && activeTab === 'admin' ? (
            <AdminDashboard
              token={user.token}
              onError={showError}
              onSuccess={showSuccess}
            />
          ) : (
            <StaffDashboard
              user={user}
              token={user.token}
              onError={showError}
              onSuccess={showSuccess}
            />
          )}
        </div>
      )}
    </div>
  );
}