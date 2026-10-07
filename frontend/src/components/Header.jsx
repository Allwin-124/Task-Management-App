import React from 'react';

export function Header({ user, onLogout, onSeedUsers, isSeeding }) {
  return (
    <header className="app-header">
      <div className="logo-group">
        <div className="logo-icon">TF</div>
        <div>
          <h2 style={{ fontSize: '1.2rem', fontWeight: '700' }}>Task Flow Engine</h2>
          <span style={{ fontSize: '0.75rem', color: 'var(--text-secondary)' }}>
            Rust Backend + 2FA + Role Authorization
          </span>
        </div>
      </div>

      <div className="user-nav">
        <button
          className="btn btn-seed btn-sm"
          onClick={onSeedUsers}
          disabled={isSeeding}
        >
          {isSeeding ? 'Seeding...' : ' Seed Users'}
        </button>

        {user && (
          <>
            <div style={{ textAlign: 'right' }}>
              <div style={{ fontSize: '0.85rem', fontWeight: '600' }}>{user.email}</div>
              <span className={`badge badge-${user.role.toLowerCase()}`}>
                {user.role}
              </span>
            </div>
            <button className="btn btn-secondary btn-sm" onClick={onLogout}>
              Logout
            </button>
          </>
        )}
      </div>
    </header>
  );
}
